# Rust-tcp-rsa
Simple chat (one-way: alice -> bob) using RSA asymmetric encryption over TCP connection

### Run:
```shell
git clone https://github.com/yuhime/rust-tcp-rsa.git && cd rust-tcp-rsa
```
Open two windows in your terminal: one for bob and one for alice then:
```shell
# bob
cargo run --example bob

# alice
cargo run --example alice
```