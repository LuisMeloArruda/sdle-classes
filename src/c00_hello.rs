use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use tokio::time::sleep;
use zeromq::{ZmqMessage, prelude::*};

/// Types of behaviour for this program: a server, a client, ...
#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the server, specifying the bind addr.
    Server { addr: SocketAddr },
    /// Run the client, specifying the remote addr.
    Client { addr: SocketAddr },
}

/// Used with `clap` crate to handle the CLI arguments
#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Mode,
}

const SERVER_REPLY: &'static str = "World";

/// Sync function that calls the async main function.
/// Needed because the `pluribus` crate cannot call async functions.
pub fn main<'a>(args: impl IntoIterator<Item = &'a String>) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async { main_impl(args).await })
}

/// Entry point of this program.
/// Based on the CLI arguments, call the server or client handlers.
pub async fn main_impl(args: impl IntoIterator<Item = &String>) -> anyhow::Result<()> {
    let cli = Cli::parse_from(args.into_iter());

    match cli.cmd {
        Mode::Server { addr } => server_handler(addr).await,
        Mode::Client { addr } => client_handler(addr).await,
    }
}

/// Server code.
/// In this example, the server responds with "World" everytime it receives
/// a message from a client.
async fn server_handler(bind_addr: SocketAddr) -> anyhow::Result<()> {
    let mut sock = zeromq::RepSocket::new();
    sock.bind(format!("tcp://{bind_addr}").as_str()).await?;

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

/// Client code.
/// In this example, the client sends "Hello" and expects a "World" return
/// message 10 times.
async fn client_handler(connect_addr: SocketAddr) -> anyhow::Result<()> {
    println!("Connecting to hello world server...");
    let mut sock = zeromq::ReqSocket::new();
    sock.connect(format!("tcp://{connect_addr}").as_str())
        .await?;

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
    Ok(())
}
