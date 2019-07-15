SHELL=bash
which=$(shell (which $1 | grep -v "^no" | grep -v "not found") || echo ".need-to-install.$1")
assertEnv=@if [ -z $${$(strip $1)+x} ]; then >&2 echo "You need to define \$$$(strip $1)"; exit 1; fi

.need-to-install.%:
	$(error "You need to install $*")

.DEFAULT=help

help: # Print target descriptions for the current makefile
	@awk -F':' '/\#\#/ { printf "\033[32m%-12s\033[m\t%s\n", $$1,$$3 }' $(firstword $(MAKEFILE_LIST))
