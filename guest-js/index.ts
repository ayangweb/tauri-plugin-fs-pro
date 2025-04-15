import { invoke } from "@tauri-apps/api/core";

export interface IconOptions {
  /**
   * The size of the icon, defaults to `32`.
   */
  size?: number;
  /**
   * The path to save the icon, defaults to the default save path.
   */
  savePath?: string;
}

export interface MetadataOptions {
  /**
   * When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it.
   */
  omitSize?: boolean;
}

export interface Metadata {
  /**
   * The size of the path in bytes.
   */
  size: number;
  /**
   * The file or directory name of the path.
   */
  name: string;
  /**
   * The extension name of the path.
   */
  extname: string;
  /**
   * The full name of the path including extension.
   */
  fullName: string;
  /**
   * The parent directory name of the path.
   */
  parentName: string;
  /**
   * Whether the path exists.
   */
  isExist: boolean;
  /**
   * Whether the path is a file.
   */
  isFile: boolean;
  /**
   * Whether the path is a directory.
   */
  isDir: boolean;
  /**
   * Whether the path is a symbolic link.
   */
  isSymlink: boolean;
  /**
   * Whether the path is an absolute path.
   */
  isAbsolute: boolean;
  /**
   * Whether the path is a relative path.
   */
  isRelative: boolean;
  /**
   * The access time of the path in milliseconds.
   */
  accessedAt: number;
  /**
   * The creation time of the path in milliseconds.
   */
  createdAt: number;
  /**
   * The modified time of the path in milliseconds.
   */
  modifiedAt: number;
}

export interface CompressOptions {
  /**
   * The name of the file or directory to be compressed.
   */
  includes?: string[];
  /**
   * The name of the file or directory not to be compressed.
   */
  excludes?: string[];
}

export interface TransferOptions {
  /**
   * The name of the file or directory to be moved.
   */
  includes?: string[];
  /**
   * The name of the file or directory not to be moved.
   */
  excludes?: string[];
}

export const COMMAND = {
  IS_EXIST: "plugin:fs-pro|is_exist",
  IS_FILE: "plugin:fs-pro|is_file",
  IS_DIR: "plugin:fs-pro|is_dir",
  SIZE: "plugin:fs-pro|size",
  NAME: "plugin:fs-pro|name",
  EXTNAME: "plugin:fs-pro|extname",
  FULL_NAME: "plugin:fs-pro|full_name",
  PARENT_NAME: "plugin:fs-pro|parent_name",
  GET_DEFAULT_SAVE_ICON_PATH: "plugin:fs-pro|get_default_save_icon_path",
  ICON: "plugin:fs-pro|icon",
  METADATA: "plugin:fs-pro|metadata",
  COMPRESS: "plugin:fs-pro|compress",
  DECOMPRESS: "plugin:fs-pro|decompress",
  TRANSFER: "plugin:fs-pro|transfer",
};

/**
 * Check if a path exists.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { isExist } from "tauri-plugin-fs-pro-api"
 *
 * const exists = await isExist("/path/to/file.txt")
 * console.log(exists) // true
 * ```
 */
export const isExist = (path: string) => {
  return invoke<boolean>(COMMAND.IS_EXIST, {
    path,
  });
};

/**
 * Check if a path is a file.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { isFile } from "tauri-plugin-fs-pro-api"
 *
 * const isFile = await isFile("/path/to/file.txt")
 * console.log(isFile) // true
 * ```
 */
export const isFile = (path: string) => {
  return invoke<boolean>(COMMAND.IS_FILE, {
    path,
  });
};

/**
 * Check if a path is a directory.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { isDir } from "tauri-plugin-fs-pro-api"
 *
 * const isDir = await isDir("/path/to/dir")
 * console.log(isDir) // true
 * ```
 */
export const isDir = (path: string) => {
  return invoke<boolean>(COMMAND.IS_DIR, {
    path,
  });
};

/**
 * Get the size of the path, or 0 if it does not exist.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { size } from "tauri-plugin-fs-pro-api"
 *
 * const size = await size("/path/to/file.txt")
 * console.log(size) // 1024
 * ```
 */
export const size = (path: string) => {
  return invoke<number>(COMMAND.SIZE, {
    path,
  });
};

/**
 * Get the name of the path.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { name } from "tauri-plugin-fs-pro-api"
 *
 * const name = await name("/path/to/file.txt")
 * console.log(name) // file
 * ```
 */
export const name = (path: string) => {
  return invoke<string>(COMMAND.NAME, {
    path,
  });
};

/**
 * Get the extension name of the path.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { extname } from "tauri-plugin-fs-pro-api"
 *
 * const extname = await extname("/path/to/file.txt")
 * console.log(extname) // txt
 * ```
 */
export const extname = (path: string) => {
  return invoke<string>(COMMAND.EXTNAME, {
    path,
  });
};

/**
 * Get the full name of a file or directory including extension.
 *
 * @param path Specify the path.
 *
 * @example
 * ```
 * import { fullName } from "tauri-plugin-fs-pro-api"
 *
 * const fullName = await fullName("/path/to/file.txt")
 * console.log(fullName) // file.txt
 * ```
 */
export const fullName = (path: string) => {
  return invoke<string>(COMMAND.FULL_NAME, {
    path,
  });
};

/**
 * Get the parent name of the path.
 *
 * @param path Specify the path.
 * @param level Specify the level of the parent directory, defaults to `1`.
 *
 * @example
 * ```
 * import { parentName } from "tauri-plugin-fs-pro-api"
 *
 * const parentName = await parentName("/path/to/file.txt")
 * console.log(parentName) // to
 * ```
 */
export const parentName = (path: string, level = 1) => {
  return invoke<string>(COMMAND.PARENT_NAME, {
    path,
    level,
  });
};

/**
 * Get the default save icon path.
 *
 * @example
 * ```
 * import { getDefaultSaveIconPath } from "tauri-plugin-fs-pro-api"
 *
 * const savePath = await getDefaultSaveIconPath()
 * console.log(savePath)
 * ```
 */
export const getDefaultSaveIconPath = () => {
  return invoke<string>(COMMAND.GET_DEFAULT_SAVE_ICON_PATH);
};

/**
 * Get the icon of the path.
 *
 * @param path Specify the path.
 * @param options.size Specify the size of the icon, defaults to `32`.
 * @param options.savePath Specify the path to save the icon, defaults to the default save path.
 *
 * @example
 * ```
 * import { icon } from "tauri-plugin-fs-pro-api"
 *
 * const iconPath = await icon("/path/to/file.txt")
 * console.log(iconPath)
 * ```
 */
export const icon = (path: string, options: IconOptions) => {
  return invoke<string>(COMMAND.ICON, {
    path,
    options,
  });
};

/**
 * Get the metadata of the path.
 *
 * @param path Specify the path.
 * @param options.omitSize When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it,
 * defaults to `false`.
 *
 * @example
 * ```
 * import { metadata } from "tauri-plugin-fs-pro-api"
 *
 * const metadata = await metadata("/path/to/file.txt")
 * console.log(metadata)
 * ```
 */
export const metadata = (path: string, options?: MetadataOptions) => {
  return invoke<Metadata>(COMMAND.METADATA, {
    path,
    options,
  });
};

/**
 * Compress the source path into a tar.gz file to the destination path.
 *
 * @param srcPath Specify the source path.
 * @param dstPath Specify the destination path.
 * @param options.includes The name of the file or directory to be compressed.
 * @param options.excludes The name of the file or directory not to be compressed.
 *
 * @example
 * ```
 * import { compress } from "tauri-plugin-fs-pro-api"
 *
 * await compress("/path/to/source.txt", "/path/to/destination.tar.gz")
 * ```
 */
export const compress = (
  srcPath: string,
  dstPath: string,
  options?: CompressOptions
) => {
  return invoke(COMMAND.COMPRESS, {
    srcPath,
    dstPath,
    options,
  });
};

/**
 * Decompress the tar.gz file from the source path to the destination path.
 *
 * @param srcPath Specify the source path.
 * @param dstPath Specify the destination path.
 *
 * @example
 * import { decompress } from "tauri-plugin-fs-pro-api"
 *
 * await decompress("/path/to/destination.tar.gz", "/path/to/source")
 */
export const decompress = (srcPath: string, dstPath: string) => {
  return invoke(COMMAND.DECOMPRESS, {
    srcPath,
    dstPath,
  });
};

/**
 * Move the source path to the destination path.
 *
 * @param srcPath Specify the source path.
 * @param dstPath Specify the destination path.
 * @param options.includes The name of the file or directory to be moved.
 * @param options.excludes The name of the file or directory not to be moved.
 *b
 * @example
 * import { transfer } from "tauri-plugin-fs-pro-api"
 *
 * await transfer("/path/to/source", "/path/to/destination")
 */
export const transfer = (
  srcPath: string,
  dstPath: string,
  options?: TransferOptions
) => {
  return invoke(COMMAND.TRANSFER, {
    srcPath,
    dstPath,
    options,
  });
};
