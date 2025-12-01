//! This example shows how to publish [ResourceRecord]s directly to the DHT.
//!
//! Change the `Keypair::random()` to your own keypair to publish your own records.
//! Change the `packet.answers` to your own records.
//!
//! run this example from the project root:
//!     $ cargo run --example publish

use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing_subscriber;
use ed25519_dalek::SecretKey;
use hex::FromHex;
use bech32::{decode, convert_bits};



use pkarr::{Client, Keypair, SignedPacket};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Publish to DHT only, Relays only, or default to both.
    #[arg(value_enum)]
    mode: Option<Mode>,
    /// List of relays (only valid if mode is 'relays')
    #[arg(requires = "mode")]
    relays: Option<Vec<String>>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Dht,
    Relays,
    Both,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("pkarr=info")
        .init();

    let cli = Cli::parse();

    let mut builder = Client::builder();
    match cli.mode.unwrap_or(Mode::Both) {
        Mode::Dht => {
            builder.no_relays();
        }
        Mode::Relays => {
            builder.no_dht();

            if let Some(relays) = cli.relays {
                builder.relays(&relays).unwrap();
            }
        }
        _ => {}
    }
    let client = builder.build()?;

    //let keypair = Keypair::random();

        let secret_bytes: [u8; 32] = [
                199, 133, 251, 69, 66, 206, 61, 213, 151, 163, 166, 14, 142, 46, 94, 231,
                66, 126, 8, 67, 114, 56, 186, 37, 12, 18, 111, 207, 0, 223, 229, 145,
            ];

            // Crear el Keypair
            let keypair = Keypair::from_secret_key(&secret_bytes);
   

    let signed_packet = SignedPacket::builder()
        .txt("ema_foo".try_into().unwrap(), "bar_dos".try_into().unwrap(), 30)
        .sign(&keypair)?;

    let instant = Instant::now();

    println!("\nPublishing {} ...", keypair.public_key());
    println!("\nPublishing secret key {:?} ...", keypair.secret_key());

    match client.publish(&signed_packet, None).await {
        Ok(()) => {
            println!(
                "\nSuccessfully published {} in {:?}",
                keypair.public_key(),
                instant.elapsed(),
            );
        }
        Err(err) => {
            println!("\nFailed to publish {} \n {}", keypair.public_key(), err);
        }
    };

    Ok(())
}


