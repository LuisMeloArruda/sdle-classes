ADDRESS = 127.0.0.1:9876

build:
	cargo build -r

clean:
	cargo clean

# c00_hello:
client_0: build
	./examples/c00_hello client $(ADDRESS)

server_0: build
	./examples/c00_hello server $(ADDRESS)
