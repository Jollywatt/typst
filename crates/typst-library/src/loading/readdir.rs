use typst_syntax::Spanned;

use crate::World;
use crate::diag::{At, HintedString, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Array, IntoValue, PathOrStr, Str, func};

/// Reads the entries of a directory.
///
/// Returns an array of strings with paths to the directory's immediate entries.
///
/// = Example <example>
/// ```example
/// #for entry in readdir(".") [
///   - #entry
/// ]
/// ```
#[func]
pub fn readdir(
    engine: &mut Engine,
    /// Path to a directory.
    path: Spanned<PathOrStr>,
) -> SourceResult<Array> {
    let span = path.span;
    let path = path.v;
    let resolved = path.resolve_if_some(span.id()).at(span)?;
    let entries = engine
        .world
        .dir(resolved.clone().intern())
        .map_err(|error| {
            let mut hinted = HintedString::from(error);
            if let PathOrStr::Str(string) = &path
                && (string.as_str().starts_with("http://")
                    || string.as_str().starts_with("https://"))
            {
                hinted.hint("network access is not supported");
            }
            hinted
        })
        .at(span)?;

    let mut array = Array::with_capacity(entries.len());
    for entry in entries {
        let path = Str::from(entry.get_with_slash());
        array.push(path.into_value());
    }

    Ok(array)
}
