# tauri-plugin-fs-pro

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

**What is the difference with `@tauri-apps/plugin-fs`?**

The `stat` method of `@tauri-apps/plugin-fs` supports only files when getting size, while the `metadata` method of `tauri-plugin-fs-pro` supports both files and directories when getting size. And some additional [methods](#methods) are provided. More methods will be expanded in the future.

https://github.com/user-attachments/assets/90207b4e-34d2-45ea-acda-c3bfc6ca7caf

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

const yes = await isExist("/Users/xxx/EcoPaste.txt");
console.log(yes); // true
```

## Methods

| Method     | Description                                                                               |
| ---------- | ----------------------------------------------------------------------------------------- |
| `isExist`  | Whether the path exists.                                                                  |
| `isDir`    | Whether the path is a directory.                                                          |
| `isFile`   | Whether the path is a file.                                                               |
| `size`     | Get the size of the path in bytes, or 0 if it does not exist.                             |
| `name`     | Get the file or directory name of the path.                                               |
| `fullName` | Get the file or directory name of the path, including the extension name if it is a file. |
| `extname`  | Get the extension name of the file.                                                       |
| `metadata` | Get the metadata of the path.                                                             |
| `open`     | Open the path in File Explorer or the default application.                                |

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
