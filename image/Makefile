include lib.mk

packer=$(call which, packer)

build: $(packer) packer.json provision.sh ##: Create an update AMI
	$(packer) build packer.json
