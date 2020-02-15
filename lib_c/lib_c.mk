
gen_rust_lib:
	bindgen lib_c/src/lib.h -o client_rust/src/bindings.rs

# Wall 提供跟多debug 信息 -g 生成gdb
# ar 用来组合多个.o文件
# ranlib 更新静态库的符号索引表
# detail https://www.jianshu.com/p/2ec7ee43e3a1
compile_lib_c:
	cd lib_c/ \
	&& gcc -Wall -g -c src/lib.c -o ../build/liblib_c.o \
	&& ar ruv ../build/liblib_c.a ../build/liblib_c.o \
	&& ranlib ../build/liblib_c.a