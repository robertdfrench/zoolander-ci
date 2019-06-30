include lib.mk

packer=$(call which, packer)
terraform=$(call which, terraform)
jq=$(call which, jq)

install: ami.json $(jq) ##: Build an updated AMI
	$(jq) '.' $<

# Machine-readable description of the most recently built AMI
ami.json: .terraform/plugins/init ami.tf packed.state
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) apply -auto-approve
	$(terraform) output -json > $@

# Download plugins for each resource provider
.terraform/plugins/initialized.state: $(terraform) providers.tf
	$(terraform) init
	@touch $@

# Checkpoint for a successful packer run
packed.state: $(packer) packer.json provision.sh
	$(packer) build packer.json
	@touch $@

clean: ##: Remove any purely-local assets (not Terraform state)
	rm -rf ami.json *.state .terraform/
