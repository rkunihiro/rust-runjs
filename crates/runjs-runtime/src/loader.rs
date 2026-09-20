use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::error::ModuleLoaderError;
use deno_core::resolve_import;
use deno_core::ModuleLoadOptions;
use deno_core::ModuleLoadReferrer;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use deno_core::ModuleSourceCode;
use deno_core::ModuleSpecifier;
use deno_core::ModuleType;
use deno_core::ResolutionKind;
use deno_error::JsErrorBox;

/// Loads ES modules from the local filesystem only, transpiling TypeScript
/// (type-stripping only, no type-checking) via `deno_ast` when needed.
///
/// Bare specifiers (npm-style) and remote (http/https) imports are rejected —
/// out of scope for v1.
#[derive(Default)]
pub struct RunjsModuleLoader;

impl ModuleLoader for RunjsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        resolve_import(specifier, referrer).map_err(JsErrorBox::from_err)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleLoadReferrer>,
        _options: ModuleLoadOptions,
    ) -> ModuleLoadResponse {
        ModuleLoadResponse::Sync(load_module(module_specifier))
    }
}

fn load_module(
    module_specifier: &ModuleSpecifier,
) -> Result<ModuleSource, ModuleLoaderError> {
    let path = module_specifier.to_file_path().map_err(|_| {
        JsErrorBox::generic("Only local file:// module specifiers are supported.")
    })?;

    let media_type = MediaType::from_path(&path);
    let (module_type, should_transpile) = match media_type {
        MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
            (ModuleType::JavaScript, false)
        }
        MediaType::Jsx => (ModuleType::JavaScript, true),
        MediaType::TypeScript | MediaType::Mts | MediaType::Cts | MediaType::Tsx => {
            (ModuleType::JavaScript, true)
        }
        MediaType::Json => (ModuleType::Json, false),
        _ => {
            return Err(JsErrorBox::generic(format!(
                "Unsupported module extension for '{}'",
                path.display()
            )));
        }
    };

    let code = std::fs::read_to_string(&path).map_err(JsErrorBox::from_err)?;

    let code = if should_transpile {
        let parsed = deno_ast::parse_module(ParseParams {
            specifier: module_specifier.clone(),
            text: code.into(),
            media_type,
            capture_tokens: false,
            scope_analysis: false,
            maybe_syntax: None,
        })
        .map_err(JsErrorBox::from_err)?;

        let transpiled = parsed
            .transpile(
                &deno_ast::TranspileOptions::default(),
                &deno_ast::TranspileModuleOptions::default(),
                &deno_ast::EmitOptions {
                    source_map: SourceMapOption::None,
                    ..Default::default()
                },
            )
            .map_err(JsErrorBox::from_err)?;

        transpiled.into_source().text
    } else {
        code
    };

    Ok(ModuleSource::new(
        module_type,
        ModuleSourceCode::String(code.into()),
        module_specifier,
        None,
    ))
}
