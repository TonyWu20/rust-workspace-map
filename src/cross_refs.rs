use crate::schema::{CrateInfo, CrossCrateImport, CrossReferences, TypeRef};
use std::collections::BTreeMap;

/// Compute cross-crate type references.
///
/// For every public item in every crate, matches it against imports from
/// other crates. Populates each `CrateInfo.cross_crate_imports` and returns
/// the global `CrossReferences` map (keyed by type/symbol name).
///
/// Takes `&mut [CrateInfo]` so it can write `cross_crate_imports` into each
/// crate while building the global cross-reference map.
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
    // Build a map: crate_name -> set of public item names.
    let crate_exports: BTreeMap<String, Vec<(String, String)>> = crates
        .iter()
        .map(|c| {
            let items: Vec<(String, String)> = c
                .modules
                .iter()
                .flat_map(|m| &m.public_items)
                .map(|item| (item.name.clone(), item.kind_to_string()))
                .collect();
            (c.name.clone(), items)
        })
        .collect();

    let mut types_map: BTreeMap<String, TypeRef> = BTreeMap::new();

    // Initialize TypeRef entries for every exported public item.
    for (crate_name, items) in &crate_exports {
        for (item_name, kind) in items {
            let entry = types_map.entry(item_name.clone()).or_insert_with(|| {
                TypeRef::builder()
                    .crate_name(crate_name.clone())
                    .kind(kind.clone())
                    .build()
            });
            if !entry.exported_by.contains(crate_name) {
                entry.exported_by.push(crate_name.clone());
            }
        }
    }

    // Scan each crate's imports to find cross-crate references.
    for crate_info in crates.iter_mut() {
        let my_name = crate_info.name.clone();
        let mut cross_imports: Vec<CrossCrateImport> = Vec::new();

        for module in &crate_info.modules {
            for import in &module.imports {
                // Extract first path segment as potential crate name.
                let first_seg = import
                    .path
                    .split("::")
                    .next()
                    .unwrap_or("")
                    .to_string();
                if first_seg.is_empty() || first_seg == "*" {
                    continue;
                }

                // Check if first segment matches any known crate.
                if crate_exports.contains_key(&first_seg) && first_seg != my_name {
                    let symbol = import
                        .path
                        .rsplit("::")
                        .next()
                        .unwrap_or("")
                        .to_string();

                    cross_imports.push(CrossCrateImport {
                        import_path: import.path.clone(),
                        target_crate: first_seg.clone(),
                        symbol: symbol.clone(),
                        line: import.line,
                    });

                    // Update global cross-references.
                    if let Some(type_ref) = types_map.get_mut(&symbol) {
                        let importer_label = format!("{}:{}", my_name, module.path);
                        if !type_ref.imported_by.contains(&importer_label) {
                            type_ref.imported_by.push(importer_label.clone());
                        }
                    }
                }
            }
        }

        cross_imports.sort_by(|a, b| a.import_path.cmp(&b.import_path));
        crate_info.cross_crate_imports = cross_imports;
    }

    CrossReferences { types: types_map }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl};

    fn make_crate(name: &str, items: Vec<(String, ItemKind)>) -> CrateInfo {
        let public_items: Vec<PublicItem> = items
            .into_iter()
            .map(|(n, k)| {
                PublicItem::builder()
                    .kind(k)
                    .name(n)
                    .file(String::new())
                    .line(1)
                    .visibility("pub".to_string())
                    .generics(String::new())
                    .attrs(Default::default())
                    .build()
            })
            .collect();
        let module = ModuleInfo::builder()
            .path("".to_string())
            .file(String::new())
            .visibility("pub".to_string())
            .public_items(public_items)
            .build();
        CrateInfo::builder()
            .name(name.to_string())
            .root(String::new())
            .package(
                schema::PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(schema::CrateType::Lib)
                    .build(),
            )
            .modules(vec![module])
            .deps(Default::default())
            .build()
    }

    #[test]
    fn compute_finds_cross_crate_import() {
        let mut crates = vec![
            make_crate("core", vec![
                ("Task".to_string(), ItemKind::Struct),
            ]),
            make_crate("engine", vec![]),
        ];
        // Manually add an import in engine that references core::Task
        let engine_module = &mut crates[1].modules[0];
        engine_module.imports.push(Import {
            path: "core::Task".to_string(),
            line: 1,
        });
        let refs = compute(&mut crates);
        // Task should be in cross-references
        assert!(refs.types.contains_key("Task"));
        let task_ref = &refs.types["Task"];
        assert_eq!(task_ref.crate_name, "core");
        // engine should have a cross_crate_import
        assert_eq!(crates[1].cross_crate_imports.len(), 1);
        assert_eq!(crates[1].cross_crate_imports[0].target_crate, "core");
    }

    #[test]
    fn compute_empty_for_no_cross_references() {
        let crates = vec![
            make_crate("a", vec![("Foo".to_string(), ItemKind::Struct)]),
            make_crate("b", vec![("Bar".to_string(), ItemKind::Struct)]),
        ];
        let mut crates_mut = crates;
        let refs = compute(&mut crates_mut);
        // No cross references since no crate imports from another
        assert!(refs.types.is_empty() || refs.types.values().all(|t| t.imported_by.is_empty()));
    }
}

// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    #[must_use]
    fn kind_to_string(&self) -> String {
        match self.kind {
            crate::schema::ItemKind::Struct => "struct".to_string(),
            crate::schema::ItemKind::Enum => "enum".to_string(),
            crate::schema::ItemKind::Trait => "trait".to_string(),
            crate::schema::ItemKind::Fn => "fn".to_string(),
            crate::schema::ItemKind::Type => "type".to_string(),
            crate::schema::ItemKind::Macro => "macro".to_string(),
        }
    }
}
