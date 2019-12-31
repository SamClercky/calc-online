<script>
    import OpsButtons from "./ops.buttons.svelte";
    import Draggable from "../utils/draggable.svelte";
    import {ops} from "./ops.ts"

    export let className = "";
    const symbols = Object.values(ops);
</script>

<style type="text/sass" lang="scss">
    @import "../global_vars";

    li {
        display: block;
        list-style: none;
        width: min-content;
    }
    ul {
        padding: $PADDING;
    }
</style>

<aside class={className}>
    {#each symbols as symbol}
        <Draggable>
            <OpsButtons {symbol} />
        </Draggable>
    {/each}
</aside>