import { invoke } from "@tauri-apps/api/core";

/**
 * Metadata information about a file or directory.
 */
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
   * The file or directory name of the path, including the extension name if it is a file.
   */
  fullName: string;
  /**
   * The extension name of the path.
   */
  extname: string;
  /**
   * Whether the path is a directory.
   */
  isDir: boolean;
  /**
   * Whether the path is a file.
   */
  isFile: boolean;
  /**
   * Whether the path exists.
   */
  isExist: boolean;
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

export interface MetadataOptions {
  /**
   * When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it.
   */
  omitSize?: boolean;
}

export interface OpenOptions {
  /**
   * Whether to open in file explorer.
   */
  explorer?: boolean;
  /**
   * If the path is a directory, does it go directly to the directory.
   */
  enterDir?: boolean;
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

export const FS_PRO_COMMAND = {
  IS_EXIST: "plugin:fs-pro|is_exist",
  IS_DIR: "plugin:fs-pro|is_dir",
  IS_FILE: "plugin:fs-pro|is_file",
  SIZE: "plugin:fs-pro|size",
  NAME: "plugin:fs-pro|name",
  FULL_NAME: "plugin:fs-pro|full_name",
  EXTNAME: "plugin:fs-pro|extname",
  METADATA: "plugin:fs-pro|metadata",
  OPEN: "plugin:fs-pro|open",
  COMPRESS: "plugin:fs-pro|compress",
  DECOMPRESS: "plugin:fs-pro|decompress",
  TRANSFER: "plugin:fs-pro|transfer",
};

/**
 * Whether the path exists.
 * @example
 * import { isExist } from "tauri-plugin-fs-pro-api"
 * const yes = await isExist("/Users/xxx/EcoPaste.txt")
 * console.log(yes) // true
 */
export const isExist = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_EXIST, {
    path,
  });
};

/**
 * Whether the path is a directory.
 * @example
 * import { isDir } from "tauri-plugin-fs-pro-api"
 * const yes = await isDir("/Users/xxx/EcoPaste")
 * console.log(yes) // true
 */
export const isDir = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_DIR, {
    path,
  });
};

/**
 * Whether the path is a file.
 * @example
 * import { isFile } from "tauri-plugin-fs-pro-api"
 * const yes = await isFile("/Users/xxx/EcoPaste.txt")
 * console.log(yes) // true
 */
export const isFile = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_FILE, {
    path,
  });
};

/**
 * Get the size of the path in bytes, or 0 if it does not exist.
 * @example
 * import { size } from "tauri-plugin-fs-pro-api"
 * const fileSize = await size("/Users/xxx/EcoPaste.txt")
 * console.log(fileSize) // 1024
 */
export const size = (path: string) => {
  return invoke<number>(FS_PRO_COMMAND.SIZE, {
    path,
  });
};

/**
 * Get the file or directory name of the path.
 * @example
 * import { name } from "tauri-plugin-fs-pro-api"
 * const fileName = await name("/Users/xxx/EcoPaste.txt")
 * console.log(fileName) // "EcoPaste"
 */
export const name = (path: string) => {
  return invoke<string>(FS_PRO_COMMAND.NAME, {
    path,
  });
};

/**
 * Get the file or directory name of the path, including the extension name if it is a file.
 * @example
 * import { fullName } from "tauri-plugin-fs-pro-api"
 * const name = await fullName("/Users/xxx/EcoPaste.txt")
 * console.log(name) // "EcoPaste.txt"
 */
export const fullName = (path: string) => {
  return invoke<string>(FS_PRO_COMMAND.FULL_NAME, {
    path,
  });
};

/**
 * Get the extension name of the file.
 * @example
 * import { extname } from "tauri-plugin-fs-pro-api"
 * const ext = await extname("/Users/xxx/EcoPaste.txt")
 * console.log(ext) // "txt"
 */
export const extname = (path: string) => {
  return invoke<string>(FS_PRO_COMMAND.EXTNAME, {
    path,
  });
};

/**
 * Get the metadata of the path.
 * @param options.omitSize When getting the metadata of a path, if you don't need to calculate the size, you can omit it to save time and return 0 after omitting it
 * Defaults to `false`.
 * @example
 * import { metadata } from "tauri-plugin-fs-pro-api"
 * const { size, isFile } = await metadata("/Users/xxx/EcoPaste.txt", { omitSize: true })
 * console.log(size) // 0
 * console.log(isFile) // true
 * @returns return {@linkcode Metadata}
 */
export const metadata = (path: string, options?: MetadataOptions) => {
  return invoke<Metadata>(FS_PRO_COMMAND.METADATA, {
    path,
    options,
  });
};

/**
 * Open the path in file explorer or the default application.
 * @param options.explorer Whether to open in file explorer
 * Defaults to `false`.
 * @param options.enterDir If the path is a directory, does it go directly into the directory
 * Defaults to `false`.
 * @example
 * import { open } from "tauri-plugin-fs-pro-api"
 * await open("/Users/xxx/EcoPaste.txt", { explorer: true }) // open in file explorer
 */
export const open = (path: string, options?: OpenOptions) => {
  return invoke(FS_PRO_COMMAND.OPEN, {
    path,
    options,
  });
};

/**
 * Compress the source path into a tar.gz file to the destination path.
 * @param options.includes The name of the file or directory to be compressed.
 * @param options.excludes The name of the file or directory not to be compressed.
 * @example
 * import { compress } from "tauri-plugin-fs-pro-api"
 * await compress("/Users/xxx/EcoPaste", "/Download/xxx/EcoPaste.tar.gz", { excludes: ["file.txt", "dir"] })
 */
export const compress = (
  srcPath: string,
  dstPath: string,
  options?: CompressOptions
) => {
  return invoke(FS_PRO_COMMAND.COMPRESS, {
    srcPath,
    dstPath,
    options,
  });
};

/**
 * Decompress the tar.gz file from the source path to the destination path.
 * @example
 * import { decompress } from "tauri-plugin-fs-pro-api"
 * await decompress("/Download/xxx/EcoPaste.tar.gz", "/Users/xxx/EcoPaste")
 */
export const decompress = (srcPath: string, dstPath: string) => {
  return invoke(FS_PRO_COMMAND.DECOMPRESS, {
    srcPath,
    dstPath,
  });
};

/**
 * Move the source path to the destination path.
 * @param options.includes The name of the file or directory to be moved.
 * @param options.excludes The name of the file or directory not to be moved.
 * @example
 * import { transfer } from "tauri-plugin-fs-pro-api"
 * await transfer("/Users/xxx/EcoPaste", "/Download/xxx/EcoPaste", { excludes: ["file.txt", "dir"] })
 */
export const transfer = (
  srcPath: string,
  dstPath: string,
  options?: TransferOptions
) => {
  return invoke(FS_PRO_COMMAND.TRANSFER, {
    srcPath,
    dstPath,
    options,
  });
};
