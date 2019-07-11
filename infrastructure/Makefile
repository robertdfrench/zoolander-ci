include lib.mk

terraform=$(call which, terraform)

install: $(terraform) .terraform/plugins/init zoolander.tf ##: Deploy latest infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(MAKE) -C image ami.json
	cp image/ami.json ami.auto.tfvars.json
	$(MAKE) -C dns ip.json
	cp dns/ip.json ip.auto.tfvars.json
	$(terraform) apply -auto-approve
	$(terraform) output -json > host.json

uninstall: $(terraform) .terraform/plugins/init ##: Remove any deployed infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) destroy -auto-approve

# Download plugins for each resource provider
.terraform/plugins/initialized.state: $(terraform) providers.tf
	$(terraform) init
	@touch $@

clean: ##: Remove any purely-local assets (not terraform state)
	rm -rf .terraform ami.auto.tfvars.json host.json
