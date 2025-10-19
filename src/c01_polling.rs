use std::{net::SocketAddr, str::FromStr, time::Duration};

use anyhow::anyhow;
use clap::Parser;
use rand::Rng;
use tokio::{io::AsyncWriteExt, time::sleep};
use zeromq::{ZmqMessage, prelude::*};

#[derive(Debug, clap::Subcommand)]
enum Mode {
    /// Run the US Publisher, specifying the bind addr.
    Publisher { addr: SocketAddr, country: Country },
    /// Run the Subscriber, specifying the remote addr and topic.
    Subscriber {
        us_addr: SocketAddr,
        pt_addr: SocketAddr,
        zip: Vec<ZipCode>,
    },
}

#[derive(Debug, Clone)]
enum Country {
    Pt,
    Us,
}

impl FromStr for Country {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pt" | "Pt" | "PT" => Ok(Country::Pt),
            "us" | "Us" | "US" => Ok(Country::Us),
            e => Err(anyhow!("Unsupported code: '{e}'.")),
        }
    }
}

#[derive(Debug, Clone)]
struct ZipCode {
    country: Country,
    zip: u32,
}

impl FromStr for ZipCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(anyhow!("Is not ascii"));
        }
        let bytes = s.as_bytes();
        if bytes.len() < 3 {
            return Err(anyhow!("Too small"));
        }

        match &bytes[0..3] {
            b"PT:" => {
                if bytes.len() > 7 {
                    return Err(anyhow!("Pt: too long zipcode"));
                }
                let code = u32::from_str_radix(String::from_utf8_lossy(&bytes[3..]).as_ref(), 10)?;
                Ok(Self {
                    country: Country::Pt,
                    zip: code,
                })
            }
            b"US:" => {
                if bytes.len() > 8 {
                    return Err(anyhow!("Us: too long zipcode"));
                }
                let code = u32::from_str_radix(String::from_utf8_lossy(&bytes[3..]).as_ref(), 10)?;
                Ok(Self {
                    country: Country::Us,
                    zip: code,
                })
            }
            _ => Err(anyhow!("Unsupported {s}")),
        }
    }
}

impl std::fmt::Display for ZipCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.country {
            Country::Pt => write!(f, "PT:{:04}", self.zip),
            Country::Us => write!(f, "US:{:05}", self.zip),
        }
    }
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
        Mode::Subscriber {
            zip,
            us_addr,
            pt_addr,
        } => sub_handler(us_addr, pt_addr, zip).await,
        Mode::Publisher { addr, country } => pub_handler(addr, country).await,
    }
}

async fn pub_handler(addr: SocketAddr, country: Country) -> anyhow::Result<()> {
    let mut sock = zeromq::PubSocket::new();
    sock.bind(format!("tcp://{addr}").as_str()).await?;

    let mut rng = rand::rng();
    loop {
        let zipcode = match &country {
            Country::Pt => ZipCode {
                country: country.clone(),
                zip: rng.random_range(0..=9999),
            },
            Country::Us => ZipCode {
                country: country.clone(),
                zip: rng.random_range(0..=99999),
            },
        };

        let temperature = rng.random_range(-14..40);
        let relhumidity = rng.random_range(0..=100);
        let update = format!(
            "Update for {zipcode}:\n  Temperature: {temperature}ºC\n  Humidity: {relhumidity}%.\n"
        );
        let Err(e) = sock.send(update.into()).await else {
            sleep(Duration::from_millis(1)).await;
            continue;
        };

        eprintln!("Error sending message for ZIP {zipcode:05}: {e}");
    }
}

async fn sub_handler(
    us_addr: SocketAddr,
    pt_addr: SocketAddr,
    zip: Vec<ZipCode>,
) -> anyhow::Result<()> {
    println!("Connecting to weather server...");
    let mut us_sock = zeromq::SubSocket::new();
    let mut pt_sock = zeromq::SubSocket::new();
    for z in zip.iter() {
        match z.country {
            Country::Pt => &mut pt_sock,
            Country::Us => &mut us_sock,
        }
        .subscribe(format!("Update for {z}:\n").as_str())
        .await?;
    }
    us_sock.connect(format!("tcp://{us_addr}").as_str()).await?;
    pt_sock.connect(format!("tcp://{pt_addr}").as_str()).await?;

    loop {
        tokio::select! {
            Ok(msg) = us_sock.recv() => dispatch_msg(msg).await?,
            Ok(msg) = pt_sock.recv() => dispatch_msg(msg).await?,
        }
    }
}

async fn dispatch_msg(msg: ZmqMessage) -> anyhow::Result<()> {
    let msg = msg.into_vec();
    for b in msg {
        tokio::io::stdout().write_all(&b).await?;
    }
    tokio::io::stdout().flush().await?;
    Ok(())
}
