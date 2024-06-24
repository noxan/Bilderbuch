import { appConfigDir, resolve } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";

import { BaseDirectory, exists, mkdir, writeFile } from '@tauri-apps/plugin-fs';

async function writeSettings(settings: object) {
  let encoder = new TextEncoder();
  let data = encoder.encode(JSON.stringify(settings));
  return writeFile('settings.json', data, { baseDir: BaseDirectory.AppConfig, create: true });
}


export const load: PageLoad = async ({ params }) => {
  const fileExists = await exists('settings.json', { baseDir: BaseDirectory.AppConfig });
  if (!fileExists) {
    console.log('Create settings directory')
    await mkdir('', {baseDir: BaseDirectory.AppConfig, recursive: true })
    console.log('Creating settings.json');
    await writeSettings({ success: true });
  }

  return {
    fileExists,
    post: {
      title: `Title for post goes here`,
      content: `Content for post goes here`,
    },
  };
};
