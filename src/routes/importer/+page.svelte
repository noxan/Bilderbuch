<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";

  type SystemTime = {
    secs_since_epoch: number;
    nanos: number;
  };

  type Item = {
    name: string;
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

  function buildImageSource(path: string) {
    switch (path.toLowerCase()) {
      case ".jpg":
      case ".jpeg":
      case ".png":
        return convertFileSrc(path);
      default:
        return "";
    }
  }

  function displayDate(time: SystemTime) {
    return new Date(time.secs_since_epoch * 1000).toDateString();
  }
</script>

<main>
  <h1>Import photos</h1>

  <p>
    <a href="/">Cancel</a>
  </p>

  <p>
    <button on:click={() => navigateTo("/Users")}>Home</button>
    <button on:click={() => navigateTo("/Volumes")}>Volumes</button>
  </p>

  <p>
    Current path: {path}

    {#if path !== "/Volumes" && path !== "/Users"}
      <button on:click={navigateUp}>Go up</button>
    {/if}
  </p>

  <div class="grid">
    {#each files as file}
      <div>
        {#if file.is_directory}
          <button on:click={() => navigateTo(file.path)}>
            {file.name}
          </button>
        {:else}
          <div>
            {file.name}
          </div>
          <div
            class="image"
            style={`background-image: url(${buildImageSource(file.path)});`}
          />
          <small>
            {displayDate(file.metadata.created) + " "}
          </small>
        {/if}
      </div>
    {/each}
  </div>
</main>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }

  .image {
    width: 200px;
    height: 200px;
    background-color: lightgrey;
    background-size: cover;
    background-position: center;
  }
</style>
