import { Button, Flex, Image, List } from "antd";
import {
  isExist,
  isDir,
  isFile,
  size,
  name,
  fullName,
  extname,
  icon,
  metadata,
  open,
} from "tauri-plugin-fs-pro-api";
import { useReactive } from "ahooks";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { filesize } from "filesize";

const App = () => {
  const state = useReactive({
    path: "",
    isExist: false,
    isDir: false,
    isFile: false,
    name: "",
    extname: "",
    metadata: "",
  });

  const handleSelect = async (directory = false) => {
    const path = await openDialog({
      directory,
    });

    if (!path) return;

    const bytes = await icon(path, 512);
    const blob = new Blob([bytes], { type: "image/png" });
    const url = URL.createObjectURL(blob);

    Object.assign(state, {
      path,
      icon: url,
      isExist: await isExist(path),
      isDir: await isDir(path),
      isFile: await isFile(path),
      size: filesize(await size(path), { standard: "jedec" }),
      name: await name(path),
      fullName: await fullName(path),
      extname: await extname(path),
      metadata: await metadata(path),
    });
  };

  return (
    <Flex vertical align="start" gap="middle">
      <Flex gap="middle">
        <Button onClick={() => handleSelect()}>Select file</Button>
        <Button onClick={() => handleSelect(true)}>Select directory</Button>
      </Flex>

      {state.isExist && (
        <>
          <List bordered>
            {Object.entries(state).map(([key, value]) => {
              return (
                <List.Item key={key}>
                  <span style={{ minWidth: 100 }}>{key}</span>

                  {key === "icon" ? (
                    <Image width={50} src={value as string} />
                  ) : (
                    <span>
                      {typeof value === "string"
                        ? value
                        : JSON.stringify(value, null, 2)}
                    </span>
                  )}
                </List.Item>
              );
            })}

            <List.Item>
              <span>open</span>
              <Flex gap="middle">
                <Button
                  onClick={() => {
                    open(state.path);
                  }}
                >
                  Default Application
                </Button>
                <Button
                  onClick={() => {
                    open(state.path, {
                      explorer: true,
                    });
                  }}
                >
                  File Explorer
                </Button>
                {state.isDir && (
                  <Button
                    onClick={() => {
                      open(state.path, {
                        explorer: true,
                        enterDir: true,
                      });
                    }}
                  >
                    Enter Directory
                  </Button>
                )}
              </Flex>
            </List.Item>
          </List>
        </>
      )}
    </Flex>
  );
};

export default App;
