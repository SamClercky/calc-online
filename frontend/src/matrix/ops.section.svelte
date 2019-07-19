<script>
    import OpsButtons from "./ops.buttons.svelte";

    const symbols = ["+", "-", "*", "="];
</script>

<style type="text/sass" lang="scss">
    @import "../global_vars";

    li {
        display: block;
        list-style: none;
        padding: 0px;
        margin: 0px;
        width: min-content;
    }
    ul {
        width: 30%;
        max-width: 100px;
        background: $PRIMARY_COLOR_DARK;
        height: 100vh;
        padding: $PADDING;
    }
</style>

<ul>
    {#each symbols as symbol}
        <li><OpsButtons {symbol} /></li>
    {/each}
</ul>