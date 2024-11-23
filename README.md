# tauri-plugin-fs-pro

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Extended support for file and directory operations.

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
