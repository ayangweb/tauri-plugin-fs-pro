const COMMANDS: &[&str] = &[
    "is_exist",
    "is_dir",
    "is_file",
    "size",
    "name",
    "full_name",
    "extname",
    "parent_name",
    "icon",
    "metadata",
    "compress",
    "decompress",
    "transfer",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
