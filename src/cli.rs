use std::path::PathBuf;

use clap::{arg, command, Args, Parser, Subcommand};

use crate::requester::Config;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Create a config file")]
    MakeConfig(MakeConfigArgs),

    #[command(about = "Run the bot")]
    Run(RunArgs), // TODO: add override args
}

#[derive(Args, Debug, Clone)]
pub struct MakeConfigArgs {
    #[arg(short = 'o', long = "output", help = "Output path")]
    pub output_path: PathBuf,

    #[arg(short = 'u', long = "username", help = "ITÜ Kullanıcı Adı")]
    pub username: String,

    #[arg(short = 'p', long = "password", help = "ITÜ Kullanıcı Şifresi")]
    pub password: String,
    // TODO: Add time
}

impl Into<Config> for MakeConfigArgs {
    fn into(self) -> Config {
        Config::new(self.username, self.password)
    }
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(short = 'c', long = "config", help = "config.json dosyasının konumu")]
    pub config_path: Option<PathBuf>,
}
