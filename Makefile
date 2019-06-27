include lib.mk

ami.auto.tfvars.json: image/ami.json
	cp $< $@

image/ami.json: ##: Build AMI
	$(MAKE) -C image ami.json
