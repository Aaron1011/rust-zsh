ZSH_VERSION = 5.4.2
INSTALL_PATH = run/zsh_install/lib/zsh/$(ZSH_VERSION)/aaron

all: target/fastbrackets

run:
	ZDOTDIR=./run ./run/zsh_install/bin/zsh

debug-install: target/debug/fastbrackets install

release-install: target/release/fastbrackets install

install:
	mkdir -p $(INSTALL_PATH)
	cp target/fastbrackets.so $(INSTALL_PATH)

target/debug/fastbrackets: src/lib.rs Cargo.toml
	cargo build -vv
	cp target/debug/libfastbrackets.so target/fastbrackets.so

target/release/libfastbrackets.so: src/lib.rs Cargo.toml
	cargo build --release
	cp target/release/libfastbrackets.so target/fastbrackets.so


clean:
	rm -r run/zsh_install
	cargo clean


.PHONY: all run debug-install release-install install clean
