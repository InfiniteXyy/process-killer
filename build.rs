fn main() {
    #[cfg(target_os = "windows")]
    embed_resource::compile("app.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
