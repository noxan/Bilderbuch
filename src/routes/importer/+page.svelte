<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";

  function lazyImage(node: HTMLElement, source: string) {
    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) {
        node.style.backgroundImage = `url(${source})`;
        observer.unobserve(node);
      }
    });
    observer.observe(node);

    return {
      destroy() {
        observer.unobserve(node);
      },
    };
  }

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

  async function navigateTo(target: string) {
    await listDirectory(target);
    path = target;
  }

  async function navigateUp() {
    const target = path.split("/").slice(0, -1).join("/");
    await listDirectory(target);
    path = target;
  }

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  async function listDirectory(target: string) {
    try {
      files = await invoke("list_directory", { path: target });
    } catch (error) {
      // TODO: Notify user about the error
    }
  }
  listDirectory(path);

  function buildImageSource(path: string) {
    const extension = path.split(".").pop()?.toLocaleLowerCase();
    switch (extension) {
      case "jpg":
      case "jpeg":
      case "png":
        return convertFileSrc(path);
      case "cr2":
      case "arw":
        return convertFileSrc(path, "preview");
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
    {#each files as file (file.path)}
      <div>
        {#if file.is_directory}
          <button on:click={() => navigateTo(file.path)}>
            {file.name}
          </button>
        {:else}
          <div>
            {file.name}
          </div>
          <div class="image" use:lazyImage={buildImageSource(file.path)}></div>
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
