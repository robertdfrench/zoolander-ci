include lib.mk

packer=$(call which, packer)
terraform=$(call which, terraform)

packed.txt: $(packer) packer.json provision.sh ##: Create an update AMI
	$(packer) build packer.json
	@touch $@

.terraform/plugins/init: $(terraform) providers.tf
	$(terraform) init
	@touch $@

ami.json: .terraform/plugins/init ami.tf packed.txt ##: Build and list ami
	$(terraform) apply -auto-approve
	$(terraform) output -json > $@
