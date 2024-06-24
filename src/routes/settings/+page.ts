import {
  BaseDirectory,
  exists,
  mkdir,
  readFile,
  writeFile,
} from "@tauri-apps/plugin-fs";
import type { PageLoad } from "./$types";

async function writeSettings(settings: object) {
  let encoder = new TextEncoder();
  let data = encoder.encode(JSON.stringify(settings));
  return writeFile("settings.json", data, {
    baseDir: BaseDirectory.AppConfig,
    create: true,
  });
}

async function readSettings() {
  const raw = await readFile("settings.json", {
    baseDir: BaseDirectory.AppConfig,
  });
  const decoder = new TextDecoder();
  const data = decoder.decode(raw);
  const userSettings = JSON.parse(data);
  return {
    ...defaultSettings,
    ...userSettings,
  };
}

const defaultSettings = {
  success: true,
};

export const load: PageLoad = async ({ params }) => {
  const fileExists = await exists("settings.json", {
    baseDir: BaseDirectory.AppConfig,
  });
  if (!fileExists) {
    console.log("Create settings directory");
    await mkdir("", { baseDir: BaseDirectory.AppConfig, recursive: true });
    console.log("Creating settings.json");
    await writeSettings({ success: true });
  }

  const settings = await readSettings();

  return {
    fileExists,
    settings,
  };
};
