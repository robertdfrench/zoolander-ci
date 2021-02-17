SHELL=bash

## Main
install: nginx letsencrypt bare-repo derek;


## Nginx Webserver
.PHONY: nginx
nginx: /etc/opt/ooce/nginx-1.16/nginx.conf
	svcadm enable svc:/network/http

/etc/opt/ooce/nginx-1.16/nginx.conf: nginx.conf
	cp $< $@


## Let's Encrypt
.PHONY: letsencrypt
letsencrypt: /etc/dehydrated/accounts /opt/ooce/nginx-1.16/html/.well-known/acme-challenge letsencrypt.crontab
	if [ ! -e /etc/dehydrated/certs/`hostname`/privkey.pem ]; then /usr/bin/dehydrated --cron --domain `hostname`; fi
	crontab  <(crontab -l && cat letsencrypt.crontab)

/etc/dehydrated/accounts:
	$(MAKE) /etc/dehydrated/config
	/usr/bin/dehydrated --register --accept-terms

/etc/dehydrated/config: /etc/dehydrated dehydrated.config
	cp dehydrated.config $@
	chown nginx $@

/etc/dehydrated: /persistent
	mkdir -p /persistent$@
	chown nginx /persistent$@
	ln -s /persistent$@ $@

/opt/ooce/nginx-1.16/html/.well-known/acme-challenge: /opt/ooce/nginx-1.16/html/.well-known
	mkdir -p $@
	chown nginx $@

/opt/ooce/nginx-1.16/html/.well-known:
	mkdir -p $@
	chown nginx $@


## Bare git repo (push-to-deploy)
.PHONY: bare-repo
bare-repo: /root/zoolander/.git/hooks/post-receive
	git config --system receive.denyCurrentBranch ignore

/root/zoolander/.git/hooks/post-receive: /root/zoolander/.git/hooks post-receive-hook.sh
	cp post-receive-hook.sh $@
	chmod +x $@

/root/zoolander/.git/hooks:
	git init /root/zoolander


## Unprivileged CI user 'derek'
.PHONY: derek
derek: /export/derek/.profile;

/export/derek/.profile: derek_profile.bash /export/derek
	cp derek_profile.bash $@
	chown derek $@

/export/derek:
	useradd -b /export -m -s /bin/bash derek

/persistent:
	$(error $@ should have been created by a previous step)
