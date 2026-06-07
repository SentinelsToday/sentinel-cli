mod audit;
mod cloud;
mod format;
mod keygen;

use clap::{Parser, Subcommand};
use cloud::Cloud;
use format::OutputFormat;

#[derive(Parser)]
#[command(
    name = "sentinel",
    version,
    about = "Sentinels operator CLI",
    long_about = None
)]
struct Cli {
    /// sentinel-cloud base URL.
    #[arg(long, default_value = "http://localhost:8787", global = true)]
    cloud: String,

    /// Output format: text | json
    #[arg(long, default_value = "text", value_parser = OutputFormat::parse, global = true)]
    output: OutputFormat,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Generate a fresh device keypair (does NOT touch any cloud).
    Keygen,
    /// Look up a device record.
    Device { did: String },
    /// Look up the current trust score for a device.
    Trust { did: String },
    /// Fetch and locally re-verify the hash-chained audit log.
    Audit {
        robot_id: String,
        /// Skip hash-chain verification.
        #[arg(long)]
        no_verify: bool,
    },
    /// Print the running CLI version.
    Version,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Keygen => {
            let key = keygen::generate();
            let value = serde_json::json!({
                "did": key.did,
                "public_key_hex": key.public_key_hex,
                "secret_hex": key.secret_hex,
            });
            println!("{}", format::render(&value, cli.output));
        }
        Cmd::Device { did } => {
            let device = Cloud::new(&cli.cloud).get_device(&did)?;
            println!("{}", format::render(&device, cli.output));
        }
        Cmd::Trust { did } => {
            let score = Cloud::new(&cli.cloud).get_trust(&did)?;
            println!("{}", format::render(&score, cli.output));
        }
        Cmd::Audit {
            robot_id,
            no_verify,
        } => {
            let page = Cloud::new(&cli.cloud).get_audit(&robot_id)?;
            if !no_verify {
                audit::verify(&page.entries)?;
            }
            println!("{}", format::render(&page, cli.output));
            if !no_verify {
                eprintln!("(audit chain of {} entries verified)", page.entries.len());
            }
        }
        Cmd::Version => {
            println!("sentinel {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
