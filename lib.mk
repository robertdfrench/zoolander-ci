SHELL=bash
which=$(shell which $1 || echo ".need-to-install.$1")
assertEnv=$(shell if [ -z $${$(strip $1)+x} ]; then echo "You need to define \$$$(strip $1)"; exit 1; fi)

.need-to-install.%:
	$(error "You need to install $*")

.DEFAULT=help

help: # Print target descriptions for the current makefile
	@awk -F':' '/\#\#/ { print $$1,"\t",$$3 }' $(firstword $(MAKEFILE_LIST))
