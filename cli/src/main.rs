use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{fs, os::unix::process::CommandExt};

use clap::{Args, Parser, Subcommand};
use serde::Deserialize;

macro_rules! docker {
    () => {
        Command::new("docker")
    };
    ($($args:expr),+ $(,)?) => {{
        let mut docker = docker!();
        docker.args([$($args),+]);
        docker
    }};
}

#[derive(Deserialize)]
struct Preset {
    /// Name of the preset.
    name: String,

    /// Tag of the image to create the container from.
    image: String,

    /// Name of the container to create/use.
    container: String,

    /// Path to the home directory.
    homedir: PathBuf,

    /// Username within the container.
    user: String,

    /// The default shell to attach to in the container.
    #[serde(default = "default_shell")]
    shell: String,

    /// Host-container pairs of volumes to mount.
    #[serde(default)]
    volumes: Vec<(String, String)>,

    /// Host-container pairs of ports to connect.
    #[serde(default)]
    ports: Vec<(u16, u16)>,

    /// Extra `docker run` arguments. To be used when creating a new container.
    #[serde(default)]
    run_args: Vec<String>,
}

fn default_shell() -> String {
    "/bin/sh".to_string()
}

impl Preset {
    /// Builds the corresponding `Command` for `docker run`.
    pub fn build_docker_run(&self) -> Command {
        let mut cmd = docker!("run");
        cmd.args(["--name", &self.container]);
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
        cmd.args([&self.container, &self.shell]);
        cmd
    }
}

#[derive(Deserialize)]
struct Config {
    presets: Vec<Preset>,
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

    pub fn get_preset<S: AsRef<str>>(&self, name: S) -> Option<&Preset> {
        let name = name.as_ref();
        self.presets.iter().find(|v| v.name == name)
    }

    pub fn quick_preset<P, S>(config_path: P, preset_name: S) -> Preset
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let cfg = Self::from_path(config_path);
        let x = preset_name.as_ref();
        let Some(preset) = cfg.presets.into_iter().find(|v| v.name == x) else {
            println!("Preset not found: {x}");
            exit(1);
        };
        preset
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Args)]
struct RunArgs {
    #[arg(short = 'c', long = "config", default_value = "aidconfig.yml")]
    config_path: String,

    preset: String,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new container, while also running it.
    ///
    /// EQ: (docker run ...)
    Run(RunArgs),

    /// Attaches to a running container.
    ///
    /// EQ: (docker exec -it ...)
    Attach(RunArgs),

    /// Start a container that already exists.
    ///
    /// Note that this will bring along
    /// all the configs that the container was created with.
    ///
    /// EQ: (docker start ...)
    Start { container: String },

    /// EQ: (docker stop ...)
    Stop { container: String },

    /// EQ: (docker kill ...)
    Kill { container: String },

    /// Removes a container.
    ///
    /// EQ: (docker rm ...)
    Rm { container: String },

    /// Removes an image.
    ///
    /// EQ: (docker rmi ...)
    Rmi { image: String },

    /// Completely obliterates a docker container.
    ///
    /// Runs `docker stop` then `docker remove` on a container.
    Annihilate { container: String },
}

fn main() {
    let cli = Cli::parse();
    // let cfg = Config::from_path(&cli.config_path).unwrap();

    match cli.command {
        Cmd::Run(RunArgs { config_path, preset }) => {
            let preset = Config::quick_preset(config_path, preset);
            preset.build_docker_run().exec();
        }
        Cmd::Attach(RunArgs { config_path, preset }) => {
            let preset = Config::quick_preset(config_path, preset);
            preset.build_docker_exec().exec();
            // docker!("exec", "--user", &user, "--workdir", &workdir, "-it",);
            // docker exec --user appliedai --workdir /home/appliedai -it $(CONTAINER) zsh
        }
        Cmd::Start { container } => {
            docker!("start", &container).exec();
        }
        Cmd::Stop { container } => {
            docker!("stop", &container).exec();
        }
        Cmd::Kill { container } => {
            docker!("kill", &container).exec();
        }
        Cmd::Rm { container } => {
            docker!("rm", &container).exec();
        }
        Cmd::Rmi { image } => {
            docker!("rmi", &image).exec();
        }
        Cmd::Annihilate { container } => {
            let container = container.as_str();
            // Stop the container if it exists
            if let Ok(mut child) = docker!("stop", container).spawn() {
                let _ = child.wait();
            }
            // Remove the container
            if let Ok(mut child) = docker!("rm", container).spawn() {
                let _ = child.wait();
            }
        }
    }
}
