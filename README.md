# Zoolander CI
Zoolander CI is a minimalist CI environment for illumos-based projects, or
cross-platform projects that can reasonably be tested on illumos.

This repository is capable of bootstrapping and deploying itself, assuming you
have valid AWS credentials and sufficient privileges to create and manage EC2
instance, images, and security groups.

Unit tests can be run via `make test`, integration tests can be launched
(together with a functioning illumos system!) via `make check`.

See the [Makefile](Makefile) for more details.
