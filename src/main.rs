use pluribus::pluribus;

mod c00_hello;
mod c00_pubsub;
mod c01_polling;

fn main() -> anyhow::Result<()> {
    pluribus!(
        symbol: main;
        returns: anyhow::Result<()>;
        with:
        - c00_hello;
        - c00_pubsub;
        - c01_polling;
    )(&std::env::args().collect::<Vec<_>>())
}
