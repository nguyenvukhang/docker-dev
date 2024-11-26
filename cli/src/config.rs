use crate::*;

#[derive(Deserialize)]
pub struct Preset {
    /// Name of the preset. Also the name of the target container.
    pub name: String,

    /// Tag of the image to create the container from.
    pub image: String,

    /// Path to the home directory.
    pub homedir: PathBuf,

    /// Username within the container.
    pub user: String,

    /// The default shell to attach to in the container.
    #[serde(default = "default_shell")]
    pub shell: String,

    /// Host-container pairs of volumes to mount.
    #[serde(default)]
    pub volumes: Vec<(String, String)>,

    /// Host-container pairs of ports to connect.
    #[serde(default)]
    pub ports: Vec<(u16, u16)>,

    /// Extra `docker run` arguments. To be used when creating a new container.
    #[serde(default)]
    pub run_args: Vec<String>,
}

fn default_shell() -> String {
    "/bin/sh".to_string()
}

impl Preset {
    /// Builds the corresponding `Command` for `docker run`.
    pub fn build_docker_run(&self) -> Command {
        let mut cmd = docker!("run");
        cmd.args(["--name", &self.name]);
        for (host, cont) in &self.volumes {
            cmd.args(["-v", &format!("{host}:{cont}")]);
        }
        for (host, cont) in &self.ports {
            cmd.args(["-p", &format!("{host}:{cont}")]);
        }
        cmd.args(["--detach", "--tty"]);
        cmd.args(&self.run_args);
        cmd.arg(&self.image);
        cmd
    }

    /// Builds the corresponding `Command` for `docker exec`.
    pub fn build_docker_exec(&self) -> Command {
        let mut cmd = docker!("exec", "-it");
        cmd.args(["--user", &self.user]);
        cmd.arg("--workdir");
        cmd.arg(&self.homedir);
        cmd.args([&self.name, &self.shell]);
        cmd
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub presets: Vec<Preset>,
}

impl Config {
    /// Returns Self, but panics with helpful error messages if it fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let txt = match fs::read_to_string(path.as_ref()) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", path.as_ref().display());
                }
                _ => panic!("{e:?}"),
            },
        };
        serde_yaml::from_str::<Config>(&txt).unwrap()
    }

    pub fn contains_preset<S: AsRef<str>>(&self, preset_name: S) -> bool {
        let x = preset_name.as_ref();
        self.presets.iter().any(|v| v.name == x)
    }

    pub fn quick_preset<P, S, F>(
        config_path: P,
        preset_name: Option<S>,
        help: Option<F>,
    ) -> Preset
    where
        P: AsRef<Path>,
        S: AsRef<str>,
        F: FnOnce() -> (),
    {
        let cfg = Self::from_path(config_path);
        let err = cfg.presets.iter().fold(
            "\
---------------------------------
Pick one of these presets to run:"
                .to_string(),
            |a, v| a + "\n  * " + &v.name,
        );
        let preset = preset_name.and_then(|x| {
            cfg.presets.into_iter().find(|v| v.name == x.as_ref())
        });
        match preset {
            Some(v) => return v,
            None => {
                help.map(|v| v());
                println!("{err}");
                exit(1);
            }
        }
    }
}
