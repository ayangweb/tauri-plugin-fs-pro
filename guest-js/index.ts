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

export interface OpenOptions {
  /**
   * Whether to open in Folder Explorer.
   */
  explorer?: boolean;
  /**
   * If the path is a directory, does it go directly to the directory.
   */
  enterDir?: boolean;
}

export const FS_PRO_COMMAND = {
  IS_EXIST: "plugin:fs-pro|is_exist",
  IS_DIR: "plugin:fs-pro|is_dir",
  IS_FILE: "plugin:fs-pro|is_file",
  SIZE: "plugin:fs-pro|size",
  NAME: "plugin:fs-pro|name",
  EXTNAME: "plugin:fs-pro|extname",
  METADATA: "plugin:fs-pro|metadata",
  OPEN: "plugin:fs-pro|open",
};

/**
 * Whether the path exists.
 * @example
 * import { isExist } from "tauri-plugin-fs-pro-api"
 * const yes = await isExist("/Users/xxx/EcoPaste.txt")
 * console.log(yes) // true
 */
export const isExist = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_EXIST, { path });
};

/**
 * Whether the path is a directory.
 * @example
 * import { isDir } from "tauri-plugin-fs-pro-api"
 * const yes = await isDir("/Users/xxx/EcoPaste")
 * console.log(yes) // true
 */
export const isDir = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_DIR, { path });
};

/**
 * Whether the path is a file.
 * @example
 * import { isFile } from "tauri-plugin-fs-pro-api"
 * const yes = await isFile("/Users/xxx/EcoPaste.txt")
 * console.log(yes) // true
 */
export const isFile = (path: string) => {
  return invoke<boolean>(FS_PRO_COMMAND.IS_FILE, { path });
};

/**
 * Get the size of the path in bytes, or 0 if it does not exist.
 * @example
 * import { size } from "tauri-plugin-fs-pro-api"
 * const fileSize = await size("/Users/xxx/EcoPaste.txt")
 * console.log(fileSize) // 1024
 */
export const size = (path: string) => {
  return invoke<number>(FS_PRO_COMMAND.SIZE, { path });
};

/**
 * Get the file or directory name of the path.
 * @example
 * import { name } from "tauri-plugin-fs-pro-api"
 * const fileName = await name("/Users/xxx/EcoPaste.txt")
 * console.log(fileName) // "EcoPaste"
 */
export const name = (path: string) => {
  return invoke<string>(FS_PRO_COMMAND.NAME, { path });
};

/**
 * Get the extension name of the file.
 * @example
 * import { extname } from "tauri-plugin-fs-pro-api"
 * const ext = await extname("/Users/xxx/EcoPaste.txt")
 * console.log(ext) // "txt"
 */
export const extname = (path: string) => {
  return invoke<string>(FS_PRO_COMMAND.EXTNAME, { path });
};

/**
 * Get the metadata of the path.
 * @example
 * import { metadata } from "tauri-plugin-fs-pro-api"
 * const { isFile } = await metadata("/Users/xxx/EcoPaste.txt")
 * console.log(isFile) // true
 * @returns return {@linkcode Metadata}
 */
export const metadata = (path: string) => {
  return invoke<Metadata>(FS_PRO_COMMAND.METADATA, { path });
};

/**
 * Open the path in File Explorer or the default application.
 * @param options.explorer Whether to open in Folder Explorer
 * Defaults to `false`.
 * @param options.enterDir If the path is a directory, does it go directly to the directory
 * Defaults to `false`.
 * @example
 * import { open } from "tauri-plugin-fs-pro-api"
 * await open("/Users/xxx/EcoPaste.txt")
 */
export const open = (path: string, options?: OpenOptions) => {
  return invoke(FS_PRO_COMMAND.OPEN, { path, options });
};
