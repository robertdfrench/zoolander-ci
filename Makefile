include lib.mk

cargo=$(call which, cargo)
git=$(call which, git)
jq=$(call which, jq)

build: $(cargo) ##: Build the application
	$(cargo) build

test: $(cargo) ##: Run unit tests
	$(cargo) test

check: add_remote $(git)  ##: Run tests on a working host
	$(git) add . \
		&& $(git) commit --allow-empty \
		&& $(git) push -u zoolander +HEAD:master

shell: remote.txt ##: Get a root shell on the zoolander host
	ssh `cat remote.txt | cut -d':' -f1`

add_remote: $(git) remote.txt  ##: Set up the zoolander remote
	($(git) remote | grep zoolander > /dev/null) \
		|| git remote add zoolander `cat remote.txt`
	($(git) remote -v | grep `cat remote.txt` > /dev/null) \
		|| git remote set-url zoolander `cat remote.txt`

install: host.json ##: Stand up host infrastructure

.PHONY: remote.txt
remote.txt: $(jq)
	$(MAKE) -C host install
	$(jq) -r '"root@" + .zoolander.value + ":zoolander"' host/host.json > $@
