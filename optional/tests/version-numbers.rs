#[macro_use]
extern crate version_sync;

#[test]
fn readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn html_root_url() {
    // FIXME: version-sync fails to parse!
    // assert_html_root_url_updated!("src/lib.rs");
}

