use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use rand::Rng;
use tokio::{io::AsyncWriteExt, time::sleep};
use zeromq::prelude::*;

#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the Publisher, specifying the bind addr.
    Publisher { addr: SocketAddr },
    /// Run the Subscriber, specifying the remote addr and topic.
    Subscriber { addr: SocketAddr, topic: u32 },
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Mode,
}

pub fn main<'a, I: IntoIterator<Item = &'a String>>(args: I) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async { main_impl(args).await })
}

pub async fn main_impl<'a, I: IntoIterator<Item = &'a String>>(args: I) -> anyhow::Result<()> {
    let cli = Cli::parse_from(args.into_iter());

    match cli.cmd {
        Mode::Publisher { addr } => pub_handler(addr).await,
        Mode::Subscriber { addr, topic } => sub_handler(addr, topic).await,
    }
}

async fn pub_handler(bind_addr: SocketAddr) -> anyhow::Result<()> {
    let mut sock = zeromq::PubSocket::new();
    sock.bind(format!("tcp://{bind_addr}").as_str()).await?;

    let mut rng = rand::rng();
    loop {
        let zipcode = rng.random_range(0..100000);
        let temperature = rng.random_range(-14..40);
        let relhumidity = rng.random_range(0..=100);
        let update = format!(
            "Update for {zipcode:05}:\n  Temperature: {temperature}ºC\n  Humidity: {relhumidity}%.\n"
        );
        let Err(e) = sock.send(update.into()).await else {
            sleep(Duration::from_millis(1)).await;
            continue;
        };

        eprintln!("Error sending message for ZIP {zipcode:05}: {e}");
    }
}

async fn sub_handler(connect_addr: SocketAddr, topic: u32) -> anyhow::Result<()> {
    println!("Connecting to weather server...");
    let mut sock = zeromq::SubSocket::new();
    sock.subscribe(format!("Update for {topic:05}:\n").as_str())
        .await?;
    sock.connect(format!("tcp://{connect_addr}").as_str())
        .await?;

    loop {
        match sock.recv().await {
            Ok(msg) => {
                let msg = msg.into_vec();
                for b in msg {
                    tokio::io::stdout().write_all(&b).await?;
                }
                tokio::io::stdout().flush().await?;
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}
