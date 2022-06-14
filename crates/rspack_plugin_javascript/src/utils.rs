use once_cell::sync::Lazy;
use rspack_core::SourceType;
use std::path::Path;
use std::sync::Arc;
use swc::{config::IsModule, Compiler as SwcCompiler};
use swc_common::{FileName, FilePathMapping, SourceMap};
use swc_ecma_parser::Syntax;
use swc_ecma_parser::{EsConfig, TsConfig};
use tracing::instrument;

static SWC_COMPILER: Lazy<Arc<SwcCompiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

  Arc::new(SwcCompiler::new(cm))
});

pub fn get_swc_compiler() -> Arc<SwcCompiler> {
  SWC_COMPILER.clone()
}

#[instrument(skip_all)]
pub fn parse_file(
  source_code: String,
  filename: &str,
  source_type: &SourceType,
) -> swc_ecma_ast::Program {
  let syntax = syntax_by_source_type(filename, source_type);
  let compiler = get_swc_compiler();
  let fm = compiler
    .cm
    .new_source_file(FileName::Custom(filename.to_string()), source_code);
  swc::try_with_handler(compiler.cm.clone(), Default::default(), |handler| {
    compiler.parse_js(
      fm,
      handler,
      swc_ecma_ast::EsVersion::Es2022,
      syntax,
      IsModule::Unknown,
      None,
    )
  })
  .unwrap()
}

pub fn syntax_by_ext(ext: &str) -> Syntax {
  match ext == "ts" || ext == "tsx" {
    true => Syntax::Typescript(TsConfig {
      decorators: false,
      tsx: ext == "tsx",
      ..Default::default()
    }),
    false => Syntax::Es(EsConfig {
      private_in_object: true,
      import_assertions: true,
      jsx: ext == "jsx",
      export_default_from: true,
      decorators_before_export: true,
      decorators: true,
      fn_bind: true,
      allow_super_outside_method: true,
    }),
  }
}

pub fn syntax_by_source_type(filename: &str, source_type: &SourceType) -> Syntax {
  match source_type {
    SourceType::Js | SourceType::Jsx => Syntax::Es(EsConfig {
      private_in_object: true,
      import_assertions: true,
      jsx: matches!(source_type, SourceType::Jsx),
      export_default_from: true,
      decorators_before_export: true,
      decorators: true,
      fn_bind: true,
      allow_super_outside_method: true,
    }),
    SourceType::Ts | SourceType::Tsx => Syntax::Typescript(TsConfig {
      decorators: false,
      tsx: matches!(source_type, SourceType::Tsx),
      ..Default::default()
    }),
    _ => {
      let ext = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("js");
      syntax_by_ext(ext)
    }
  }
}