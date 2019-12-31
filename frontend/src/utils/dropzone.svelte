<script context="module">
    let totalComponents = 0;
</script>

<script>
    import { dragMessage } from "./dragstore";
    import { onDestroy, createEventDispatcher } from "svelte";

    totalComponents += 1;
    const dropzoneref = "dropzone"+totalComponents;

    const dispatch = createEventDispatcher();

    const subscription = dragMessage.subscribe(msg => {
        if (msg.targetZoneRef == dropzoneref) { // enkel en alleen als msg voor ons is
            dispatch("droprequest", {
                dropMessage: msg.dropMessage
            })
        }
    });

    onDestroy(subscription);
</script>

<style type="text/sass" lang="scss">
    @import "../global_vars";

    span {
        margin: 0px;
        padding: 0px;
        width: 100%;
        height: 100%;
    }
</style>

<!-- droppablezone ipv dropzone door nameconflict -->
<span droppablezone="true" dropzoneref="{dropzoneref}">
    <slot></slot>
</span>