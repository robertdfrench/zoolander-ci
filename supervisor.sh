#!/bin/bash
export PATH=/usr/gnu/bin:$PATH:/opt/ooce/bin

# HTTPS clone path for $repo on GitHub
remote_url() {
	repo=$1

	echo "https://github.com/${repo}.git"
}

# Canonical path for the local clone of $repo
full_repo_path() {
	repo=$1

	echo "/tmp/zoolander-ci/repos/$repo"
}

# Clone the repo to its canonical, local path
clone() {
	repo=$1

	git clone $(remote_url $repo) $(full_repo_path $repo)
}

# Only clone if we don't already have a local copy
ensure_repo_exists() {
	repo=$1

	if [ ! -d $(full_repo_path $repo) ]; then clone $repo; fi
}

# Run CI tasks as yourself, unless you are root. If you are root, run them as
# 'derek', an otherwised unprivileged user who exists for this purpose
if_root_then_derek() {
	if [ $(whoami) = "root" ]; then
		echo "derek";
	else
		whoami
	fi
}

# Canonical path for a local checkout of $repo's $ref
full_worktree_path() {
	repo=$1
	ref=$2

	echo "/tmp/zoolander-ci/worktrees/$repo/$ref"
}

# Fetch new refs, then checkout the desired worktree to its canonical path
checkout_worktree() {
	repo=$1
	ref=$2

	GIT_DIR=".git" git -C $(full_repo_path $repo) fetch
	mkdir -p $(full_worktree_path $repo $ref)
	GIT_DIR=".git" git -C $(full_repo_path $repo) \
		--work-tree=$(full_worktree_path $repo $ref) \
		checkout $ref -- .
}

# Only checkout a worktree if we don't have a local copy
ensure_worktree_exists() {
	repo=$1
	ref=$2

	ensure_repo_exists $repo
	if [ ! -d $(full_worktree_path $repo $ref) ]; then checkout_worktree $repo $ref; fi
}

# Make sure a worktree for $repo's $ref is owned by $ci_user
worktree_for_user() {
	repo=$1
	ref=$2
	ci_user=$3

	ensure_worktree_exists $repo $ref
	chown -R $ci_user $(full_worktree_path $repo $ref)
}

# POST stdin (must be json) to the github status api
github_status_api() {
	repo=$1
	ref=$2
	token=$3

	>/dev/null curl --silent \
		-H "Accept: application/vnd.github.v3+json" \
		-H "Authorization: token ${token}" \
		-d @- \
		"https://api.github.com/repos/${repo}/statuses/${ref}"
}

# Construct a valid json object for the github status api
status_object() {
	repo=$1
	ref=$2
	state=$3

	echo '{"state": "STATE", "target_url": "URL", "context": "zoolander"}' \
		| sed s/STATE/$state/ \
		| sed s,URL,$(target_url $repo $ref),
}

# URL for the build output of $repo's $ref
target_url() {
	repo=$1
	ref=$2

	echo "http://54.211.12.29/zoolander/jobs/$ref"
}

# Update the GitHub status for $repo's $ref to be "$status". Use $token for auth
mark_status() {
	status=$1
	repo=$2
	ref=$3
	token=$4

	echo $(status_object $repo $ref $status) | github_status_api $repo $ref $token
}

# Call sudo only if we are changing users
doas() {
	user=$1
	shift

	if [ ! $(whoami) = "${user}" ]; then
		sudo -u $user -i $*
	else
		$*
	fi
}


# Test $repo's $ref as the $ci_user. Assumes worktree contains Zoolander.mk with
# a "test" target.
launch_job() {
	repo=$1
	ref=$2
	ci_user=$3

	worktree_for_user $repo $ref $ci_user
	doas $ci_user gmake -C $(full_worktree_path $repo $ref) -f Zoolander.mk test 
}


# main
ref=$1
repo=robertdfrench/zoolander-ci
ci_user=$(if_root_then_derek)
token=$(cat /tmp/ghpat)

mark_status "pending" $repo $ref $token
if launch_job $repo $ref $ci_user; then
	mark_status "success" $repo $ref $token
else
	mark_status "failure" $repo $ref $token
fi
