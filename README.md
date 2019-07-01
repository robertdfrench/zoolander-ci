# Zoolander CI
Zoolander CI is a minimal CI environment for illumos-based projects. It provides
a single VM which manages CI for a single GitHub project. If you want a real CI
system, try [Cirrus CI](https://cirrus-ci.org/).

This repository is capable of bootstrapping and deploying itself, assuming you
have valid AWS credentials and sufficient privileges to create and manage EC2
instance, images, and security groups. You will also need a github account.

Unit tests can be run via `make test`, integration tests can be launched
(together with a functioning illumos system!) via `make check`.

See the [Makefile](Makefile) for more details.
