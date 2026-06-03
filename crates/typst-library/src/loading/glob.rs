use typst_syntax::{Spanned, VirtualPath};

use crate::World;
use crate::diag::{At, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Array, Str, Value, func};

/// Finds files matching a glob pattern and returns their paths as an array.
///
/// The pattern may contain the following wildcards:
/// - `?` matches any single character within a path segment.
/// - `*` matches any sequence of characters within a path segment.
/// - `**` matches any sequence of path segments, including none.
///
/// Patterns follow the same path resolution rules as other paths in Typst:
/// - A pattern starting with `/` resolves relative to the project root.
/// - Otherwise, the pattern resolves relative to the directory of the calling
///   file.
///
/// By default the returned paths are *relative* to the directory of the calling
/// file, so they can be passed directly to functions like @read, @image, or
/// @include from the same location. Pass `{absolute: true}` to get
/// project-root-relative paths instead (starting with `/`).
///
/// This function is only available when compiling with the CLI; it is not
/// supported in environments without direct filesystem access.
///
/// = Example <example>
/// Load and include all chapters from a subdirectory (relative paths):
/// ```typ
/// #for chapter in glob("chapters/*.typ") {
///   include chapter
/// }
/// ```
///
/// Collect all PNG images, passing them to a function in another file where
/// absolute paths are needed:
/// ```typ
/// #let images = glob("/assets/**/*.png", absolute: true)
/// ```
#[func]
pub fn glob(
    engine: &mut Engine,
    /// A glob pattern specifying which files to match.
    ///
    /// A pattern starting with `/` is resolved relative to the project root.
    /// Otherwise, the pattern is resolved relative to the directory of the
    /// calling file.
    pattern: Spanned<Str>,
    /// Whether to return absolute paths (relative to the project root,
    /// starting with `/`) instead of paths relative to the calling file's
    /// directory.
    ///
    /// Defaults to `{false}`.
    #[named]
    #[default(false)]
    absolute: bool,
) -> SourceResult<Array> {
    let span = pattern.span;
    let within = span.id().ok_or("cannot access file system from here").at(span)?;

    let paths = engine.world.glob(pattern.v.into(), within).at(span)?;

    // Caller's directory — parent of the calling file's virtual path.
    let caller_dir: VirtualPath = within
        .vpath()
        .parent()
        .unwrap_or_else(|| within.vpath().clone());

    Ok(paths
        .into_iter()
        .map(|vpath| {
            let s: Str = if absolute {
                vpath.get_with_slash().into()
            } else {
                vpath.relative_from(&caller_dir).as_str().into()
            };
            Value::Str(s)
        })
        .collect())
}
