use std::{io::Write, net::SocketAddr, time::Duration};
use clap::Parser;
use rand::Rng;
use tokio::time::{sleep, Instant};
use zeromq::prelude::*;

#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the Ventilator, specifying the sink addr and bind addr for workers.
    Ventilator {
        sender: SocketAddr,
        sink: SocketAddr,
    },
    /// Run the Worker, specifying the sink and ventilator addresses.
    Worker {
        receiver: SocketAddr,
        sender: SocketAddr,
    },
    /// Run the Sink, specifying the bind addr for workers.
    Sink { receiver: SocketAddr },
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
        Mode::Ventilator { sender, sink } => ventilator_handler(sender, sink).await,
        Mode::Worker { receiver, sender } => worker_handler(receiver, sender).await,
        Mode::Sink { receiver } => sink_handler(receiver).await,
    }
}

async fn ventilator_handler(sender_addr: SocketAddr, sink_addr: SocketAddr) -> anyhow::Result<()> {
    let mut sender = zeromq::PushSocket::new();
    sender.bind(format!("tcp://{sender_addr}").as_str()).await?;
    let mut sink = zeromq::PushSocket::new();
    sink.connect(format!("tcp://{sink_addr}").as_str()).await?;

    println!("Press Enter when the workers are ready: ");
    std::io::stdin().read_line(&mut String::new())?;
    println!("Sending tasks to workers...");

    sink.send("0".into()).await?; // Signal the start of a batch

    let mut rng = rand::rng();
    let mut total_msec = 0;
    for _ in 0..100 {
        let workload = rng.random_range(1..100u64);
        total_msec += workload;
        sender.send(format!("{workload}").into()).await?
    }
    
    println!("Total expected cost: {total_msec} msec");
    
    sink.close().await;
    sender.close().await;
    Ok(())
}

async fn worker_handler(receiver_addr: SocketAddr, sender_addr: SocketAddr) -> anyhow::Result<()> {
    let mut receiver = zeromq::PullSocket::new();
    receiver.connect(format!("tcp://{receiver_addr}").as_str()).await?;
    let mut sender = zeromq::PushSocket::new();
    sender.connect(format!("tcp://{sender_addr}").as_str()).await?;
    
    loop {
        let bytes = match receiver.recv().await {
            Ok(bytes) => bytes,
            Err(e) => {eprintln!("{e}"); continue;}
        };
        
        let msg = match String::try_from(bytes) {
            Ok(msg) => msg,
            Err(e) => {eprintln!("{e}"); continue;}
        };
        
        let num = match msg.parse::<u64>() {
            Ok(num) => num,
            Err(e) => {eprintln!("{e}"); continue;}
        };
        
        println!("Got task: Sleep for {num} msec");
        
        sleep(Duration::from_millis(num)).await;  // Faking a long computation
        sender.send("".into()).await?;    // "" is the result of the computation
    }
}

async fn sink_handler(receiver_addr: SocketAddr) -> anyhow::Result<()> {
    let mut receiver = zeromq::PullSocket::new();
    receiver.bind(format!("tcp://{receiver_addr}").as_str()).await?;
        
    let start_of_batch = String::try_from(receiver.recv().await?).expect("Failed to get string from message.");
    assert!("0".to_string() == start_of_batch);
        
    let start = Instant::now();
    
    for task_number in 0..100 {
        let msg = String::try_from(receiver.recv().await?).expect("Failed to get string from message.");
        assert!(msg.is_empty());
        
        std::io::stdout().flush()?;
        match task_number % 10 == 0 {
            true => print!(":"),
            false => print!("."),
        }
        std::io::stdout().flush()?;
    }
    println!("");
    
    let elapsed_time = (Instant::now() - start).as_millis();
    println!("Total elapsed time: {elapsed_time} msec");
    
    receiver.close().await;
    Ok(())
}
