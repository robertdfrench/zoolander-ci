include lib.mk

terraform=$(call which, terraform)

install: AWS_DEFAULT_REGION.env $(terraform) ami.auto.tfvars.json .terraform/plugins/init zoolander.tf ##: Deploy latest infrastructure
	$(terraform) apply -auto-approve

ami.auto.tfvars.json: # Generate AMI listing from image/ target
	$(MAKE) -C image ami.json
	cp image/ami.json $@

.terraform/plugins/init: $(terraform) providers.tf # Download any needed terraform plugins
	$(terraform) init
	@touch $@
