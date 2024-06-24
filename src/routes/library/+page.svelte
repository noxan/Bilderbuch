<script lang="ts">
  import type { PageData } from "./$types";
  export let data: PageData;
</script>

<main>
  <h1>Library</h1>

  <p>
    <a href="/">Cancel</a>
  </p>

  <pre><code>{JSON.stringify(data, null, 2)}</code></pre>
</main>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }
</style>
