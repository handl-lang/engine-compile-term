# tool-compile-term

Term JSON compiler for canonical Handl sources — part of the `tool-compile-*`
compiler-backend family: lowers canonical Handl sources to the `Term`
arrow IR and emits its JSON form.

The crate lives in `modules/compile-term` inside this workspace.

## Development

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

`xtask/` provides the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `workflows/gate.yaml` is the gate entry. Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.
