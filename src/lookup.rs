use crate::schema::{
    FileEntry, ItemKind, ModuleInfo, SymbolEntry, WorkspaceMap,
    WorkspaceRelativePath,
};

/// Result of a symbol lookup.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status")]
pub enum SymbolLookupResult {
    #[serde(rename = "found")]
    Found(SymbolEntry),
    #[serde(rename = "ambiguous")]
    Ambiguous {
        name: String,
        candidates: Vec<DisambiguationHint>,
    },
    #[serde(rename = "not_found")]
    NotFound,
}

/// Hint for disambiguating a symbol name collision.
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DisambiguationHint {
    pub canonical_path: String,
    pub crate_name: String,
    pub kind: ItemKind,
    pub file: String,
    pub line: usize,
}

/// Result of a file lookup.
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileLookupResult {
    pub file_entry: FileEntry,
    pub primary_module: ModuleInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inline_modules: Vec<ModuleInfo>,
}

/// Look up a symbol by name in the workspace map's name_index.
///
/// Returns `Found` if exactly one canonical path is found,
/// `Ambiguous` if multiple candidates exist, or `NotFound` otherwise.
#[must_use]
pub fn lookup_symbol(map: &WorkspaceMap, name: &str) -> SymbolLookupResult {
    let candidates = match map.name_index.get(name) {
        Some(candidates) => candidates,
        None => return SymbolLookupResult::NotFound,
    };

    if candidates.len() == 1 {
        let canonical = &candidates[0];
        if let Some(entry) = map.symbols.get(canonical) {
            return SymbolLookupResult::Found(entry.clone());
        }
    }

    if candidates.len() > 1 {
        let mut hints = Vec::new();
        for canonical in candidates {
            if let Some(entry) = map.symbols.get(canonical) {
                hints.push(
                    DisambiguationHint::builder()
                        .canonical_path(canonical.0.clone())
                        .crate_name(entry.crate_name.clone())
                        .kind(entry.kind.clone())
                        .file(entry.file.clone())
                        .line(entry.line)
                        .build(),
                );
            }
        }
        return SymbolLookupResult::Ambiguous {
            name: name.to_string(),
            candidates: hints,
        };
    }

    SymbolLookupResult::NotFound
}

/// Look up a file by workspace-relative path.
///
/// Returns `Some` with the file entry and associated modules if found,
/// or `None` if the file is not in the files index.
#[must_use]
pub fn lookup_file(map: &WorkspaceMap, file: &str) -> Option<FileLookupResult> {
    let key = WorkspaceRelativePath(file.to_string());
    let file_entry = map.files.get(&key)?;

    let mut primary_module: Option<ModuleInfo> = None;
    let mut inline_modules: Vec<ModuleInfo> = Vec::new();

    for crate_info in &map.crates {
        for module in &crate_info.modules {
            if module.file != file {
                continue;
            }
            if module.path == file_entry.module_path {
                primary_module = Some(module.clone());
            } else {
                inline_modules.push(module.clone());
            }
        }
    }

    let primary_module = primary_module?;

    Some(FileLookupResult::builder()
        .file_entry(file_entry.clone())
        .primary_module(primary_module)
        .inline_modules(inline_modules)
        .build())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ItemAttrs, ModuleInfo,
        PackageInfo, PublicItem, WorkspaceInfo,
    };

   fn make_test_map() -> WorkspaceMap {
        let item = PublicItem::builder()
            .name("Task".to_string())
            .kind(ItemKind::Struct)
            .file("core/src/lib.rs".to_string())
            .line(1)
            .visibility("pub".to_string())
            .generics(String::new())
            .attrs(ItemAttrs::default())
            .build();
        let module = ModuleInfo::builder()
            .path("core".to_string())
            .file("core/src/lib.rs".to_string())
            .visibility("pub".to_string())
            .public_items(vec![item])
            .build();
        let crate_info = CrateInfo::builder()
            .name("core".to_string())
            .root("core".to_string())
            .package(PackageInfo::builder()
                .name("core".to_string())
                .version("0.1.0".to_string())
                .edition("2021".to_string())
                .crate_type(CrateType::Lib)
                .build())
            .modules(vec![module])
            .deps(DepInfo::default())
            .build();

        let (symbols, name_index, files) = crate::indexes::derive_from_crates(&[crate_info.clone()]);

        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root("/tmp/test".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![crate_info])
            .cross_references(CrossReferences::default())
            .symbols(symbols)
            .name_index(name_index)
            .files(files)
            .workspace_root(std::path::PathBuf::from("/tmp/test"))
            .build()
    }

    #[test]
    fn lookup_symbol_found() {
        let map = make_test_map();
        let result = lookup_symbol(&map, "Task");
        match result {
            SymbolLookupResult::Found(entry) => {
                assert_eq!(entry.crate_name, "core");
                assert_eq!(entry.kind, ItemKind::Struct);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn lookup_symbol_not_found() {
        let map = make_test_map();
        let result = lookup_symbol(&map, "DoesNotExist");
        assert!(matches!(result, SymbolLookupResult::NotFound));
    }

    #[test]
    fn lookup_file_found() {
        let map = make_test_map();
        let result = lookup_file(&map, "core/src/lib.rs");
        assert!(result.is_some(), "expected file to be found");
    }

    #[test]
    fn lookup_file_not_found() {
        let map = make_test_map();
        let result = lookup_file(&map, "nonexistent.rs");
        assert!(result.is_none());
    }
}
