<script>
 export let span;
 export let decrypt = false;
 import * as backend from "./turbocharger_generated";
</script>

{#await span then result}
 {(decrypt ? "encryption not supported" || result : result) || ""}
{:catch error}
 <span style="color: red">{error.message}</span>
{/await}
