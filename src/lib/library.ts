import { path } from "@tauri-apps/api";
import { BaseDirectory, pictureDir } from "@tauri-apps/api/path";
import { exists, mkdir } from "@tauri-apps/plugin-fs";

const pictureDirectory = await pictureDir();

export const libraryDirectory = path.join(pictureDirectory, "Bilderbuch");

export async function ensureLibraryDirectoryExists() {
  const fileExists = await exists("Bilderbuch", {
    baseDir: BaseDirectory.Picture,
  });
  if (!fileExists) {
    await mkdir("Bilderbuch", {
      baseDir: BaseDirectory.Picture,
      recursive: true,
    });
  }
}
