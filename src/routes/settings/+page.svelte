<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<main>
  <h1>Settings</h1>

  <p>
    <a href="/">Close</a>
  </p>

  <pre><code>{JSON.stringify(data, null, 2)}</code></pre>
</main>
