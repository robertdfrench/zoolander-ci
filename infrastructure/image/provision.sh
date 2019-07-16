#!/usr/bin/bash -ex
pkg install git build-essential zip
pkg set-publisher -g https://pkg.omniosce.org/r151030/extra/ extra.omnios
pkg install rust nginx-116
curl --silent \
	https://raw.githubusercontent.com/lukas2511/dehydrated/master/dehydrated \
	> /usr/bin/dehydrated \
	&& chmod +x /usr/bin/dehydrated
