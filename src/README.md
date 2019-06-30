# Zoolander Application Source
Other than `main`, each module includes its own unit tests, organized in a
submodule named `tests`. This prevents us from having to maintain mirrored
file structures, which often need to replicate (or nearly replicate)
one-another's import statements.

Unit tests can be run from the parent directory via `make test`.
