include lib.mk

cargo=$(call which, cargo)

build: $(cargo) ##: Build the application
	$(cargo) build

install: ##: Stand up host infrastructure
	$(MAKE) -C host install
