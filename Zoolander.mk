include lib.mk

cargo=$(call which, cargo)

test:
	@echo Running CI for $*
	$(cargo) test
