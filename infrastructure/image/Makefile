include lib.mk

packer=$(call which, packer)
jq=$(call which, jq)

install: packed.state ##: Build an updated AMI
	@echo + Image installation complete

packed.state: $(packer) vars.json packer.json provision.sh ##: Build an updated AMI
	$(packer) build -var-file=vars.json packer.json
	@touch $@

vars.json: $(jq) omnios_by_region.json $(AWS_DEFAULT_REGION).txt
	$(call assertEnv, AWS_DEFAULT_REGION)
	$(jq) \
		'{region: "$(AWS_DEFAULT_REGION)", omnios_ami: ."'$(AWS_DEFAULT_REGION)'"}' \
		omnios_by_region.json > $@

%.txt:
	@touch $@

.txt:
	$(error You did not specify AWS_DEFAULT_REGION)

deregister_old: filter_old_image_ids.jq  ##: Clean up any old images
	$(call assertEnv, AWS_DEFAULT_REGION)
	aws ec2 describe-images --owners self \
		| jq -rf $< \
		| xargs -n1 aws ec2 deregister-image --image-id

clean: ##: Remove any purely-local assets (not images)
	rm -rf vars.json
