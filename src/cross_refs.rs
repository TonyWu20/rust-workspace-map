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

// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
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
