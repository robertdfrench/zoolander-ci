# Zoolander Application Source
Other than `main`, each module includes its own unit tests, organized in a
submodule named `tests`. This prevents us from having to maintain mirrored
file structures, which often need to replicate (or nearly replicate)
one-another's import statements.

The other exception to this is `lib.rs`, which serves as a layer of indirection
to make various constructs available to both `main` and the
[integration tests][1]. This allows us to compose `main` in the same way that we
might compose a rather elaborate integration test. This adheres to the value of
minimizing the difference between "testing" the software and "using" the
software, thus improving the fidelity with which the tests reinforce the
behavior we'd like to see in production.

Unit and integration tests can be run from the parent directory via `make test`.

[1]: ../tests
