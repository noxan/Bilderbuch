import { appConfigDir, resolve } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";

import { BaseDirectory, exists, mkdir, writeFile } from '@tauri-apps/plugin-fs';


export const load: PageLoad = async ({ params }) => {
  const fileExists = await exists('settings.json', { baseDir: BaseDirectory.AppConfig });
  if (!fileExists) {
    console.log('Create settings directory')
    const settingsDirectory = await appConfigDir();
    await mkdir(settingsDirectory, { recursive: true })
    console.log('Creating settings.json');
    let encoder = new TextEncoder();
    let data = encoder.encode(JSON.stringify({ success: true }));
    await writeFile('settings.json', data, { baseDir: BaseDirectory.AppConfig, create: true });
  }

  return {
    fileExists,
    post: {
      title: `Title for post goes here`,
      content: `Content for post goes here`,
    },
  };
};
