use helix_loader::grammar::{build_grammars, fetch_grammars};

fn main() {
    if std::env::var("HELIX_DISABLE_AUTO_GRAMMAR_BUILD").is_err() {
        fetch_grammars().expect("Failed to fetch tree-sitter grammars");
        build_grammars(Some(std::env::var("TARGET").unwrap()))
            .expect("Failed to compile tree-sitter grammars");
    }

    println!("cargo:warning=CWD: {:?}", std::env::current_dir());

    if cfg!(target_os = "windows") {
        println!("cargo:warning=CWD: {:?}", std::env::current_dir());
        println!("cargo:rustc-link-lib=./contrib/helix-icon-windows");
    }
}
