<script>
  import DropZone from "../utils/dropzone.svelte";
  import Button from "./ops.buttons.svelte";

  export let className = "";

  let dropzone;

  function addButton(msg) {
    new Button({
      target: dropzone,
      props: {
        symbol: msg.dropMessage
      }
    });
  }
</script>

<style type="text/sass" lang="scss">
  @import "../global_vars";

  main {
      display: flex; /* Zorg dat DragZone alle plaats in neemt */
  }
</style>

<main class={className}>
  <DropZone bind:this={dropzone} on:droprequest="{addButton}">Calc section</DropZone>
</main>
