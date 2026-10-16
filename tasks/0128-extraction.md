# Task 0128 — `term/` extraction (ADR-0128 Phase 7)

Expand ADR-0128 Phase 7 for this repo. The docs are already on
this `project` branch; the code lands with the extraction.

## Docs owned here

(none — `term: {}` in platform's `docs-split.yaml`)

## Extraction

- [x] Follow ADR-0128 Phase 7: create the repo's workspace,
      move the crates/packages named there, wire the git dep + `[patch]`.
      **Done:** `modules/compile-term` (subtree-split history of
      `crates/plugin-compile-term`, package name unchanged);
      `plugin-ext-compile` consumes it via a pinned git dep.
- [x] Keep `main` a clean build tree (`/project/` ignored).
