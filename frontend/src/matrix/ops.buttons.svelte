<script>
  export let symbol = "+";
</script>

<style type="text/sass" lang="scss">
  @import "../global_vars";
  @import "@material/button/mdc-button";
  @import "@material/ripple/mdc-ripple";

  .button-ops {
    margin: $MARGIN;
    @include mdc-button-ink-color($PRIMARY_COLOR);
    @include mdc-button-container-fill-color($PRIMARY_COLOR_DARK)
    @include mdc-states($PRIMARY_COLOR);
    @include mdc-ripple-surface;
    @include mdc-ripple-radius-bounded;
    @include mdc-states;
    @include mdc-states-base-color($PRIMARY_COLOR);
  }
</style>

<button class="button-ops mdc-button">
  <!-- <span class="mdc-button__label">{symbol}</span> -->
  <i class="material-icons">{symbol}</i>
</button>
