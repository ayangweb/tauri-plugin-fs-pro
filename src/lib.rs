use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

pub use commands::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("fs-pro")
        .invoke_handler(tauri::generate_handler![
            commands::is_exist,
            commands::is_dir,
            commands::is_file,
            commands::size,
            commands::name,
            commands::full_name,
            commands::extname,
            commands::metadata,
            commands::open,
            commands::compress,
            commands::decompress,
            commands::transfer,
        ])
        .build()
}
