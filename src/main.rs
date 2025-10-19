use pluribus::pluribus;

mod c00_hello;
mod c00_pubsub;
mod c01_polling;
mod c01_queue;
mod c02_xpubxsub;
mod c02_pushpull;

/// The entry point of the program.
/// Executes one of the examples based on the name of the executable.
/// Check README.md or the pluribus documentation on how to execute the examples.
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
        - c02_pushpull;
    )(&std::env::args().collect::<Vec<_>>())
}
