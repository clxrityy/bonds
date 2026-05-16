# Bonds | Contributing | Testing

Before opening a pull request, run the checks that match the surface you changed.

## Common workspace commands

For most Rust changes, these are the first commands to reach for:

```bash
make test
make lint
make build
make pre-commit
```

## Targeted commands

If you only need a narrower pass, useful commands include:

```bash
make test-core
make test-cli
make test-docs
make docs
```

## Desktop app checks

If you changed anything under `crates/app`, also run:

```bash
cd crates/app && pnpm lint
cd crates/app && pnpm build
```

## CI-parity Rust checks

The CI workflow validates the Rust workspace with commands equivalent to:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
```

If your contribution touches Rust code, running these before opening a PR is strongly recommended.

## What to test

As a rule of thumb:

- Rust logic changes: run workspace build, lint, and tests
- CLI changes: run CLI tests and verify command behavior manually if needed
- docs changes: run `make test-docs` or `make docs`
- app changes: run app lint/build and include screenshots when helpful

## Testing expectations

You do not need to add a new test for every typo fix or text-only docs change.

You should usually add or update tests when you:

- fix a bug
- change command behavior
- change metadata or query behavior
- change lifecycle or event behavior
- introduce new user-facing functionality

When in doubt, lean toward adding coverage. Future-you will appreciate the paper trail.
