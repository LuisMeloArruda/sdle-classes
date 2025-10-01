use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use tokio::time::sleep;
use zeromq::{ZmqMessage, prelude::*};

#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the server, specifying the bind addr.
    Server { addr: SocketAddr },
    /// Run the client, specifying the remote addr.
    Client { addr: SocketAddr },
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Mode,
}

const SERVER_REPLY: &'static str = "World";

pub fn main<'a, I: IntoIterator<Item = &'a String>>(args: I) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async { main_impl(args).await })
}

pub async fn main_impl<'a, I: IntoIterator<Item = &'a String>>(args: I) -> anyhow::Result<()> {
    let cli = Cli::parse_from(args.into_iter());

    match cli.cmd {
        Mode::Server { addr } => {
            let mut sock = zeromq::RepSocket::new();
            sock.bind(format!("tcp://{addr}").as_str()).await?;

            loop {
                match sock.recv().await {
                    Ok(msg) => {
                        if msg.len() == 0 {
                            println!("ERROR: Msg empty!");
                            continue;
                        }
                        println!("Received Hello");

                        sleep(Duration::from_secs(1)).await;

                        let reply = ZmqMessage::from(SERVER_REPLY);
                        sock.send(reply).await?;
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
        Mode::Client { addr } => {
            println!("Connecting to hello world server...");
            let mut sock = zeromq::ReqSocket::new();
            sock.connect(format!("tcp://{addr}").as_str()).await?;

            for i in 0..10 {
                println!("Sending Hello {i}...");
                sock.send(ZmqMessage::from("Hello")).await?;
                match sock.recv().await {
                    Ok(_) => {
                        println!("Received World {i}...");
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
    }

    Ok(())
}
