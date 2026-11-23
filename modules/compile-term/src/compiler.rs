//! The `TermCompiler` implementation.

use handl_calculus::Term;
use handl_plugin_compile::{CompileError, Compiler, CompilerConfig, CompilerInput, CompilerOutput};
use hnl_tool_codegen::{ArtifactRef, Change, CodegenError, EmitContext, Emitter};

use crate::config::TermConfig;
use crate::output::TermOutput;

/// Serialize `term` as pretty JSON — the `term` backend's only
/// emission format (`json` and `pretty` are aliases).
fn render_term(term: &Term) -> Result<String, String> {
    serde_json::to_string_pretty(term).map_err(|e| e.to_string())
}

/// Term compiler: serializes the input's lowered `Term` as JSON.
///
/// Under the codegen kernel the `Term` is the IR — `compile` consumes
/// `input.term` directly rather than re-lowering the AST (PL-004 T-092).
#[derive(Debug, Default, Clone, Copy)]
pub struct TermCompiler;

impl Compiler for TermCompiler {
    fn compile(
        &self,
        input: &CompilerInput,
        config: &dyn CompilerConfig,
    ) -> Result<Box<dyn CompilerOutput>, CompileError> {
        let cfg = config
            .as_any()
            .downcast_ref::<TermConfig>()
            .ok_or_else(|| CompileError::InvalidInput("expected TermConfig".to_string()))?;

        match cfg.emit.as_str() {
            "json" | "pretty" => {}
            other => return Err(CompileError::UnsupportedTarget(other.to_string())),
        }

        let json = render_term(&input.term).map_err(CompileError::Emit)?;
        Ok(Box::new(TermOutput::new(json)))
    }

    fn target(&self) -> &str {
        "term"
    }
}

/// `Emitter<Term>` over the desired-state codegen kernel (PL-004 T-092):
/// the `Term` *is* the IR, so `materialize` serializes `ir` directly —
/// no re-lower. Emits the same `term.json` artifact as
/// [`TermCompiler::compile`]. The legacy `Compiler` impl stays until
/// `handl-plugin-compile` migrates to the kernel pipeline.
impl Emitter<Term> for TermCompiler {
    fn materialize(&self, ir: &Term, req: &EmitContext) -> Result<Vec<Change>, CodegenError> {
        match req.config.get("emit").map(String::as_str) {
            None | Some("json") | Some("pretty") => {}
            Some(other) => {
                return Err(CodegenError::Unsupported(format!(
                    "term emit format {other:?} (expected \"json\" or \"pretty\")"
                )));
            }
        }
        let json = render_term(ir).map_err(CodegenError::Emit)?;
        Ok(vec![Change::text(ArtifactRef::new("term.json"), json)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture() -> Term {
        Term::Seq {
            axis: Box::new(Term::constant(json!(0))),
            fst: Box::new(Term::op_native("id", vec![])),
            snd: Box::new(Term::op_native_lit("elem", vec![], json!(2))),
        }
    }

    #[test]
    fn materialize_emits_term_json_matching_render() {
        let term = fixture();
        let changes = TermCompiler
            .materialize(&term, &EmitContext::new())
            .expect("materialize");
        assert_eq!(changes.len(), 1);
        match &changes[0] {
            Change::Put {
                artifact, content, ..
            } => {
                assert_eq!(artifact.as_str(), "term.json");
                let text = String::from_utf8(content.clone()).unwrap();
                assert_eq!(text, render_term(&term).unwrap());
                // The artifact round-trips back to the same Term.
                let back: Term = serde_json::from_str(&text).unwrap();
                assert_eq!(back, term);
            }
            other => panic!("expected Put, got {other:?}"),
        }
    }

    #[test]
    fn materialize_rejects_unknown_emit_format() {
        let term = fixture();
        let req = EmitContext::new().with_config("emit", "xml");
        let err = TermCompiler
            .materialize(&term, &req)
            .expect_err("unknown emit format must decline");
        assert!(matches!(err, CodegenError::Unsupported(_)));
    }
}
