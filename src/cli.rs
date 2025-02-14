use std::{path::PathBuf, string::ParseError, time::SystemTime};

use clap::{arg, command, Args, Parser, Subcommand};
use humantime::parse_rfc3339_weak;

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

fn parse_time(arg: &str) -> Result<std::time::Duration, ParseError> {
    // TODO: Error handling
    let time = parse_rfc3339_weak(arg).unwrap();

    Ok(time.duration_since(SystemTime::UNIX_EPOCH).unwrap())
}

#[derive(Args, Debug, Clone)]
pub struct MakeConfigArgs {
    #[arg(short = 'o', long = "output", help = "Output path")]
    pub output_path: PathBuf,

    #[arg(short = 'u', long = "username", help = "ITÜ Kullanıcı Adı")]
    pub username: String,

    #[arg(short = 'p', long = "password", help = "ITÜ Kullanıcı Şifresi")]
    pub password: String,

    #[arg(short = 't', long = "time", help = "Ders Seçim Zamanı", value_parser = parse_time)]
    pub time: std::time::Duration,

    #[arg(long = "crn", help = "Eklenecek CRN'ler", value_delimiter = ',')]
    pub crn_list: Vec<String>,

    #[arg(long = "scrn", help = "Çıkartılacak CRN'ler", value_delimiter = ',')]
    pub scrn_list: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(short = 'c', long = "config", help = "config.json dosyasının konumu")]
    pub config_path: Option<PathBuf>,
}
