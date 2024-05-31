import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return {
    post: {
      title: `Title for post goes here`,
      content: `Content for post goes here`,
    },
  };
};
