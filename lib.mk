SHELL=bash
which=$(shell which $1 || echo ".need-to-install.$1")

.need-to-install.%:
	$(error "You need to install $*")

.DEFAULT=help

help:
	@awk -F':' '/\#\#/ { print $$1,"\t",$$3 }' $(firstword $(MAKEFILE_LIST))
