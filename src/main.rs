use ej_config::ej_board_config::EjBoardConfigApi;
use ej_dispatcher_sdk::{EjRunResult, prelude::*};
use std::{collections::HashMap, path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "ejkmer-dispatcher")]
#[command(about = "EJ Kmer Dispatcher - Job dispatcher and result handler for the Kmer project")]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DispatchRun {
        /// Path to the EJD's unix socket
        #[arg(short, long)]
        socket: PathBuf,
        /// The maximum job duration in seconds
        #[arg(long)]
        seconds: u64,
        /// Git commit hash
        #[arg(long)]
        commit_hash: String,
        /// Git remote url
        #[arg(long)]
        remote_url: String,
    },
}

struct ConfigResult {
    config: EjBoardConfigApi,
    data: HashMap<String, usize>,
}

fn parse_results(job_result: &EjRunResult) -> Vec<ConfigResult> {
    let mut parsed_results: Vec<ConfigResult> = Vec::new();
    for (board_config, result) in job_result.results.iter() {
        let mut occurrences_map: HashMap<String, usize> = HashMap::new();
        let mut found_start_of_results = false;
        for line in result.lines() {
            if line.contains("Results:") {
                found_start_of_results = true;
                continue;
            }
            if !found_start_of_results {
                continue;
            }
            if line.contains(':') {
                let splitted: Vec<&str> = line.split(": ").collect();
                assert_eq!(splitted.len(), 2);
                let sequence = splitted[0];
                let n_occurences = splitted[1]
                    .parse()
                    .expect("Expected number on right side of ':'");
                occurrences_map.insert(sequence.to_string(), n_occurences);
            }
        }
        parsed_results.push(ConfigResult {
            config: board_config.clone(),
            data: occurrences_map,
        });
    }
    parsed_results
}

fn check_results(parsed_results: &Vec<ConfigResult>) {
    for i in 0..parsed_results.len() {
        for j in (i + 1)..parsed_results.len() {
            let config_i = &parsed_results[i].config;
            let config_j = &parsed_results[j].config;
            let data_i = &parsed_results[i].data;
            let data_j = &parsed_results[j].data;

            assert_eq!(
                data_i.len(),
                data_j.len(),
                "Different number of sequences for {} and {} {} vs {}",
                config_i.name,
                config_j.name,
                data_i.len(),
                data_j.len(),
            );

            for (sequence, expected) in parsed_results[i].data.iter() {
                let actual = data_j.get(sequence);
                assert!(
                    actual.is_some(),
                    "Couldn't find {} in {}",
                    sequence,
                    config_j.name
                );

                let actual = actual.unwrap();

                assert_eq!(
                    expected, actual,
                    "Expected {} occurrences for {}. Got {} ",
                    expected, sequence, actual
                );
            }
        }
    }
}

async fn do_run(
    socket: PathBuf,
    seconds: u64,
    commit_hash: String,
    remote_url: String,
) -> Result<()> {
    let job_result = ej_dispatcher_sdk::dispatch_run(
        &socket,
        commit_hash,
        remote_url,
        None,
        Duration::from_secs(seconds),
    )
    .await?;
    println!("{}", job_result);

    if !job_result.success {
        return Err(Error::RunError);
    }
    let parsed_results = parse_results(&job_result);
    check_results(&parsed_results);
    println!("Results OK!");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::DispatchRun {
            socket,
            seconds,
            commit_hash,
            remote_url,
        } => do_run(socket, seconds, commit_hash, remote_url).await,
    }
}
