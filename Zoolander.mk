# This is an example CI Pipeline definition under Zoolander. For a one-step
# pipeline, we simply define a "test" target.
include lib.mk

cargo=$(call which, cargo)

test:
	$(cargo) test
