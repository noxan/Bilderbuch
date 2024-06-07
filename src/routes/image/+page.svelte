<script>
  import { convertFileSrc } from "@tauri-apps/api/core";
</script>

<main>
  <h1>Image</h1>

  <p>
    <a href="/">Close</a>
  </p>

  <img src={convertFileSrc("IMG_6360.CR2", "preview")} alt={"Image"} />
  <img src={convertFileSrc("DSC_0702.NEF", "preview")} alt={"Image"} />
</main>
