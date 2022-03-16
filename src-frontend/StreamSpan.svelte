<script>
 export let stream;
 export let decrypt = false;
 import * as backend from "./turbocharger_generated";
</script>

{#await $stream then result}
 {(decrypt ? backend.wasm_decrypt(result || "") || result : result) || ""}
{:catch error}
 <span style="color: red">{error.message}</span>
{/await}
