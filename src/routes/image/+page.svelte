<script>
  import { convertFileSrc } from "@tauri-apps/api/core";
</script>

<main>
  <h1>Image</h1>

  <p>
    <a href="/">Close</a>
  </p>

  <img src={convertFileSrc("IMG_6360.CR2", "rawimage")} alt={"Image"} />
</main>
