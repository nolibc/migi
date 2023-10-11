# Makefile

INSTALL_PREFIX = /usr/local/bin

build:
	@which cargo > /dev/null
	cargo build --release

install :
	mkdir -p $(INSTALL_PREFIX)
	cp -f target/release/migi $(INSTALL_PREFIX)

uninstall :
	@which migi > /dev/null
	rm -f $(INSTALL_PREFIX)/migi

.PHONY: build install uninstall
