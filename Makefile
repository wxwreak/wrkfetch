BINARY_NAME=wrkfetch
LOCAL_BIN_DIR=$(HOME)/.local/bin

.PHONY: all install clean
all: build

build:
	cargo build --release

install: build
	cp target/release/${BINARY_NAME} $(LOCAL_BIN_DIR)/${BINARY_NAME}
	@echo "Installed ${BINARY_NAME} to $(LOCAL_BIN_DIR)/${BINARY_NAME}"

clean: 
	cargo clean
