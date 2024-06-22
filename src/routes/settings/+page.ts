import type { PageLoad } from "./$types";

import { BaseDirectory, exists } from '@tauri-apps/plugin-fs';


export const load: PageLoad = async ({ params }) => {
  // Check if the `$APPDATA/avatar.png` file exists
  const exists = await exists('avatar.png', { baseDir: BaseDirectory.AppData });

  return {
    exists,
    post: {
      title: `Title for post goes here`,
      content: `Content for post goes here`,
    },
  };
};
