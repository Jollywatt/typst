use typst_syntax::Spanned;

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
/// The returned paths are absolute (starting with `/`) and can be passed
/// directly to functions like @read, @image, @csv, or similar.
///
/// This function is only available when compiling with the CLI; it is not
/// supported in environments without direct filesystem access.
///
/// = Example <example>
/// Load and include all chapters from a subdirectory:
/// ```typ
/// #for chapter in glob("chapters/*.typ") {
///   include chapter
/// }
/// ```
///
/// Collect all PNG images under the project root:
/// ```typ
/// #let images = glob("/assets/**/*.png")
/// #grid(
///   columns: 3,
///   ..images.map(image),
/// )
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
) -> SourceResult<Array> {
    let span = pattern.span;
    let within = span.id().ok_or("cannot access file system from here").at(span)?;

    let paths = engine
        .world
        .glob(pattern.v.into(), within)
        .at(span)?;

    Ok(paths
        .into_iter()
        .map(|vpath| Value::Str(vpath.get_with_slash().into()))
        .collect())
}
