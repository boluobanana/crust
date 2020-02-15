run_client_rust:
	./build/rust_client

compile_client_rust:
	cd client_rust && rustc ./src/main.rs -o ../build/rust_client -L ../build -l lib_c


