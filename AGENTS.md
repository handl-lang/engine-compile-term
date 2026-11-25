# Agent guide — tool-compile-term

Term JSON compiler for canonical Handl sources — `tool-compile-*`
compiler-backend family member: lowers canonical Handl sources to the
`Term` arrow IR and emits its JSON form. Rust 2024.

## Layout

| path | role |
| --- | --- |
| `modules/compile-term/` | the crate — `handl-plugin-compile-term` (`src/lib.rs`, `compiler.rs`, `config.rs`, `output.rs`) |
| `xtask/` | vendored repo gate |
| `workflows/gate.yaml`, `bupha.toml`, `.cargo/` | CI wiring + `cargo xtask` alias |

## Contract

- Virtual-workspace root `Cargo.toml` (`members = ["modules/compile-term"]`);
  `[patch]` tables map engine/kernel/host/tool-codegen git deps to
  sibling checkouts — including `handl-kit-arrow-lower`
  (`handl-lang/host`) for the arrow lowering.
- Output is the hnl_codec Term JSON wire (`{"Op"/"Arr"/"Const"}` /
  `dag.program`) consumed by the `tool-compile-*` emitters.

## Verification

```bash
cargo xtask gate   # run at the workspace root
```


<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
