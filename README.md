# tauri-plugin-fs-pro

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

**What is the difference with `@tauri-apps/plugin-fs`?**

The `stat` method of `@tauri-apps/plugin-fs` supports only files when getting size, while the `metadata` method of `tauri-plugin-fs-pro` supports both files and directories when getting size. And some additional [methods](#methods) are provided. More methods will be expanded in the future.

https://github.com/user-attachments/assets/33dff210-9962-4d73-9648-5227d8fd9519

## Platform Support

| Platform | Supported |
| -------- | --------- |
| Windows  | ✅        |
| macOS    | ✅        |
| Linux    | ✅        |
| Android  | ❌        |
| iOS      | ❌        |

## Install

```shell
cargo add tauri-plugin-fs-pro
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```shell
pnpm add tauri-plugin-fs-pro-api
```

## Usage

`src-tauri/src/lib.rs`

```diff
pub fn run() {
    tauri::Builder::default()
+       .plugin(tauri_plugin_fs_pro::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/capabilities/default.json`

```diff
{
    ...
    "permissions": [
        ...
+       "fs-pro:default"
    ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import { isExist } from "tauri-plugin-fs-pro-api";

const exists = await isExist("/Users/xxx/EcoPaste.txt");
console.log(exists); // true
```

## Methods

| Method       | Description                                                                               |
| ------------ | ----------------------------------------------------------------------------------------- |
| `isExist`    | Whether the path exists.                                                                  |
| `isDir`      | Whether the path is a directory.                                                          |
| `isFile`     | Whether the path is a file.                                                               |
| `size`       | Get the size of the path in bytes, or 0 if it does not exist.                             |
| `name`       | Get the file or directory name of the path.                                               |
| `fullName`   | Get the file or directory name of the path, including the extension name if it is a file. |
| `extname`    | Get the extension name of the file.                                                       |
| `icon`       | Get the system icon of the path.                                                          |
| `metadata`   | Get the metadata of the path.                                                             |
| `open`       | Open the path in file explorer or the default application.                                |
| `compress`   | Compress the source path into a tar.gz file to the destination path.                      |
| `decompress` | Decompress the tar.gz file from the source path to the destination path.                  |
| `transfer`   | Move the source path to the destination path.                                             |

## Example

```shell
git clone https://github.com/ayangweb/tauri-plugin-fs-pro.git
```

```shell
pnpm install

pnpm build

cd examples/tauri-app

pnpm install

pnpm tauri dev
```

## Thanks

- Use [file_icon_provider](https://github.com/IohannRabeson/file_icon_provider) and [image](https://github.com/image-rs/image) to get the system icon for a path.

- Use [open](https://github.com/Byron/open-rs) to open the path in default application.

- Use [showfile](https://github.com/jf2048/showfile) to open the path in file explorer.

- Use [flate2](https://github.com/rust-lang/flate2-rs) and [tar](https://github.com/alexcrichton/tar-rs) to compress and decompress tar.gz.

- Use [fs_extra](https://github.com/webdesus/fs_extra) to implement the move path.

## Who's Use It

- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Open source cross-platform clipboard management tool.

- [Coco AI](https://github.com/infinilabs/coco-app) - Search, Connect, Collaborate, Your Personal AI Search and Assistant, all in one space.
