SHELL=bash
which=$(shell which $1 || echo ".need-to-install.$1")

.need-to-install.%:
	$(error "You need to install $*")

.DEFAULT=help

help: # Print target descriptions for the current makefile
	@awk -F':' '/\#\#/ { print $$1,"\t",$$3 }' $(firstword $(MAKEFILE_LIST))

%.env: # Require an environment variable to be set
	@if [ -z $${$*+x} ]; then echo "You need to define \$$$*"; exit 1; fi
