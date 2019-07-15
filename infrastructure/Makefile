include lib.mk

terraform=$(call which, terraform)
jq=$(call which, jq)

install: host.json ##: Deploy latest infrastructure

.PHONY: host.json
host.json: $(terraform) .terraform/plugins/init
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(call assertEnv, DOMAIN_NAME)
	$(terraform) apply -auto-approve -var 'parent_zone=$(DOMAIN_NAME)'
	$(terraform) output -json host > $@

uninstall: $(terraform) .terraform/plugins/init ##: Remove any deployed infrastructure
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(call assertEnv, DOMAIN_NAME)
	$(terraform) destroy -auto-approve -var 'parent_zone=$(DOMAIN_NAME)'

# Download plugins for each resource provider
.terraform/plugins/init: $(terraform) providers.tf
	$(terraform) init
	@touch $@

clean: ##: Remove any purely-local assets (not terraform state)
	rm -rf .terraform ami.auto.tfvars.json host.json ami.auto.tfvars.json
