fn main() {
    println!("cargo::rustc-check-cfg=cfg(docs_rs)");
}