use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use tokio::time::sleep;
use zeromq::{ZmqMessage, prelude::*};

#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the worker/server, specifying the bind addr.
    Worker { addr: SocketAddr },
    /// Run the client, specifying the remote addr.
    Client { addr: SocketAddr },
    /// Run the broker, specifying the addresses of the client and the server.
    Broker { client_addr: SocketAddr, worker_addr: SocketAddr },
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Mode,
}

const SERVER_REPLY: &'static str = "World";

pub fn main<'a>(args: impl IntoIterator<Item = &'a String>) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async { main_impl(args).await })
}

pub async fn main_impl(args: impl IntoIterator<Item = &String>) -> anyhow::Result<()> {
    let cli = Cli::parse_from(args.into_iter());

    match cli.cmd {
        Mode::Worker { addr } => worker_handler(addr).await,
        Mode::Client { addr } => client_handler(addr).await,
        Mode::Broker { client_addr, worker_addr } => broker_handler(client_addr, worker_addr).await,
    }
}

async fn broker_handler(client_addr: SocketAddr, worker_addr: SocketAddr) -> anyhow::Result<()> {
    let mut frontend = zeromq::RouterSocket::new();
    frontend.bind(format!("tcp://{client_addr}").as_str()).await?;
    let mut backend = zeromq::DealerSocket::new();
    backend.bind(format!("tcp://{worker_addr}").as_str()).await?;
    
    zeromq::proxy(frontend, backend, None).await?;
    
    Ok(())
}

async fn worker_handler(addr: SocketAddr) -> anyhow::Result<()> {
    let mut sock = zeromq::RepSocket::new();
    sock.connect(format!("tcp://{addr}").as_str()).await?;
    
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
