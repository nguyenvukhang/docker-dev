#[macro_use]
mod prelude;

mod config;
mod docker_ps;

use std::process::Output;

use clap::CommandFactory;
use prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Args)]
struct RunArgs {
    preset: Option<String>,
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
    #[clap(visible_alias = "a")]
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
    #[clap(visible_alias = "d")]
    Delete { container: String },

    /// Lists all containers.
    #[clap(visible_alias = "ls")]
    List {
        #[arg(short, long)]
        all: bool,
    },
}

fn show_help(subcm: &str) {
    match Cli::command().get_subcommands_mut().find(|v| v.get_name() == subcm) {
        Some(v) => v.print_long_help().unwrap_or(()),
        None => println!("Invalid subcommand: {subcm}"),
    }
}

fn display_containers(cfg: &Config, output: Output) {
    for ln in std::str::from_utf8(&output.stdout).unwrap().lines() {
        let Ok(dp) = serde_json::from_str::<DockerPsLine>(ln) else { continue };
        let managed = cfg.contains_preset(&dp.name);
        let s_id = dp.short_id(10);
        match managed {
            true => print!("\x1b[32m*>\x1b[m "),
            false => print!("*  "),
        }
        print!("{s_id} + ");
        match managed {
            true => print!("\x1b[32m{: <21}\x1b[m", dp.name),
            false => print!("{: <21}", dp.name),
        }
        println!(
            "{status: <16}{created_at}",
            status = dp.status,
            created_at = dp.created_at,
        );
        println!("   image      | {}", dp.image);

        let ports = dp.ports();
        let mut ports = ports.iter();
        ports.next().map(|v| println!("   ports      | {v}"));
        ports.for_each(|v| println!("              | {v}"));
    }
}

fn main() {
    let cli = Cli::parse();
    let cfg = Config::resolve();
    match cli.command {
        Cmd::Run(RunArgs { preset }) => {
            let preset = cfg.get_preset(preset, Some(|| show_help("run")));
            preset.build_docker_run().exec();
        }
        Cmd::Attach(RunArgs { preset }) => {
            let preset = cfg.get_preset(preset, Some(|| show_help("attach")));
            preset.build_docker_exec().exec();
        }
        Cmd::Start { container } => docker!(exec, "start", &container),
        Cmd::Stop { container } => docker!(exec, "stop", &container),
        Cmd::Kill { container } => docker!(exec, "kill", &container),
        Cmd::Rm { container } => docker!(exec, "rm", &container),
        Cmd::Rmi { image } => docker!(exec, "rmi", &image),
        Cmd::Delete { container } => {
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
        Cmd::List { all } => {
            let mut cmd = docker!("ps", "--no-trunc", "--format", "json");
            if all {
                cmd.arg("--all");
            }
            display_containers(&cfg, cmd.output().expect("`docker ps` failed."))
        }
    }
}
