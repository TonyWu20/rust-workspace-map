use std::collections::BTreeMap;
use std::path::PathBuf;

// ── Path newtypes for flat indexes ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CanonicalPath(pub String);

impl std::fmt::Display for CanonicalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for CanonicalPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CanonicalPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceRelativePath(pub String);

impl std::fmt::Display for WorkspaceRelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for WorkspaceRelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for WorkspaceRelativePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// ── Error type ──────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no workspace root found starting from {0}")]
    WorkspaceRootNotFound(PathBuf),

    #[error("no Cargo.toml with [package] section found starting from {0}")]
    CrateRootNotFound(PathBuf),

    #[error("failed to read file {path}: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse {path}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to parse Rust source {path}: {source}")]
    SynParse {
        path: PathBuf,
        source: syn::Error,
    },

    #[error("workspace member {0} does not exist")]
    MemberNotFound(PathBuf),

    #[error("glob pattern error: {0}")]
    GlobPattern(String),

    #[error("workspace Cargo.toml is missing the [workspace] section")]
    MissingWorkspaceSection,
}

pub type Result<T> = std::result::Result<T, Error>;

// ── Config ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    /// Absolute, canonical path to the workspace root (or a subdirectory within it).
    pub workspace_path: PathBuf,

    /// If Some, write JSON to this file instead of stdout.
    pub output_path: Option<PathBuf>,

    /// When true, run validation checks (orphan files, dead re-exports).
    #[builder(default)]
    pub validate: bool,
}

// ── Crate type ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CrateType {
    Lib,
    Bin,
    #[serde(rename = "lib_and_bin")]
    LibAndBin,
}

// ── Top-level output ────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,

    #[builder(default)]
    pub symbols: BTreeMap<CanonicalPath, SymbolEntry>,

    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<CanonicalPath>>,

    #[builder(default)]
    pub files: BTreeMap<WorkspaceRelativePath, FileEntry>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,

    /// Not serialized — used for path relativization during construction.
    #[builder(default)]
    #[serde(skip)]
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub root: String,
    pub workspace_name: String,
}

// ── Crate ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrateInfo {
    pub name: String,
    pub root: String,
    pub package: PackageInfo,
    pub modules: Vec<ModuleInfo>,
    pub deps: DepInfo,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cross_crate_imports: Vec<CrossCrateImport>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub crate_type: CrateType,
}

// ── Dependencies ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DepInfo {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub normal: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dev: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub workspace_members: Vec<String>,
}

// ── Module ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    pub path: String,
    pub file: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_items: Vec<PublicItem>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<Import>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub re_exports: Vec<ReExport>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submodules: Vec<String>,
}

// ── Public items ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PublicItem {
    pub kind: ItemKind,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub attrs: ItemAttrs,
    pub generics: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    Fn,
    Type,
    Macro,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ItemAttrs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc: Vec<String>,
}

// ── Impl blocks ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplInfo {
    /// Serialized as "type" in JSON.
    #[serde(rename = "type")]
    pub type_: String,
    pub items: Vec<ImplItem>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplItem {
    pub kind: ImplItemKind,
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImplItemKind {
    Fn,
    Type,
    Const,
}

// ── Imports / Re-exports ────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ReExport {
    pub import_path: String,
    pub export_path: String,
    pub line: usize,
}

// ── Cross-crate ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossCrateImport {
    pub import_path: String,
    pub target_crate: String,
    pub symbol: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossReferences {
    #[builder(default)]
    pub types: BTreeMap<String, TypeRef>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef {
    pub crate_name: String,
    pub kind: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imported_by: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exported_by: Vec<String>,
}

// ── Diagnostic kind ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    OrphanedModule,
    TomlParseError,
    MissingCrateRoots,
    ModuleTreeError,
    SynParseError,
    MissingWorkspaceSection,
    GlobPatternError,
    MemberNotFound,
    OrphanFile,
    DeadReExport,
}

// ── Error severity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverity {
    Error,
    Warning,
}

// ── Error context ───────────────────────────────────────────────────────

/// Optional context attached to an error, providing additional location
/// and source information for diagnostics.
#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<String>,

    /// Line number in the source file where the error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// A short source snippet near the error location (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

// ── Internal types ──────────────────────────────────────────────────────

/// Internal intermediate type consumed by `module_tree`.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub public_items: Vec<PublicItem>,
    pub imports: Vec<Import>,
    pub re_exports: Vec<ReExport>,
    pub submodules: Vec<SubmoduleDecl>,
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct SubmoduleDecl {
    pub name: String,
    #[builder(default)]
    pub is_test: bool,
}

// ── Flat index entry types ──────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive_attrs: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub module_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_module_file: Option<String>,
    pub is_crate_root: bool,
}

// ── Error reporting ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: DiagnosticKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
