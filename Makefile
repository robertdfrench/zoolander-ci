include lib.mk

cargo=$(call which, cargo)
git=$(call which, git)
jq=$(call which, jq)

build: $(cargo) ##: Build the application
	$(cargo) build

test: $(cargo) ##: Run unit tests
	$(cargo) test --tests "test"

check: $(cargo) ##: Run integration tests
	$(cargo) test --tests "integration"

launch: test $(cargo) ##: Launch new FCGI server, killing old one if it exists
	kill -9 `ps aux | awk '/zoola/ { print $$2 }'` || true
	nohup $(cargo) run >/tmp/zoolander.out 2>/tmp/zoolander.err &

deploy: remote $(git)  ##: Launch on a working host
	$(git) add . \
		&& $(git) commit --allow-empty \
		&& $(git) push -u zoolander +HEAD:master

shell: remote.txt ##: Get a root shell on the zoolander host
	ssh `cat remote.txt | cut -d':' -f1`

delete: ##: Tear down your zoolander instance
	$(MAKE) -C infrastructure uninstall

remote: $(git) remote.txt  ##: Set up the zoolander remote
	($(git) remote | grep zoolander > /dev/null) \
		|| git remote add zoolander `cat remote.txt`
	($(git) remote -v | grep `cat remote.txt` > /dev/null) \
		|| git remote set-url zoolander `cat remote.txt`

rotate: remote.txt ##: Rotate your instance's API tokens
	@printf "GitHub Personal Access Token: "
	@read -s GHPAT && echo "$$GHPAT" \
		| ssh `cat remote.txt | cut -d':' -f1` bash -c 'cat /dev/stdin > /tmp/ghpat'


.PHONY: remote.txt
remote.txt: $(jq)
	$(MAKE) -C infrastructure install
	$(jq) -r '"root@" + .zoolander.value + ":zoolander"' infrastructure/host.json > $@

ci_user=$(shell if [ `whoami` = "root" ]; then echo "derek"; else whoami; fi)
%.job:
	mkdir -p /tmp/zoolander-ci/$*
	git --work-tree /tmp/zoolander-ci/$* checkout $* -- .
	chown -R $(ci_user) /tmp/zoolander-ci/$*
	sudo -u $(ci_user) -i gmake -C /tmp/zoolander-ci/$* test check
