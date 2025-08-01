module := "nova9"

client:
    RUST_LOG="info,wgpu=error,naga=warn,client=trace" cargo run --bin client

server:
    spacetime publish -p server -y {{module}} -c

bindings:
    spacetime generate --out-dir client/src/bindings --lang rust -p server

sc: server client
