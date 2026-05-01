use crate::schema::{
    CrateInfo, CanonicalPath, FileEntry, SymbolEntry, WorkspaceRelativePath,
};
use std::collections::BTreeMap;

type SymbolsIndex = BTreeMap<CanonicalPath, SymbolEntry>;
type NameIndex = BTreeMap<String, Vec<CanonicalPath>>;
type FilesIndex = BTreeMap<WorkspaceRelativePath, FileEntry>;

/// Build flat indexes from a slice of `CrateInfo`.
///
/// Returns three maps:
/// 1. `symbols` — canonical-path-keyed map of all public items
/// 2. `name_index` — short-name to canonical-path list (for disambiguation)
/// 3. `files` — workspace-relative-path-keyed map of all source files with module metadata
///
/// The symbols and `name_index` are constructed via an iterator pipeline;
/// the files map uses a for-loop due to the inline-detection accumulator.
///
/// # Inline module detection
///
/// This function relies on depth-first traversal order from `build_module_tree`.
/// The heuristic: the first module encountered per file path is the primary
/// (declared in a separate file); subsequent modules sharing the same file
/// are inline modules and are excluded from the files map.
#[allow(clippy::type_complexity)]
#[must_use]
pub fn derive_from_crates(
    crates: &[CrateInfo],
) -> (SymbolsIndex, NameIndex, FilesIndex) {
    // ── symbols & name_index via iterator pipeline ────────────────────

    let (syms, mut nidx): (
        BTreeMap<CanonicalPath, SymbolEntry>,
        BTreeMap<String, Vec<CanonicalPath>>,
    ) = crates
        .iter()
        .flat_map(|c| {
            c.modules
                .iter()
                .flat_map(move |m| {
                    m.public_items
                        .iter()
                        .map(move |item| (c.name.as_str(), m, item))
                })
        })
        .fold(
            (
                BTreeMap::<CanonicalPath, SymbolEntry>::new(),
                BTreeMap::<String, Vec<CanonicalPath>>::new(),
            ),
            |(mut syms, mut nidx), (crate_name, m, item)| {
                let canonical = CanonicalPath::from(format!("{}::{}", m.path, item.name));
                syms.insert(
                    canonical.clone(),
                    SymbolEntry::builder()
                        .crate_name(crate_name.to_string())
                        .module(m.path.clone())
                        .file(m.file.clone())
                        .line(item.line)
                        .kind(item.kind.clone())
                        .derive_attrs(item.attrs.derive.clone())
                        .build(),
                );
                nidx.entry(item.name.clone()).or_default().push(canonical);
                (syms, nidx)
            },
        );

    // Sort name_index values for determinism.
    for val in nidx.values_mut() {
        val.sort();
    }

    // ── files via for-loop (inline detection requires accumulator) ───

    let mut files = BTreeMap::new();
    let mut seen_files: BTreeMap<String, String> = BTreeMap::new(); // file -> module_path of first owner

    for crate_info in crates {
        for module in &crate_info.modules {
            if module.file == "<unresolved>" {
                continue;
            }

            let file = &module.file;
            if seen_files.contains_key(file) {
                // Inline module — shares file with an earlier (parent) module.
                continue;
            }
            seen_files.insert(file.clone(), module.path.clone());

            let is_crate_root = module.path == crate_info.name;

            // Compute parent_module_file by walking the crate's modules.
            let parent_module_file = if module.path.is_empty() {
                None // Root module has no parent.
            } else {
                // Strip last ::name segment to get parent path.
                if let Some(pos) = module.path.rfind("::") {
                    let parent_path = &module.path[..pos];
                    // Find the module entry whose path matches the parent.
                    crate_info
                        .modules
                        .iter()
                        .find(|m| m.path == parent_path)
                        .map(|m| m.file.clone())
                } else {
                    None
                }
            };

            let files_entry = match parent_module_file {
                Some(pf) => FileEntry::builder()
                    .module_path(module.path.clone())
                    .parent_module_file(pf)
                    .is_crate_root(is_crate_root)
                    .build(),
                None => FileEntry::builder()
                    .module_path(module.path.clone())
                    .is_crate_root(is_crate_root)
                    .build(),
            };
            files.insert(
                WorkspaceRelativePath(file.clone()),
                files_entry,
            );
        }
    }

    (syms, nidx, files)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateType, DepInfo, ItemAttrs, ItemKind, ModuleInfo,
        PackageInfo, PublicItem,
    };

    fn make_crate(
        name: &str,
        modules: Vec<ModuleInfo>,
    ) -> CrateInfo {
        CrateInfo::builder()
            .name(name.to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(modules)
            .deps(DepInfo::default())
            .build()
    }

    fn make_module(path: &str, file: &str, items: Vec<PublicItem>) -> ModuleInfo {
        ModuleInfo::builder()
            .path(path.to_string())
            .file(file.to_string())
            .visibility("pub".to_string())
            .public_items(items)
            .build()
    }

    fn make_item(name: &str, kind: ItemKind, line: usize) -> PublicItem {
        PublicItem::builder()
            .name(name.to_string())
            .kind(kind)
            .file("src/lib.rs".to_string())
            .line(line)
            .visibility("pub".to_string())
            .generics(String::new())
            .attrs(ItemAttrs::default())
            .build()
    }

    #[test]
    fn symbol_index_single_root_item() {
        let module = make_module(
            "mycrate",
            "src/lib.rs",
            vec![make_item("MyStruct", ItemKind::Struct, 1)],
        );
        let crates = vec![make_crate("mycrate", vec![module])];
        let (syms, _, _) = derive_from_crates(&crates);

        let key = CanonicalPath::from("mycrate::MyStruct".to_string());
        let entry = syms.get(&key).expect("expected mycrate::MyStruct");
        assert_eq!(entry.crate_name, "mycrate");
        assert_eq!(entry.module, "mycrate");
        assert_eq!(entry.kind, ItemKind::Struct);
    }

    #[test]
    fn symbol_index_nested_module_item() {
        let root = make_module("mycrate", "src/lib.rs", vec![]);
        let sub = make_module(
            "mycrate::sub",
            "src/sub.rs",
            vec![make_item("Helper", ItemKind::Fn, 5)],
        );
        let crates = vec![make_crate("mycrate", vec![root, sub])];
        let (syms, _, _) = derive_from_crates(&crates);

        let key = CanonicalPath::from("mycrate::sub::Helper".to_string());
        let entry = syms.get(&key).expect("expected mycrate::sub::Helper");
        assert_eq!(entry.module, "mycrate::sub");
        assert_eq!(entry.file, "src/sub.rs");
    }

    #[test]
    fn name_index_multi_crate_collision() {
        let root_a = make_module(
            "alpha",
            "src/lib.rs",
            vec![make_item("Foo", ItemKind::Struct, 1)],
        );
        let root_b = make_module(
            "beta",
            "src/lib.rs",
            vec![make_item("Foo", ItemKind::Fn, 10)],
        );
        let crates = vec![make_crate("alpha", vec![root_a]), make_crate("beta", vec![root_b])];
        let (_, nidx, _) = derive_from_crates(&crates);

        let candidates = nidx.get("Foo").expect("expected Foo in name_index");
        assert_eq!(candidates.len(), 2);
        assert!(candidates.iter().any(|p| p.as_ref() == "alpha::Foo"));
        assert!(candidates.iter().any(|p| p.as_ref() == "beta::Foo"));
    }

    #[test]
    fn files_excludes_inline_module() {
        // Root module in lib.rs, inline sub in lib.rs (same file).
        let root = make_module(
            "mycrate",
            "src/lib.rs",
            vec![make_item("Item", ItemKind::Struct, 1)],
        );
        let inline = make_module(
            "mycrate::inner",
            "src/lib.rs", // same file as root
            vec![make_item("InnerItem", ItemKind::Struct, 5)],
        );
        let crates = vec![make_crate("mycrate", vec![root, inline])];
        let (_, _, files) = derive_from_crates(&crates);

        // Only one file entry should exist (the root).
        assert_eq!(files.len(), 1);
        let key = WorkspaceRelativePath::from("src/lib.rs".to_string());
        let entry = files.get(&key).expect("expected src/lib.rs in files");
        assert!(entry.is_crate_root);
    }

    #[test]
    fn files_parent_module_file() {
        let root = make_module("mycrate", "src/lib.rs", vec![]);
        let sub = make_module(
            "mycrate::sub",
            "src/sub.rs",
            vec![make_item("X", ItemKind::Struct, 1)],
        );
        let crates = vec![make_crate("mycrate", vec![root, sub])];
        let (_, _, files) = derive_from_crates(&crates);

        let root_key = WorkspaceRelativePath::from("src/lib.rs".to_string());
        let root_entry = files.get(&root_key).expect("expected root");
        assert!(root_entry.parent_module_file.is_none(), "root should have no parent");

        let sub_key = WorkspaceRelativePath::from("src/sub.rs".to_string());
        let sub_entry = files.get(&sub_key).expect("expected sub");
        assert_eq!(
            sub_entry.parent_module_file,
            Some("src/lib.rs".to_string()),
            "sub should have lib.rs as parent"
        );
    }
}
