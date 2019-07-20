<script>
  import MatrixSection from "./matrix.section.svelte";
  import CalcSection from "./calc.section.svelte";
  import OpsSection from "./ops.section.svelte";

  export let className = "";
</script>

<style type="text/scss" lang="scss">
  @import "../global_vars.scss";

  div {
    display: grid;
    grid-template-columns: minmax(250px, 30%) auto;
    grid-template-rows: 9fr 1fr;
    grid-template-areas:
        "matrix__matrix-section matrix__calc-section"
        "matrix__ops-section matrix__ops-section";
  }
  @media screen (min-width: 900px) {
    div {
      grid-template-columns: minmax(250px, 30%) auto minmax(100px, 10%);
      grid-template-rows: 100%;
      grid-template-areas: "matrix__matrix-section matrix__calc-section matrix__ops-section";
    }
  }

  :global(.matrix__matrix-section) {
    background: red;
    grid-area: matrix__matrix-section;
  }
  :global(.matrix__calc-section) {
    background: blue;
    grid-area: matrix__calc-section;
  }
  :global(.matrix__ops-section) {
    background: green;
    grid-area: matrix__ops-section;
    margin: 0px;
  }
</style>

<div class={className}>
  <MatrixSection className="matrix__matrix-section" />
  <CalcSection className="matrix__calc-section" />
  <OpsSection className="matrix__ops-section" />
</div>
