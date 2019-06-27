include lib.mk

.PHONY: image

image: ##: Build AMI
	$(MAKE) -C image build
