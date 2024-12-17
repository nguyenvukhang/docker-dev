pub(crate) use crate::{config::*, docker_ps::*};

pub(crate) use clap::{Args, Parser, Subcommand};
pub(crate) use serde::Deserialize;

pub(crate) use std::fs;
pub(crate) use std::os::unix::process::CommandExt;
pub(crate) use std::path::PathBuf;
pub(crate) use std::process::{exit, Command};

pub(crate) const CONFIG_FILENAME: &str = "aidconfig.yml";

macro_rules! docker {
    () => {
        std::process::Command::new("docker")
    };
    (exec, $($args:expr),+ $(,)?) => {{
        docker!($($args),+).exec();
    }};
    ($($args:expr),+ $(,)?) => {{
        let mut docker = docker!();
        docker.args([$($args),+]);
        docker
    }};
}
