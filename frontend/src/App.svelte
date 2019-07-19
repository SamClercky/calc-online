<script>
	import Matrix from "./matrix/index.svelte";

	import * as wasm from "calc-online";
	export let name;

	function onclick() {
		wasm.greet("Andreas");
	}
</script>

<style type="text/sass">
	h1 {
		color: blue;
	}
</style>

<svelte:head>
	<title>Calc online</title>
	<link rel="stylesheet" type="text/css" href="https://unpkg.com/normalize.css@8.0.1/normalize.css" />
</svelte:head>

<h1 on:click={onclick}>Hello {name}!</h1>
<Matrix />