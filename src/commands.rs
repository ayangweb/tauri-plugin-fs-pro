use file_icon_provider::get_file_icon;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use fs_extra::{
    dir::{get_size, ls, CopyOptions, DirEntryAttr, DirEntryValue},
    move_items,
};
use image::{DynamicImage, ImageFormat, RgbaImage};
use serde::Serialize;
use showfile::show_path_in_file_manager;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all, read_dir, File},
    io::{self, Cursor},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tar::Archive;
use tauri::command;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    // The size of the path in bytes.
    pub size: u64,
    // The file or directory name of the path.
    pub name: String,
    // The file or directory name of the path, including the extension name if it is a file.
    pub full_name: String,
    // The extension name of the path.
    pub extname: String,
    // Whether the path is a directory.
    pub is_dir: bool,
    // Whether the path is a file.
    pub is_file: bool,
    // Whether the path exists.
    pub is_exist: bool,
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
    // When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it.
    pub omit_size: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOptions {
    // Whether to open in file explorer.
    pub explorer: Option<bool>,
    // If the path is a directory, does it go directly into the directory.
    pub enter_dir: Option<bool>,
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

/// Whether the path exists.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::is_exist;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let exists = is_exist(path).await;
/// println!("{}", exists); // true
/// ```
#[command]
pub async fn is_exist(path: PathBuf) -> bool {
    path.exists()
}

/// Whether the path is a directory.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::is_dir;
///
/// let path = PathBuf::from("/path/to/dir");
/// let is_dir = is_dir(path).await;
/// println!("{}", is_dir); // true
/// ```
#[command]
pub async fn is_dir(path: PathBuf) -> bool {
    path.is_dir()
}

/// Whether the path is a file.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::is_file;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let is_file = is_file(path).await;
/// println!("{}", is_file); // true
/// ```
#[command]
pub async fn is_file(path: PathBuf) -> bool {
    path.is_file()
}

/// Get the size of the path, or 0 if it does not exist.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::size;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let size = size(path).await;
/// println!("{}", size); // 1024
/// ```
#[command]
pub async fn size(path: PathBuf) -> u64 {
    get_size(path).unwrap_or(0)
}

/// Get the file or directory name of the path.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::name;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let name = name(path).await;
/// println!("{}", name); // file
/// ```
#[command]
pub async fn name(path: PathBuf) -> String {
    path.file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the file or directory name of the path, including the extension name if it is a file.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::full_name;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let name = full_name(path).await;
/// println!("{}", name); // file.text
/// ```
#[command]
pub async fn full_name(path: PathBuf) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the extension name of the path.
///
/// # Arguments
///
/// - `path`: Specify the path.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::extname;
///
/// let path = PathBuf::from("/path/to/file.text");
/// let name = extname(path).await;
/// println!("{}", name); // text
/// ```
#[command]
pub async fn extname(path: PathBuf) -> String {
    path.extension()
        .map(|extname| extname.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Get the system icon for the path.
///
/// # Arguments
/// - `path`: Specify the path.
/// - `size`: Specify the size of the icon, defaults to `32`.
///
/// # Returns
/// - `Ok(Vec<u8>)`: A byte vector containing the PNG image data on success.
/// - `Err(String)`: An error message string on failure.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::icon;
///
/// let path = PathBuf::from("/path/to/file.png");
/// let bytes = icon(path, None).await.unwrap();
/// println!("{:?}", bytes);
/// ```
#[command]
pub async fn icon(path: PathBuf, size: Option<u16>) -> Result<Vec<u8>, String> {
    let size = size.unwrap_or(32);

    let icon = get_file_icon(path, size).map_err(|err| err.to_string())?;

    let image = RgbaImage::from_raw(icon.width, icon.height, icon.pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| "Failed to create RGBA image".to_string())?;

    let mut bytes = Cursor::new(Vec::new());

    image
        .write_to(&mut bytes, ImageFormat::Png)
        .map_err(|err| err.to_string())?;

    Ok(bytes.into_inner())
}

// Converting system time to unix milliseconds.
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
///
/// * `path`: Specify the path.
/// * `options.omitSize`: When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it, defaults to `false`.
///
/// # Returns
/// - `Ok(Metadata)`: The metadata of the path.
/// - `Err(String)`: An error message string on failure.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::metadata;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// let metadata = metadata(path, None).await.unwrap();
/// println!("{:?}", metadata);
/// ```
#[command]
pub async fn metadata(path: PathBuf, options: Option<MetadataOptions>) -> Result<Metadata, String> {
    let options = options.unwrap_or(MetadataOptions {
        omit_size: Some(false),
    });
    let omit_size = options.omit_size.unwrap_or(false);

    let size = match omit_size {
        true => 0,
        false => size(path.clone()).await,
    };
    let name = name(path.clone()).await;
    let full_name = full_name(path.clone()).await;
    let extname = extname(path.clone()).await;

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
        full_name,
        extname,
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

/// Open the path in file explorer or the default application.
///
/// # Arguments
/// - `path`: Specify the path.
/// - `options.explorer`: Whether to open in file explorer, defaults to `false`.
/// - `options.enterDir`: If the path is a directory, does it go directly into the directory, defaults to `false`.
///
/// # Example
/// ```
/// use tauri_plugin_fs_pro::open;
///
/// let path = PathBuf::from("/path/to/file.txt");
/// open(path, None).await.unwrap();
/// ```
#[command]
pub async fn open(path: PathBuf, options: Option<OpenOptions>) -> Result<(), String> {
    let options = options.unwrap_or(OpenOptions {
        explorer: Some(false),
        enter_dir: Some(false),
    });
    let explorer = options.explorer.unwrap_or(false);
    let enter_dir = options.enter_dir.unwrap_or(false);

    if explorer && !(path.is_dir() && enter_dir) {
        // Open the path in file explorer.
        show_path_in_file_manager(path);
    } else {
        // Open the path in the default application.
        open::that(path).map_err(|err| err.to_string())?;
    }

    Ok(())
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
/// use tauri_plugin_fs_pro::compress;
///
/// let src_path = PathBuf::from("/path/src/EcoPaste");
/// let dst_path = PathBuf::from("/path/dst/EcoPaste.tar.gz");
/// compress(src_path, dst_path, None).await.unwrap();
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
/// use tauri_plugin_fs_pro::decompress;
///
/// let src_path = PathBuf::from("/path/src/EcoPaste.tar.gz");
/// let dst_path = PathBuf::from("/path/dst/EcoPaste");
/// decompress(src_path, dst_path).await.unwrap();
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
/// use tauri_plugin_fs_pro::transfer;
///
/// let src_path = PathBuf::from("/path/src/EcoPaste");
/// let dst_path = PathBuf::from("/path/dst/EcoPaste");
/// transfer(src_path, dst_path, None).await.unwrap();
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
