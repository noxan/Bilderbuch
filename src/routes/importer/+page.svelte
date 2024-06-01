<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";

  type SystemTime = {
    secs_since_epoch: number;
    nanos: number;
  };

  type Item = {
    path: string;
    metadata: {
      created: SystemTime;
      accessed: SystemTime;
      modified: SystemTime;
    };
    is_directory: boolean;
  };

  let path = "/Volumes";
  let files: Item[] = [];

  function browseDirectory(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.textContent) {
      path = target.textContent;
      listDirectory();
    }
  }

  function navigateTo(target: string) {
    path = target;
    listDirectory();
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

  function filterDisplayFile(path: string) {
    const name = path.toLowerCase();
    return name.endsWith(".jpg") || name.endsWith(".png");
  }
</script>

<main>
  <h1>Import photos</h1>

  <p>
    <a href="/">Cancel</a>
  </p>

  <p>
    Current path: {path}

    <button on:click={() => navigateTo("/Users")}>Home</button>
    <button on:click={() => navigateTo("/Volumes")}>Volumes</button>

    {#if path !== "/Volumes" && path !== "/Users"}
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
          {#if filterDisplayFile(file.path)}
            <img
              src={convertFileSrc(file.path)}
              loading="lazy"
              alt={file.path}
              height="100"
              width="100"
            />
          {/if}
          {new Date(file.metadata.created.secs_since_epoch * 1000)}
        {/if}
      </li>
    {/each}
  </ul>
</main>
