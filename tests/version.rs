#[cfg(feature = "custom-default-colors")]
use atomic_ref as _;
#[cfg(feature = "detect-color-support")]
use supports_color as _;
use unwind_context as _;

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn test_changelog_mentions_version() {
    version_sync::assert_contains_regex!("CHANGELOG.md", "^## \\[{version}\\] - ");
    version_sync::assert_contains_regex!("CHANGELOG.md", "\\[{version}\\]: ");
}
