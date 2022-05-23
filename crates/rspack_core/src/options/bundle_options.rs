use crate::{CodeSplittingOptions, LoaderOptions};
use std::collections::HashMap;

use rspack_swc::swc_ecma_transforms_react;

#[derive(Debug)]
pub struct BundleReactOptions {
  pub runtime: swc_ecma_transforms_react::Runtime,
  pub refresh: bool,
}

impl Default for BundleReactOptions {
  fn default() -> Self {
    Self {
      runtime: swc_ecma_transforms_react::Runtime::Automatic,
      refresh: false,
    }
  }
}

#[derive(Debug, Clone)]
pub enum BundleMode {
  Dev,
  Prod,
  None,
}

#[derive(Debug, Clone)]
pub struct ResolveOption {
  pub extensions: Vec<String>,
  pub alias: Vec<(String, Option<String>)>,
}

impl Default for ResolveOption {
  fn default() -> Self {
    Self {
      extensions: vec![".tsx", ".jsx", ".ts", ".js", ".json"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
      alias: vec![],
    }
  }
}

#[derive(Debug)]
pub struct BundleOptions {
  pub react: BundleReactOptions,
  pub loader: LoaderOptions,
  pub mode: BundleMode,
  pub entries: Vec<String>,
  pub minify: bool,
  pub outdir: String,
  pub entry_filename: String, // | ((chunkInfo: PreRenderedChunk) => string)
  pub chunk_filename: String,
  pub code_splitting: Option<CodeSplittingOptions>,
  pub lazy_compilation: bool,
  pub root: String,
  pub inline_style: bool,
  pub resolve: ResolveOption,
  pub source_map: bool,
  pub svgr: bool,
  pub define: HashMap<String, String>,
}

impl Default for BundleOptions {
  fn default() -> Self {
    Self {
      resolve: Default::default(),
      react: Default::default(),
      root: std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string(),
      mode: BundleMode::Prod,
      entries: Default::default(),
      // format: InternalModuleFormat::ES,
      outdir: std::env::current_dir()
        .unwrap()
        .join("./dist")
        .to_string_lossy()
        .to_string(),
      minify: Default::default(),
      entry_filename: "[name].js".to_string(),
      chunk_filename: "chunk-[name].js".to_string(),
      code_splitting: Some(Default::default()),
      lazy_compilation: false,
      loader: Default::default(),
      inline_style: Default::default(),
      source_map: true,
      svgr: false,
      define: Default::default(),
    }
  }
}

pub type NormalizedBundleOptions = BundleOptions;