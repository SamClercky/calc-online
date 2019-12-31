<script>
	import Matrix from "./matrix/index.svelte";

	// import * as wasm from "calc-online";
	// export let name;

	// function onclick() {
	// 	wasm.greet("Andreas");
	// }
</script>

<style type="text/sass">
	div {
		width: 100vw;
		height: 90vh;
	}
	:global(.app__matrix) {
		width: 100%;
		height: 100%;
	}
</style>

<svelte:head>
</svelte:head>

<div>
	<Matrix className="app__matrix" />
</div>