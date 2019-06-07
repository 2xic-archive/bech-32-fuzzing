GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)
AFL_BIN ?= $(shell which afl-gcc)
AFL_fuzz ?= $(shell which afl-fuzz)


all:
	cd rust && $(CARGO_BIN) build --release && cd ..
	$(AFL_BIN) main.c -lm -Isrc  -L. -l:rust/target/release/librustbech32.so -o main 

afl:
	$(AFL_fuzz) -i input/ -o output/ -- ./main @@
