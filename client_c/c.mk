run_c:
	gcc client_c/src/main.c -L target/debug -l lib_rust -o build/main && ./build/main

compilation:
	gcc -S client_c/src/main.c -o build/main.s

assambly:
	as -o build/main.o build/main.s

# Try list the default include-paths in your system used by the "GNU C Preprocessor" via "cpp -v":
verbose:
	gcc -v -o build/main client_c/src/main.c