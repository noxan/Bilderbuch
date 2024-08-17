import { pictureDir } from "@tauri-apps/api/path";
import { BaseDirectory, exists, mkdir } from "@tauri-apps/plugin-fs";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  const fileExists = await exists("", { baseDir: BaseDirectory.Picture });
  if (!fileExists) {
    await mkdir("", { baseDir: BaseDirectory.Picture, recursive: true });
  }
  const libraryDirectory = await pictureDir();

  return {
    libraryDirectory,
  };
};
