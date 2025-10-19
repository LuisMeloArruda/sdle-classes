ADDRESS1 = 127.0.0.1:9876
ADDRESS2 = 127.0.0.1:9877

build:
	cargo build -r

clean:
	cargo clean

# c00_hello:
client_c00_hello: build
	./examples/c00_hello client $(ADDRESS1)

server_c00_hello: build
	./examples/c00_hello server $(ADDRESS1)

# c00_pubsub:
client_c00_pubsub: build
	./examples/c00_pubsub subscriber $(ADDRESS1) 3

server_c00_pubsub: build
	./examples/c00_pubsub publisher $(ADDRESS1)

# c01_polling:
client_c01_polling: build
	./examples/c01_polling subscriber $(ADDRESS2) $(ADDRESS1) US:1234 PT:5678 

server_c01_polling_pt: build
	./examples/c01_polling publisher $(ADDRESS1) PT

server_c01_polling_en: build
	./examples/c01_polling publisher $(ADDRESS2) US
	
# c01_queue:
client_c01_queue: build
	./examples/c01_queue client $(ADDRESS1)

server_c01_queue: build
	./examples/c01_queue worker $(ADDRESS2)

broker_c01_queue: build
	./examples/c01_queue broker $(ADDRESS1) $(ADDRESS2)

# c02_xpubxsub:
client_c02_xpubxsub: build
	./examples/c02_xpubxsub subscriber $(ADDRESS1) 3

server_c02_xpubxsub: build
	./examples/c02_xpubxsub publisher $(ADDRESS2)

broker_c02_xpubxsub: build
	./examples/c02_xpubxsub broker $(ADDRESS1) $(ADDRESS2)

# c02_pushpull:
ventilator_c02_pushpull: build
	./examples/c02_pushpull ventilator $(ADDRESS1) $(ADDRESS2)

worker_c02_pushpull: build
	./examples/c02_pushpull worker $(ADDRESS1) $(ADDRESS2)

sink_c02_pushpull: build
	./examples/c02_pushpull sink $(ADDRESS2)
