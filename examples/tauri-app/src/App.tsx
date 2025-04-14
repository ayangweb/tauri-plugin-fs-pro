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
  parentName,
  Metadata,
} from "tauri-plugin-fs-pro-api";
import { useReactive } from "ahooks";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { filesize } from "filesize";
import { convertFileSrc } from "@tauri-apps/api/core";

const App = () => {
  const state = useReactive<Partial<Metadata & { icon: string }>>({});

  const handleSelect = async (directory = false) => {
    const path = await openDialog({
      directory,
    });

    if (!path) return;

    Object.assign(state, {
      path,
      icon: convertFileSrc(await icon(path, 512)),
      isExist: await isExist(path),
      isDir: await isDir(path),
      isFile: await isFile(path),
      size: filesize(await size(path), { standard: "jedec" }),
      name: await name(path),
      extname: await extname(path),
      fullName: await fullName(path),
      parentName: await parentName(path),
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
          </List>
        </>
      )}
    </Flex>
  );
};

export default App;
