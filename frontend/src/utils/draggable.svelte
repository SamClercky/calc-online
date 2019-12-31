<script>
  import { onMount } from "svelte";
  import { getElementOffset } from "./offset";
  import { dragMessage } from "./dragstore";

  export let targetZone = null; // Doel zone
  export let dropMessage = null; // Bericht dat naar de dropzone gaat

  let isDragging = false;
  let container;

  // Function to get only targetZones
  function getDropzone(elementList) {
    const searchableAttr = "droppablezone";
    if (targetZone == null) {
      return elementList.filter(element => {
        return element.getAttribute(searchableAttr) != null;
      });
    } else {
      return elementList.filter(element => {
        // filter all the non-Dropzones
        return element.getAttribute(searchableAttr) == targetZone;
      });
    }
  }

  function sendDropRequest(msg, element) {
    dragMessage.set({
        targetZoneRef: element.getAttribute("dropzoneref"),
        dropMessage: msg,
    });
  }

  function onTouchStart(ev) {
    // console.log(ev);
    isDragging = true;
  }

  function onTouchMove(ev) {
    // console.log(ev);
    container.style.left = ev.touches[0].clientX + "px";
    container.style.top = ev.touches[0].clientY + "px";
  }

  function onTouchEnd(ev) {
    console.log(ev);
    const position = getElementOffset(container);
    const zones = getDropzone(
      document.elementsFromPoint(position.x, position.y)
    );

    // add draggable element to Dropzone
    if (zones.length > 0) {
      sendDropRequest(dropMessage, zones[0]);
    }

    // zorg dat het element terug naar de oorsprong gaat
    isDragging = false;
  }

  onMount(() => {});
</script>

<style type="text/sass" lang="scss">
  @import "../global_vars";

  span {
    margin: 0px;
    padding: 0px;
    width: fit-content;
    height: fit-content;
  }

  .dragging {
    position: absolute;
    z-index: 1000;
  }
</style>

<span
  bind:this={container}
  draggable="true"
  on:touchstart={onTouchStart}
  on:touchmove={onTouchMove}
  on:touchend={onTouchEnd}
  class={isDragging ? 'dragging' : ''}>
  <slot />
</span>
