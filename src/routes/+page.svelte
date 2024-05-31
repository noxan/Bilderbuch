<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Item = {
    path: string;
    is_directory: boolean;
  };

  let path = "/Volumes/EOS_DIGITAL";
  let files: Item[] = [];

  function browseDirectory(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.textContent) {
      path = target.textContent;
      listDirectory();
    }
  }

  function navigateUp() {
    path = path.split("/").slice(0, -1).join("/");
    listDirectory();
  }

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  async function listDirectory() {
    files = await invoke("list_directory", { path });
  }
  listDirectory();
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>

  <p>
    Current path: {path}

    {#if path !== "/Volumes"}
      <button on:click={navigateUp}>Go up</button>
    {/if}
  </p>

  <ul>
    {#each files as file}
      <li>
        {#if file.is_directory}
          <button on:click={browseDirectory}>
            {file.path}
          </button>
        {:else}
          {file.path}
        {/if}
      </li>
    {/each}
  </ul>
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
