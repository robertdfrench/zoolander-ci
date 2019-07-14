include lib.mk

packer=$(call which, packer)
terraform=$(call which, terraform)
jq=$(call which, jq)

install: ami.json $(jq) ##: Build an updated AMI
	$(jq) '.' $<

# Machine-readable description of the most recently built AMI
ami.json: $(terraform) .terraform/plugins/init ami.tf packed.state
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(terraform) apply -auto-approve
	$(terraform) output -json > $@

# Download plugins for each resource provider
.terraform/plugins/init: $(terraform) providers.tf
	$(terraform) init
	@touch $@

# Checkpoint for a successful packer run
packed.state: $(packer) vars.json packer.json provision.sh
	$(packer) build -var-file=vars.json packer.json
	@touch $@

vars.json: $(jq) omnios_by_region.json
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(jq) \
		'{region: "$(AWS_DEFAULT_REGION)", omnios_ami: ."'$(AWS_DEFAULT_REGION)'"}' \
		omnios_by_region.json > $@

deregister_old: filter_old_image_ids.jq  ##: Clean up any old images
	$(call assertEnv, AWS_DEFAULT_REGION)
	aws ec2 describe-images --owners self \
		| jq -rf $< \
		| xargs -n1 aws ec2 deregister-image --image-id

clean: ##: Remove any purely-local assets (not Terraform state)
	rm -rf ami.json *.state .terraform/
