# Zoolander CI
Zoolander CI is a minimal CI environment for illumos-based projects. It provides
a single VM which manages CI for a single GitHub project. If you want a real CI
system, try [Cirrus CI](https://cirrus-ci.org/).

This repository is capable of bootstrapping and deploying itself, assuming you
have valid AWS credentials and sufficient privileges to create and manage EC2
instance, images, and security groups. You will also need a github account.

Unit tests can be run via `make test`, integration tests can be launched
(together with a functioning illumos system!) via `make check`.

### Getting Started
* [Create a GitHub API Token](https://github.com/settings/tokens/new)
* [Configure your AWS Credentials file](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
* Install:
  * [Packer](https://www.packer.io/downloads.html)
  * [Terraform](https://www.terraform.io/downloads.html)
  * [jq](https://stedolan.github.io/jq/)
  * [GNU Make](https://www.gnu.org/software/make/) (probably already installed).

Each [Makefile](Makefile) in this repo is self-documenting. Just run `make help`
in any directory to describe the available tasks.
