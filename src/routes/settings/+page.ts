import type { PageLoad } from "./$types";

import { BaseDirectory, exists } from '@tauri-apps/plugin-fs';


export const load: PageLoad = async ({ params }) => {
  const fileExists = await exists('avatar.png', { baseDir: BaseDirectory.AppConfig });

  return {
    fileExists,
    post: {
      title: `Title for post goes here`,
      content: `Content for post goes here`,
    },
  };
};
