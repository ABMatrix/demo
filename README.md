# demo


cargo build --target=x86_64-unknown-linux-musl --release


cp ./target/x86_64-unknown-linux-musl/release/demo .


docker build -t demo -f Dockerfile.server .


nitro-cli build-enclave --docker-uri demo --output-file demo.eif


./nitro-cli-config -t 2 -m 256

nitro-cli run-enclave --eif-path demo.eif --cpu-count 2 --memory 256 --debug-mode

nitro-cli console --enclave-id $id
