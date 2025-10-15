use pluribus::pluribus;

mod c00_hello;
mod c00_pubsub;
mod c01_polling;
mod c01_queue;
mod c02_xpubxsub;

fn main() -> anyhow::Result<()> {
    pluribus!(
        symbol: main;
        returns: anyhow::Result<()>;
        with:
        - c00_hello;
        - c00_pubsub;
        - c01_polling;
        - c01_queue;
        - c02_xpubxsub;
    )(&std::env::args().collect::<Vec<_>>())
}
