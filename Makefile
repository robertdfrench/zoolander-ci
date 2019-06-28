include lib.mk

terraform=$(call which, terraform)

install: AWS_DEFAULT_REGION.env $(terraform) .terraform/plugins/init zoolander.tf ##: Deploy latest infrastructure
	$(MAKE) -C image ami.json
	cp image/ami.json ami.auto.tfvars.json
	$(terraform) apply -auto-approve

uninstall: AWS_DEFAULT_REGION.env $(terraform) .terraform/plugins/init  ##: Remove any deployed infrastructure
	$(terraform) destroy

.terraform/plugins/init: $(terraform) providers.tf # Download any needed terraform plugins
	$(terraform) init
	@touch $@
