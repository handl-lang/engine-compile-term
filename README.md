# compile-term

Term JSON compiler for canonical Handl sources — part of the `compile-*`
compiler-backend family: lowers canonical Handl sources to the `Term`
arrow IR and emits its JSON form.

The crate lives in `modules/compile-term` inside this workspace.

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.
