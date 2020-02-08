gen_rust_cheader:
	cd lib_rust && cbindgen -c ./cbindgen.toml --lockfile ../Cargo.lock -l c --output ./lib_rust.h

build_rust_lib:
	cd lib_rust && cargo build