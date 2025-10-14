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
	./examples/c00_pubsub subscriber $(ADDRESS) 3

server_c00_pubsub: build
	./examples/c00_pubsub publisher $(ADDRESS)

# c01_polling:
client_c01_polling: build
	./examples/c01_polling subscriber 127.0.0.1:9877 $(ADDRESS) US:1234 PT:5678 

server_c01_polling_pt: build
	./examples/c01_polling publisher $(ADDRESS) PT

server_c01_polling_en: build
	./examples/c01_polling publisher 127.0.0.1:9877 US
