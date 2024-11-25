const COMMANDS: &[&str] = &[
    "is_exist",
    "is_dir",
    "is_file",
    "size",
    "name",
    "full_name",
    "extname",
    "metadata",
    "open",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
