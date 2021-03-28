all: target/release/libtree.so

target/release/libtree.so: src/lib.rs
	cargo build --release
