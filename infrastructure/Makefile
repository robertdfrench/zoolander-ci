include lib.mk

terraform=$(call which, terraform)
jq=$(call which, jq)

install: $(terraform) .terraform/plugins/init ##: Deploy latest infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(call assertEnv, DOMAIN_NAME)
	$(terraform) apply -auto-approve -var 'parent_zone=$(DOMAIN_NAME)'
	$(terraform) output -json > host.json

uninstall: $(terraform) .terraform/plugins/init ##: Remove any deployed infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) destroy -auto-approve

# Download plugins for each resource provider
.terraform/plugins/init: $(terraform) providers.tf
	$(terraform) init
	@touch $@

clean: ##: Remove any purely-local assets (not terraform state)
	rm -rf .terraform ami.auto.tfvars.json host.json ami.auto.tfvars.json
