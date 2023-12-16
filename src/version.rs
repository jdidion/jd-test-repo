use once_cell::sync::Lazy;

include!(concat!(env!("OUT_DIR"), "/built.rs"));

/// Version of the software including
/// - Git commit hash
/// - Git dirty info (whether the repo had uncommitted changes)
/// - Cargo package version if no git info found
pub static VERSION: Lazy<String> = Lazy::new(|| {
    let prefix = if let Some(s) = GIT_COMMIT_HASH {
        format!("{PKG_VERSION}-{}", s[0..8].to_owned())
    } else {
        // This shouldn't happen
        PKG_VERSION.to_string()
    };
    let suffix = match GIT_DIRTY {
        Some(true) => "-dirty",
        _ => "",
    };
    format!("{prefix}{suffix}")
});
