mod cli;
mod course;
mod login;
mod requester;

use std::{
    error::Error,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use clap::Parser;
use cli::Cli;
use requester::{Config, Requester};
use serde_json;

const DEFAULT_CONFIG_PATH: &str = "config.json";

async fn run_requester(config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    let requester = Requester::new(config);

    requester.run().await?;

    Ok(())
}

fn write_config_to_file(file: &File, config: &Config) -> Result<(), Box<dyn Error>> {
    serde_json::to_writer(file, config)?;
    Ok(())
}

fn write_config(output_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    match File::create_new(output_path) {
        Ok(file) => {
            write_config_to_file(&file, config)?;
            println!("Ayarlar {} dosyasına yazıldı.", output_path.display());
            Ok(())
        }
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => {
                println!("Dosya {} zaten var. Siliniyor...", output_path.display());
                std::fs::remove_file(output_path)?;
                write_config(output_path, config)
            }
            _ => {
                eprintln!("{}", e);
                Err(Box::new(e))
            }
        },
    }
}

#[allow(dead_code)]
fn read_config_file(config_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(file)?;
    println!("Read config: {:?}", config);
    println!("Time is: {}", config.time);
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Command::MakeConfig(make_config_args) => {
            let output_path = make_config_args
                .output_path
                .clone()
                .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH));
            match write_config(&output_path, &make_config_args.clone().into()) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
        cli::Command::Run(run_args) => {
            match run_requester(
                &run_args
                    .config_path
                    .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH)),
            )
            .await
            {
                Ok(_) => println!(
                    "Program başarıyla sonlandı. Umarım derslerini alabilmişsindir! <3 :pray:"
                ),
                Err(e) => eprintln!("Özür dilerim ama program hatalı sonlandı: {}", e),
            }
        }
    }
}
