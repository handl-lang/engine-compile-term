# ADR-0140: `compile-term` target

## Context

[ADR-0114](../0114-handl-compile-plugin-architecture.md) states the compile
port (`Compiler` / `CompilerRegistry`), and
[ADR-0100](../0100-canonical-term-arrows.md) made the `Term` representation
the canonical lowering target. Serialising that `Term` graph as JSON is one
compiler among several.

## Decision

`term` owns the Term-JSON compiler: `handl-plugin-compile-term` (target
`term`), which lowers canonical Handl sources to `Term` JSON — here as
`modules/compile-term`.

`host` keeps the port and the registry seam.

## Consequences

- `hnl compile term` goes through the registry; the target name is unchanged.
- This is the reference lowering that the other targets (`dot`, `sexp`,
  `handl`) read, so its `Term` shape is a cross-repo interface — changes to it
  are multi-repo changes plus a rev bump (ADR-0128 §Costs).

## Status

Accepted — extracted in ADR-0128 Phase 7.
