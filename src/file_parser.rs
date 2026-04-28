use crate::schema::{
    Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
    ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
};
use std::path::Path;

// ── Internal parse result types ────────────────────────────────────────

/// Result of parsing a Rust source file.
///
/// Unlike `Result<T, Error>`, this type always succeeds — parse
/// failures are reported as data, not as errors, so the caller
/// can continue processing other files. The caller constructs
/// `ErrorEntry` values from `SynParseError` when needed.
pub struct ParsedFile {
    pub ast: syn::File,
    pub file_info: FileInfo,
    pub parse_error: Option<SynParseError>,
}

/// Structured information about a parse failure.
pub struct SynParseError {
    pub message: String,
    pub line: usize,
}

// ── parse_file ──────────────────────────────────────────────────────────

/// Read and parse a Rust source file.
///
/// On parse failure, returns the original file content and a
/// `SynParseError` alongside an empty `FileInfo`. Callers use the
/// error to construct an `ErrorEntry`.
pub fn parse_file(path: &Path) -> ParsedFile {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(source) => {
            let err = SynParseError {
                message: source.to_string(),
                line: 0,
            };
            return ParsedFile {
                ast: syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![],
                },
                file_info: FileInfo::default(),
                parse_error: Some(err),
            };
        }
    };

    match syn::parse_file(&content) {
        Ok(file) => {
            let file_info = FileInfo {
                public_items: extract_public_items(&file.items),
                imports: extract_imports(&file.items),
                re_exports: extract_re_exports(&file.items),
                submodules: extract_submodules(&file.items),
                impls: extract_impls(&file.items),
            };
            ParsedFile {
                ast: file,
                file_info,
                parse_error: None,
            }
        },
        Err(e) => {
            let line = e.span().start().line;
            let err = SynParseError {
                message: e.to_string(),
                line,
            };
            ParsedFile {
                ast: syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![],
                },
                file_info: FileInfo::default(),
                parse_error: Some(err),
            }
        }
    }
}

pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
    ErrorEntry::builder()
        .file(path.to_string_lossy().to_string())
        .line(err.line)
        .message(err.message.clone())
        .severity(ErrorSeverity::Error)
        .kind("syn_parse_error".to_string())
        .build()
}

// ── extract_public_items ────────────────────────────────────────────────

/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {
    let mut result: Vec<PublicItem> = items
        .iter()
        .filter(|item| !is_visibility_inherited(item))
        .filter_map(into_public_item)
        .collect();
    result.sort_by(|a, b| a.name.cmp(&b.name).then(a.line.cmp(&b.line)));
    result
}

// ── extract_imports ─────────────────────────────────────────────────────

/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {
    let mut result: Vec<Import> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))
            } else {
                None
            }
        })
        .flatten()
        .collect();
    result.sort_by(|a, b| a.path.cmp(&b.path));
    result
}

// ── extract_re_exports ──────────────────────────────────────────────────

/// Extract `pub use` re-exports. Results sorted by export_path.
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {
    let mut result: Vec<ReExport> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                if matches!(u.vis, syn::Visibility::Public(_)) {
                    Some(extract_re_exports_from_tree(
                        &u.tree,
                        String::new(),
                        line_of_item(item),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();
    result.sort_by(|a, b| a.export_path.cmp(&b.export_path));
    result
}

// ── extract_submodules ──────────────────────────────────────────────────

/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {
    let mut result: Vec<SubmoduleDecl> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Mod(m) = item {
                let is_test = m.attrs.iter().any(|attr| {
                    if !attr.path().is_ident("cfg") {
                        return false;
                    }
                    if let syn::Meta::List(list) = &attr.meta {
                        let tokens = list.tokens.to_string();
                        tokens.trim() == "test"
                    } else {
                        false
                    }
                });
                Some(SubmoduleDecl {
                    name: m.ident.to_string(),
                    is_test,
                })
            } else {
                None
            }
        })
        .collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

// ── extract_impls ───────────────────────────────────────────────────────

/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {
    items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Impl(imp) = item {
                let type_name = match imp.self_ty.as_ref() {
                    syn::Type::Path(tp) => tp
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                if type_name.is_empty() {
                    return None;
                }
                let impl_items: Vec<ImplItem> = imp
                    .items
                    .iter()
                    .filter_map(|ii| match ii {
                        syn::ImplItem::Fn(f) => Some(ImplItem {
                            kind: ImplItemKind::Fn,
                            name: f.sig.ident.to_string(),
                            params: generics_to_string(&f.sig.generics),
                        }),
                        syn::ImplItem::Type(t) => Some(ImplItem {
                            kind: ImplItemKind::Type,
                            name: t.ident.to_string(),
                            params: String::new(),
                        }),
                        syn::ImplItem::Const(c) => Some(ImplItem {
                            kind: ImplItemKind::Const,
                            name: c.ident.to_string(),
                            params: String::new(),
                        }),
                        _ => None,
                    })
                    .collect();
                Some(ImplInfo {
                    type_: type_name,
                    items: impl_items,
                })
            } else {
                None
            }
        })
        .collect()
}

// ── Helpers ─────────────────────────────────────────────────────────────

fn is_visibility_inherited(item: &syn::Item) -> bool {
    matches!(item_vis(item), syn::Visibility::Inherited)
}

fn item_vis(item: &syn::Item) -> &syn::Visibility {
    match item {
        syn::Item::Const(i) => &i.vis,
        syn::Item::Enum(i) => &i.vis,
        syn::Item::ExternCrate(i) => &i.vis,
        syn::Item::Fn(i) => &i.vis,
        syn::Item::Mod(i) => &i.vis,
        syn::Item::Static(i) => &i.vis,
        syn::Item::Struct(i) => &i.vis,
        syn::Item::Trait(i) => &i.vis,
        syn::Item::TraitAlias(i) => &i.vis,
        syn::Item::Type(i) => &i.vis,
        syn::Item::Union(i) => &i.vis,
        syn::Item::Use(i) => &i.vis,
        _ => &syn::Visibility::Inherited,
    }
}

fn line_of_item(item: &syn::Item) -> usize {
    item_ident_span(item).unwrap_or(0)
}

fn item_ident_span(item: &syn::Item) -> Option<usize> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.span().start().line),
        syn::Item::Enum(e) => Some(e.ident.span().start().line),
        syn::Item::Trait(t) => Some(t.ident.span().start().line),
        syn::Item::Fn(f) => Some(f.sig.ident.span().start().line),
        syn::Item::Type(t) => Some(t.ident.span().start().line),
        syn::Item::Mod(m) => Some(m.ident.span().start().line),
        syn::Item::Macro(m) => m.ident.as_ref().map(|i| i.span().start().line),
        _ => None,
    }
}

fn vis_to_string(vis: &syn::Visibility) -> String {
    match vis {
        syn::Visibility::Public(_) => "pub".to_string(),
        syn::Visibility::Restricted(r) => {
            let path = r
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            if path.is_empty() {
                "pub(restricted)".to_string()
            } else {
                format!("pub({})", path)
            }
        }
        syn::Visibility::Inherited => "private".to_string(),
    }
}

fn generics_to_string(generics: &syn::Generics) -> String {
    if generics.params.is_empty() {
        return String::new();
    }
    let params: Vec<String> = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(t) => t.ident.to_string(),
            syn::GenericParam::Lifetime(l) => l.lifetime.ident.to_string(),
            syn::GenericParam::Const(c) => c.ident.to_string(),
        })
        .collect();
    params.join(", ")
}

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(tp) => {
            tp.path
                .segments
                .iter()
                .map(|s| {
                    let ident = s.ident.to_string();
                    match &s.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            let inner: Vec<String> = args
                                .args
                                .iter()
                                .filter_map(|a| match a {
                                    syn::GenericArgument::Type(t) => Some(type_to_string(t)),
                                    syn::GenericArgument::Lifetime(l) => {
                                        Some(l.ident.to_string())
                                    }
                                    _ => None,
                                })
                                .collect();
                            format!("{}<{}>", ident, inner.join(", "))
                        }
                        _ => ident,
                    }
                })
                .collect::<Vec<_>>()
                .join("::")
        }
        syn::Type::Reference(tr) => {
            let mut s = String::from("&");
            if tr.lifetime.is_some() {
                s.push_str("'a ");
            }
            s.push_str(&type_to_string(&tr.elem));
            s
        }
        syn::Type::Tuple(tt) => {
            let inner: Vec<String> = tt.elems.iter().map(type_to_string).collect();
            format!("({})", inner.join(", "))
        }
        syn::Type::Slice(ts) => format!("[{}]", type_to_string(&ts.elem)),
        syn::Type::Array(ta) => format!("[{}; _]", type_to_string(&ta.elem)),
        syn::Type::Ptr(tp) => {
            let kw = if tp.const_token.is_some() {
                "const"
            } else {
                "mut"
            };
            format!("*{} {}", kw, type_to_string(&tp.elem))
        }
        syn::Type::BareFn(_) => "fn(...)".to_string(),
        syn::Type::Never(_) => "!".to_string(),
        syn::Type::TraitObject(to) => {
            let bounds: Vec<String> = to.bounds.iter().map(|b| quote_bound(b)).collect();
            bounds.join(" + ")
        }
        syn::Type::ImplTrait(ti) => {
            let bounds: Vec<String> = ti.bounds.iter().map(|b| quote_bound(b)).collect();
            format!("impl {}", bounds.join(" + "))
        }
        syn::Type::Paren(tp) => format!("({})", type_to_string(&tp.elem)),
        syn::Type::Group(tg) => type_to_string(&tg.elem),
        _ => "?".to_string(),
    }
}

fn quote_bound(bound: &syn::TypeParamBound) -> String {
    match bound {
        syn::TypeParamBound::Trait(tb) => tb
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        syn::TypeParamBound::Lifetime(l) => l.ident.to_string(),
        _ => "?".to_string(),
    }
}

fn fields_to_strings(fields: &syn::Fields) -> Vec<String> {
    fields
        .iter()
        .map(|f| {
            let vis = match &f.vis {
                syn::Visibility::Public(_) => "pub ",
                _ => "",
            };
            match &f.ident {
                Some(name) => format!("{}{}: {}", vis, name, type_to_string(&f.ty)),
                None => format!("{}{}", vis, type_to_string(&f.ty)),
            }
        })
        .collect()
}

fn variants_to_strings(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Vec<String> {
    variants
        .iter()
        .map(|v| {
            let name = v.ident.to_string();
            match &v.fields {
                syn::Fields::Named(fields) => {
                    let inner: Vec<String> = fields
                        .named
                        .iter()
                        .map(|f| match &f.ident {
                            Some(id) => format!("{}: {}", id, type_to_string(&f.ty)),
                            None => type_to_string(&f.ty),
                        })
                        .collect();
                    format!("{} {{ {} }}", name, inner.join(", "))
                }
                syn::Fields::Unnamed(fields) => {
                    let inner: Vec<String> =
                        fields.unnamed.iter().map(|f| type_to_string(&f.ty)).collect();
                    format!("{}({})", name, inner.join(", "))
                }
                syn::Fields::Unit => name,
            }
        })
        .collect()
}

fn extract_attrs(attrs: &[syn::Attribute]) -> ItemAttrs {
    let mut derive = Vec::new();
    let mut doc = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(list) = &attr.meta {
                let derives: Vec<String> = list
                    .tokens
                    .to_string()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                derive.extend(derives);
            }
        } else if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(el) = &nv.value {
                    if let syn::Lit::Str(ls) = &el.lit {
                        doc.push(ls.value());
                    }
                }
            }
        }
    }

    derive.sort();
    ItemAttrs { derive, doc }
}

fn into_public_item(item: &syn::Item) -> Option<PublicItem> {
    let (kind, name, fields, variants, generics, attrs_src) = match item {
        syn::Item::Struct(s) => (
            ItemKind::Struct,
            s.ident.to_string(),
            fields_to_strings(&s.fields),
            vec![],
            generics_to_string(&s.generics),
            &s.attrs,
        ),
        syn::Item::Enum(e) => (
            ItemKind::Enum,
            e.ident.to_string(),
            vec![],
            variants_to_strings(&e.variants),
            generics_to_string(&e.generics),
            &e.attrs,
        ),
        syn::Item::Trait(t) => (
            ItemKind::Trait,
            t.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&t.generics),
            &t.attrs,
        ),
        syn::Item::Fn(f) => (
            ItemKind::Fn,
            f.sig.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&f.sig.generics),
            &f.attrs,
        ),
        syn::Item::Type(t) => (
            ItemKind::Type,
            t.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&t.generics),
            &t.attrs,
        ),
        syn::Item::Macro(m) => {
            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
            if name.is_empty() {
                return None;
            }
            (
                ItemKind::Macro,
                name,
                vec![],
                vec![],
                String::new(),
                &m.attrs,
            )
        }
        _ => return None,
    };

    let line = item_ident_span(item).unwrap_or(0);
    let attrs = extract_attrs(attrs_src);
    let visibility = vis_to_string(item_vis(item));

    Some(PublicItem {
        kind,
        name,
        file: String::new(),
        line,
        attrs,
        generics,
        visibility,
        fields,
        variants,
        impls: vec![],
    })
}

fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {
    match tree {
        syn::UseTree::Path(p) => {
            let new_prefix = if prefix.is_empty() {
                p.ident.to_string()
            } else {
                format!("{}::{}", prefix, p.ident)
            };
            flatten_use_tree(&p.tree, new_prefix, line)
        }
        syn::UseTree::Name(n) => {
            let path = if prefix.is_empty() {
                n.ident.to_string()
            } else {
                format!("{}::{}", prefix, n.ident)
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Rename(r) => {
            let path = if prefix.is_empty() {
                format!("{} as {}", r.ident, r.rename)
            } else {
                format!("{}::{} as {}", prefix, r.ident, r.rename)
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Glob(_) => {
            let path = if prefix.is_empty() {
                "*".to_string()
            } else {
                format!("{}::*", prefix)
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|t| flatten_use_tree(t, prefix.clone(), line))
            .collect(),
    }
}

fn extract_re_exports_from_tree(
    tree: &syn::UseTree,
    import_path: String,
    line: usize,
) -> Vec<ReExport> {
    match tree {
        syn::UseTree::Path(p) => {
            let new_import = if import_path.is_empty() {
                p.ident.to_string()
            } else {
                format!("{}::{}", import_path, p.ident)
            };
            extract_re_exports_from_tree(&p.tree, new_import, line)
        }
        syn::UseTree::Name(n) => {
            vec![ReExport {
                import_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            vec![ReExport {
                import_path,
                export_path: r.rename.to_string(),
                line,
            }]
        }
        syn::UseTree::Glob(_) => {
            vec![ReExport {
                import_path,
                export_path: "*".to_string(),
                line,
            }]
        }
        syn::UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
            .collect(),
    }
}
