include lib.mk

cargo=$(call which, cargo)
git=$(call which, git)
jq=$(call which, jq)

build: $(cargo) ##: Build the application
	$(cargo) build

test: $(cargo) ##: Run unit tests
	$(cargo) test

check: add_remote $(git)
	$(git) add . \
		&& $(git) commit --amend \
		&& $(git) push -u zoolander --force

add_remote: $(git) remote.txt  ## Set up the zoolander remote
	($(git) remote | grep zoolander > /dev/null) \
		|| git remote add zoolander `cat remote.txt`
	($(git) remote -v | grep `cat remote.txt` > /dev/null) \
		|| git remote set-url zoolander `cat remote.txt`

install: host.json ##: Stand up host infrastructure

remote.txt: $(jq)
	$(MAKE) -C host host.json
	$(jq) -r '"root@" + .zoolander.value + ":zoolander"' host/host.json > $@
