use pluribus::pluribus;

mod c00_hello;
mod c00_pubsub;

fn main() -> anyhow::Result<()> {
    pluribus!(
        symbol: main;
        returns: anyhow::Result<()>;
        with:
        - c00_hello;
        - c00_pubsub;
    )(&std::env::args().collect::<Vec<_>>())
}
