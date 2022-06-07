<!-- <script lang="ts">
 import * as backend from "./turbocharger_generated";

 let sk = backend.wasm_client_sk();
 let verified_sk;
 $: {
  verified_sk = backend.wasm_client_sk();
  sk = sk;
 }
</script>

<p>{verified_sk}</p>

<input
 class="font-mono text-gray-600"
 type="text"
 bind:value={sk}
 size="64"
 on:input={() => backend.wasm_set_client_sk(sk)}
/>

{#await backend.animal_log() then info}
 <pre class="text-xs pt-1">{info}</pre>
{:catch error}
 {error}
{/await} -->
