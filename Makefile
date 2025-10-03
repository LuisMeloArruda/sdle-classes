ADDRESS = 127.0.0.1:9876

build:
	cargo build -r

clean:
	cargo clean

# c00_hello:
client_c00_hello: build
	./examples/c00_hello client $(ADDRESS)

server_c00_hello: build
	./examples/c00_hello server $(ADDRESS)

# c00_pubsub:
client_c00_pubsub: build
	./examples/c00_pubsub subscriber $(ADDRESS) 0

server_c00_pubsub: build
	./examples/c00_pubsub publisher $(ADDRESS)
