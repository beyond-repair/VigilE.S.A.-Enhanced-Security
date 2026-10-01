//! VigilE.S.A. — Claim-0 CLI entrypoint.
//!
//! Loads `config/security.toml` (or built-in defaults) and runs a local mock
//! security loop. Use `--demo` / `--once` for CI and clean-clone smoke tests.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;
use vigil_esa::{SecurityConfig, SecurityLoop};

#[derive(Parser, Debug)]
#[command(
    name = "vigil-esa",
    about = "VigilE.S.A. Enhanced Security — Claim-0 runnable sketch (mock demos only)",
    long_about = "Local in-memory security-architecture demo. Does NOT provide real Zero Trust,\n\
eBPF filtering, HSM cryptography, SGX enclaves, or cloud protection."
)]
struct Cli {
    /// Path to security.toml (default: config/security.toml)
    #[arg(short, long, global = true, default_value = "config/security.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the mock security loop for configured heartbeats then exit (CI-friendly)
    Demo,
    /// Single heartbeat then exit
    Once,
    /// Print loaded config summary and exit
    Status,
    /// Continuous mock loop until Ctrl-C (not used in CI)
    Run,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = match SecurityConfig::load(&cli.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "warning: could not load {}: {e}; using built-in demo defaults",
                cli.config.display()
            );
            SecurityConfig::demo_default()
        }
    };

    let cmd = cli.command.unwrap_or(Commands::Demo);
    let loop_ = SecurityLoop::from_config(config.clone());

    match cmd {
        Commands::Status => {
            println!("VigilE.S.A. Claim-0 status");
            println!("  label:           {}", config.demo.label);
            println!("  hsm.provider:    {}", config.hsm.provider);
            println!("  network.iface:   {}", config.network.interface);
            println!("  cloud.region:    {}", config.cloud.aws_region);
            println!("  cloud.project:   {}", config.cloud.gcp_project);
            println!("  note:            mock demos only — no real eBPF/HSM/cloud");
            ExitCode::SUCCESS
        }
        Commands::Once => {
            let beat = loop_.tick_once(1).await;
            print_heartbeat(&config.demo.label, &beat);
            println!("done (once)");
            ExitCode::SUCCESS
        }
        Commands::Demo => {
            let max = config.demo.max_heartbeats.max(1);
            let delay = Duration::from_secs(config.demo.heartbeat_secs.max(1));
            println!(
                "VigilE.S.A. demo starting label={} heartbeats={}",
                config.demo.label, max
            );
            for i in 1..=max {
                let beat = loop_.tick_once(i).await;
                print_heartbeat(&config.demo.label, &beat);
                if i < max {
                    tokio::time::sleep(delay).await;
                }
            }
            println!("demo complete ({max} heartbeats)");
            ExitCode::SUCCESS
        }
        Commands::Run => {
            println!(
                "VigilE.S.A. continuous mock loop label={} (Ctrl-C to stop)",
                config.demo.label
            );
            let mut tick = 0u64;
            loop {
                tick += 1;
                let beat = loop_.tick_once(tick).await;
                print_heartbeat(&config.demo.label, &beat);
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(
                        config.demo.heartbeat_secs.max(1),
                    )) => {}
                    _ = tokio::signal::ctrl_c() => {
                        println!("shutdown signal received; exiting cleanly");
                        break;
                    }
                }
            }
            ExitCode::SUCCESS
        }
    }
}

fn print_heartbeat(label: &str, beat: &vigil_esa::Heartbeat) {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    println!(
        "[{ts}] {label} tick={} network={} cloud={} hsm={}",
        beat.tick, beat.network_status, beat.cloud_status, beat.hsm_status
    );
}
