# Zoolander Application Source
Each module includes its own unit tests, organized in a submodule named `tests`.
This prevents us from having to maintain mirrored file structures, which often
need to replicate (or nearly replicate) one-another's import statements.

[main.rs](main.rs) is special in that it is the only module allowed to include
3rd party libraries. Its role is to wrap those libraries as thinly as possible
and inject them into the other modules (this is a rough analogue of Dependency
Injection, as it applies to Rust). Its tests are therefore integration tests.

Unit and integration tests can be run from the parent directory via `make test`.
