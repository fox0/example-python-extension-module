all: target/release/libtree.so libtree.so

target/release/libtree.so: src/lib.rs
	cargo build --release

libtree.so:
	ln -s $< $@

clean:
	cargo clean
