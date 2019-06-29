include lib.mk

terraform=$(call which, terraform)

install: host.json; ##: Deploy latest infrastructure

host.json: $(terraform) .terraform/plugins/init zoolander.tf
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(MAKE) -C image ami.json
	cp image/ami.json ami.auto.tfvars.json
	$(terraform) apply -auto-approve
	$(terraform) output -json > $@

uninstall: $(terraform) .terraform/plugins/init  ##: Remove any deployed infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) destroy

.terraform/plugins/init: $(terraform) providers.tf # Download any needed terraform plugins
	$(terraform) init
	@touch $@
