<script>
    import OpsButtons from "./ops.buttons.svelte";

    export let className = "";

    const symbols = ["add", "remove", "clear", "="];
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

<section class={className}>
    {#each symbols as symbol}
        <OpsButtons {symbol} />
    {/each}
</section>