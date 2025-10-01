use pluribus::pluribus;

mod c00_hello;

fn main() -> anyhow::Result<()> {
    pluribus!(
        symbol: main;
        returns: anyhow::Result<()>;
        with:
        - c00_hello;
    )(&std::env::args().collect::<Vec<_>>())
}
