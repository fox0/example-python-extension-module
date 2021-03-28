all: target/release/libtree.so libtree.so

target/release/libtree.so: src/lib.rs
	cargo build --release

libtree.so:
	ln -s target/release/libtree.so $@

clean:
	cargo clean
