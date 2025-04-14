use file_icon_provider::get_file_icon;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use fs_extra::{
    dir::{get_size, ls, CopyOptions, DirEntryAttr, DirEntryValue},
    move_items,
};
use image::{DynamicImage, RgbaImage};
use serde::Serialize;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all, read_dir, File},
    io::{self},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tar::Archive;
use tauri::{command, AppHandle, Manager, Runtime};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    // The size of the path in bytes.
    pub size: u64,
    // The file or directory name of the path.
    pub name: String,
    // The extension name of the path.
    pub extname: String,
    // The full name of the path including extension.
    pub full_name: String,
    // The parent directory name of the path.
    pub parent_name: String,
    // Whether the path exists.
    pub is_exist: bool,
    // Whether the path is a file.
    pub is_file: bool,
    // Whether the path is a directory.
    pub is_dir: bool,
    // Whether the path is a symbolic link.
    pub is_symlink: bool,
    // Whether the path is an absolute path.
    pub is_absolute: bool,
    // Whether the path is a relative path.
    pub is_relative: bool,
    // The access time of the path in milliseconds.
    pub accessed_at: u128,
    // The creation time of the path in milliseconds.
    pub created_at: u128,
    // The modified time of the path in milliseconds.
    pub modified_at: u128,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataOptions {
    // When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it, defaults to `false`.
    pub omit_size: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CompressOptions {
    // The name of the file or directory to be compressed.
    pub includes: Option<Vec<String>>,
    // The name of the file or directory not to be compressed.
    pub excludes: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TransferOptions {
    // The name of the file or directory to be moved.
    pub includes: Option<Vec<String>>,
    // The name of the file or directory not to be moved.
    pub excludes: Option<Vec<String>>,
}

/// Check if a path exists.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::is_exist;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let exists = is_exist(path).await;
/// println!("{}", exists); // true
/// ```
#[command]
pub async fn is_exist(path: PathBuf) -> bool {
    path.exists()
}

/// Check if a path is a file.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::is_file;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let is_file = is_file(path).await;
/// println!("{}", is_file); // true
/// ```
#[command]
pub async fn is_file(path: PathBuf) -> bool {
    path.is_file()
}

/// Check if a path is a directory.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::is_dir;
///
/// let path = PathBuf::from("/path/to/directory");
/// let is_dir = is_dir(path).await;
/// println!("{}", is_dir); // true
/// ```
#[command]
pub async fn is_dir(path: PathBuf) -> bool {
    path.is_dir()
}

/// Get the size of the path, or 0 if it does not exist.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::size;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let size = size(path).await;
/// println!("{}", size); // 1024
/// ```
#[command]
pub async fn size(path: PathBuf) -> u64 {
    get_size(path).unwrap_or(0)
}

/// Get the name of the path.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::name;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let name = name(path).await;
/// println!("{}", name); // "file"
/// ```
#[command]
pub async fn name(path: PathBuf) -> String {
    if path.is_dir() {
        return full_name(path).await;
    }

    path.file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the extension name of a path.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::extname;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let ext = extname(path).await;
/// println!("{}", ext); // "txt"
/// ```
#[command]
pub async fn extname(path: PathBuf) -> String {
    if path.is_dir() {
        return String::default();
    }

    path.extension()
        .map(|extname| extname.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the full name of a file or directory including extension.
///
/// # Arguments
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::full_name;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let full_name = full_name(path).await;
/// println!("{}", full_name); // "file.txt"
/// ```
#[command]
pub async fn full_name(path: PathBuf) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the parent directory name of a path.
///
/// # Arguments
/// - `path`: Specify the path.
/// - `level`: Specify the level of the parent directory, defaults to `1`.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::parent_name;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let parent = parent_name(path, None).await;
/// println!("{}", parent.unwrap()); // "to"
/// ```
#[command]
pub async fn parent_name(path: PathBuf, level: Option<u8>) -> Result<String, String> {
    let mut current = path;
    for _ in 0..level.unwrap_or(1) {
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return Ok(String::default());
        }
    }
    Ok(full_name(current).await)
}

async fn get_icon_name(path: PathBuf) -> Result<String, String> {
    let is_dir = is_dir(path.clone()).await;
    let name = name(path.clone()).await;
    let extname = extname(path.clone()).await;
    let full_name = full_name(path.clone()).await;

    let is_mac_app = cfg!(target_os = "macos") && extname.eq(&"app");
    let is_win_app = cfg!(target_os = "windows") && extname.eq(&"exe");

    if is_mac_app || is_win_app {
        return Ok(full_name);
    }

    if is_dir {
        return Ok("__TAURI_PLUGIN_FS_PRO_DIRECTORY__".to_string());
    }

    if extname.is_empty() {
        return Ok(name);
    }

    return Ok(extname);
}

/// Get the icon of a path.
///
/// # Arguments
/// - `path`: Specify the path.
/// - `size`: Specify the size of the icon, defaults to `32`.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::icon;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let icon_path = icon(app.handle(), path, None).await?;
/// println!("{}", icon_path);
/// ```
#[command]
pub async fn icon<R: Runtime>(
    app_handle: AppHandle<R>,
    path: PathBuf,
    size: Option<u16>,
) -> Result<PathBuf, String> {
    let size = size.unwrap_or(32);

    let icon = get_file_icon(path.clone(), size).map_err(|err| err.to_string())?;

    let image = RgbaImage::from_raw(icon.width, icon.height, icon.pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| "Failed to convert Icon to Image".to_string())?;

    // TODO: 支持自定义存储路路径
    let save_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join("tauri-plugin-fs-pro")
        .join("icons");

    create_dir_all(&save_dir).map_err(|err| err.to_string())?;

    let icon_name = get_icon_name(path).await?;

    let save_path = save_dir.join(format!("{}.png", icon_name));

    if save_path.exists() {
        return Ok(save_path);
    }

    image.save(&save_path).map_err(|err| err.to_string())?;

    Ok(save_path)
}

fn system_time_to_unix_millis(time: io::Result<SystemTime>) -> u128 {
    match time {
        Ok(system_time) => system_time
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default(),
        Err(_) => 0,
    }
}

/// Get the metadata of the path.
///
/// # Arguments
/// - `path`: Specify the path.
/// - `options.omitSize`: When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it, defaults to `false`.
///
/// # Returns
/// - `Ok(Metadata)`: The metadata of the path.
/// - `Err(String)`: An error message string on failure.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::metadata;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let metadata = metadata(path, None).await?;
/// println!("{:?}", metadata);
/// ```
#[command]
pub async fn metadata(path: PathBuf, options: Option<MetadataOptions>) -> Result<Metadata, String> {
    let omit_size = options
        .map(|options| options.omit_size.unwrap_or(false))
        .unwrap_or(false);

    let size = if omit_size {
        0
    } else {
        size(path.clone()).await
    };
    let name = name(path.clone()).await;
    let extname = extname(path.clone()).await;
    let full_name = full_name(path.clone()).await;
    let parent_name = parent_name(path.clone(), Some(1)).await?;

    let is_dir = path.is_dir();
    let is_file = path.is_file();
    let is_exist = path.exists();
    let is_symlink = path.is_symlink();
    let is_absolute = path.is_absolute();
    let is_relative = path.is_relative();

    let metadata = fs::metadata(path).map_err(|err| err.to_string())?;
    let accessed_at = system_time_to_unix_millis(metadata.accessed());
    let created_at = system_time_to_unix_millis(metadata.created());
    let modified_at = system_time_to_unix_millis(metadata.modified());

    Ok(Metadata {
        size,
        name,
        extname,
        full_name,
        parent_name,
        is_dir,
        is_file,
        is_exist,
        is_symlink,
        is_absolute,
        is_relative,
        accessed_at,
        created_at,
        modified_at,
    })
}

/// Compress the source path into a tar.gz file to the destination path.
///
/// # Arguments
/// - `src_path`: Specify the source path.
/// - `dst_path`: Specify the destination path.
/// - `options.includes`: The name of the file or directory to be compressed.
/// - `options.excludes`: The name of the file or directory not to be compressed.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::compress;
///
/// let src_path = PathBuf::from("/path/to/source");
/// let dst_path = PathBuf::from("/path/to/destination.tar.gz");
/// compress(src_path, dst_path, None).await?;
/// ```
#[command]
pub async fn compress(
    src_path: PathBuf,
    dst_path: PathBuf,
    options: Option<CompressOptions>,
) -> Result<(), String> {
    let options = options.unwrap_or(CompressOptions {
        includes: Some(vec![]),
        excludes: Some(vec![]),
    });
    let includes = options.includes.unwrap_or(vec![]);
    let excludes = options.excludes.unwrap_or(vec![]);

    let dst_file = File::create(dst_path.clone()).map_err(|err| err.to_string())?;
    let enc = GzEncoder::new(dst_file, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for entry in read_dir(&src_path).map_err(|err| err.to_string())? {
        let path = entry.map_err(|err| err.to_string())?.path();
        let is_file = path.is_file();
        let full_name = full_name(path.clone()).await;

        if excludes.iter().any(|name| &full_name == name) {
            continue;
        }

        if !includes.is_empty() && !includes.iter().any(|name| &full_name == name) {
            continue;
        }

        if is_file {
            let file = &mut File::open(path.clone()).map_err(|err| err.to_string())?;

            tar.append_file(full_name, file)
                .map_err(|err| err.to_string())?;
        } else {
            tar.append_dir_all(full_name, path.clone())
                .map_err(|err| err.to_string())?;
        }
    }

    tar.finish().map_err(|err| err.to_string())?;

    Ok(())
}

/// Decompress the tar.gz file from the source path to the destination path.
///
/// # Arguments
/// - `src_path`: Specify the source path.
/// - `dst_path`: Specify the destination path.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::decompress;
///
/// let src_path = PathBuf::from("/path/to/source.tar.gz");
/// let dst_path = PathBuf::from("/path/to/destination");
/// decompress(src_path, dst_path).await?;
/// ```
#[command]
pub async fn decompress(src_path: PathBuf, dst_path: PathBuf) -> Result<(), String> {
    create_dir_all(dst_path.clone()).map_err(|err| err.to_string())?;

    let src_file = File::open(src_path).map_err(|err| err.to_string())?;
    let decoder = GzDecoder::new(src_file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries().map_err(|err| err.to_string())? {
        let mut entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path().map_err(|err| err.to_string())?.to_path_buf();

        #[cfg(target_os = "windows")]
        let path = std::path::Path::new(&path.to_string_lossy().replace("\\", "/")).to_path_buf();

        entry
            .unpack(dst_path.join(path))
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

/// Move the source path to the destination path.
///
/// # Arguments
/// - `src_path`: Specify the source path.
/// - `dst_path`: Specify the destination path.
/// - `options.includes`: The name of the file or directory to be moved.
/// - `options.excludes`: The name of the file or directory not to be moved.
///
/// # Example
/// ```
/// use std::path::PathBuf;
/// use tauri_plugin_fs_pro::transfer;
///
/// let src_path = PathBuf::from("/path/to/source");
/// let dst_path = PathBuf::from("/path/to/destination");
/// transfer(src_path, dst_path, None).await?;
/// ```
#[command]
pub async fn transfer(
    src_path: PathBuf,
    dst_path: PathBuf,
    options: Option<TransferOptions>,
) -> Result<(), String> {
    let options = options.unwrap_or(TransferOptions {
        includes: Some(vec![]),
        excludes: Some(vec![]),
    });
    let includes = options.includes.unwrap_or(vec![]);
    let excludes = options.excludes.unwrap_or(vec![]);

    create_dir_all(dst_path.clone()).map_err(|err| err.to_string())?;

    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Path);

    let ls_result = ls(&src_path, &config).map_err(|err| err.to_string())?;

    let mut from_items = Vec::new();

    for item in ls_result.items {
        if let Some(path) = item.get(&DirEntryAttr::Path) {
            if let &DirEntryValue::String(ref path) = path {
                let path = PathBuf::from(path);
                let full_name = full_name(path.clone()).await;

                if excludes.iter().any(|name| &full_name == name) {
                    continue;
                }

                if !includes.is_empty() && !includes.iter().any(|name| &full_name == name) {
                    continue;
                }

                from_items.push(path);
            }
        }
    }

    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        content_only: false,
        depth: 0,
    };

    move_items(&from_items, &dst_path, &options).map_err(|err| err.to_string())?;

    Ok(())
}
