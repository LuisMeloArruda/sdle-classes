# SDLE examples

Examples in Rust of the [ZeroMQ guide](https://zguide.zeromq.org/docs/chapter1/) suggested in the SDLE classes

> Faculty of Engineering at University of Porto
> Master's Degree in Informatics and Computing Engineering
> Large Scale Distributed Systems (M.EIC004) 2025/2026
>
> - Carlos Miguel Ferraz Baquero-Moreno (Co-regent of the course)
> - Pedro Alexandre Guimarães Lobo Ferreira Souto (Co-regent of the course)
> - Francisco António Ferraz Martins de Almeida Maia (Co-regent of the course)
> - José Pedro Peixoto Ferreira (Theoretical-Practical classes)

> **Class 05; Group ??**
>
> - Guilherme Duarte Silva Matos (up202208755@up.pt)
> - João Vítor da Costa Ferreira (up202208393@up.pt)
> - Luís Miguel Melo Arruda (up202206970@up.pt)

## Examples:

- [`c00_hello.rs`](./src/c00_hello.rs): Various clients can send 10 "Hello" messages and the server will reply to each with "World";

- [`c00_pubsub.rs`](./src/c00_pubsub.rs): Each client subscribe to a topic and the publisher (i.e., the server) sends updates to the clients associated with that topic.

- [`c01_polling.rs`](./src/c01_polling.rs): In a Pub/Sub architecture, a client can connect to two publishers with a US and PT zipcodes. For that, polling is used on the client.

- [`c01_queue.rs`](./src/c01_queue.rs): A "Hello World" example with a Dealer/Router architecture, where all clients connect to the Router of the Broker and all the servers connect to the Dealer of the Broker.

- [`c02_xpubxsub.rs`](./src/c02_xpubxsub.rs): A Pub/Sub "zipcode example" with a broker in the middle. The broker is a subscriber of all topics and
a single publisher for all the subscribers.

- [`c02_pushpull.rs`](./src/c02_pushpull.rs): A Divide and Conquer stategy example where a ventilator gives 100 tasks (sleep between 1ms and 100ms and return `""`) to workers, which give the result to the sink.

## Prerequisites:
- Rust stable with Cargo in version 1.90.0 or higher;
- GNU Make toolkit.

## Usage:

```bash
make [client|server|broker]_<example_name>
```

For example, to spawn a client of the example inside the file `c00_hello.rs`, execute `make client_c00hello`.
