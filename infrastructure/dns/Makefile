include lib.mk

terraform=$(call which, terraform)
jq=$(call which, jq)

install: ip.json $(jq) ##: Create an Elastic IP resource
	$(jq) '.' $<

remove: $(terraform) .terraform/plugins/init ##: Release the Elastic IP back to Amazon
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) destroy -auto-approve
	rm -f ip.json

# Machine-readable description of current Elastic IP
ip.json: $(terrafom) .terraform/plugins/init dns.tf
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) apply -auto-approve
	$(terraform) output -json > $@
	
# Download plugins for each resource provider
.terraform/plugins/init: $(terraform) providers.tf
	$(terraform) init
	@touch $@
