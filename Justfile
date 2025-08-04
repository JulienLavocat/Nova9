module := "nova9"

client:
    RUST_LOG="info,wgpu=error,naga=warn,client=trace" SPACETIMEDB_URI="https://stdb.jlavocat.eu" cargo run --bin client --features dev

server:
    spacetime publish -p server -y {{module}} -c

bindings:
    spacetime generate --out-dir client/src/bindings --lang rust -p server

sc: server client

maincloud:
    spacetime publish -s maincloud -p server -y {{module}} -c

maincloud_client:
    cargo run --bin client
