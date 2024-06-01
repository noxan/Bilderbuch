<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<main>
  <h1>Settings</h1>

  <a href="/">Close</a>

  {JSON.stringify(data)}
</main>
