include lib.mk

cargo=$(call which, cargo)

test:
	$(cargo) test
