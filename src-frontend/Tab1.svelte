<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import StreamSpan from "./StreamSpan.svelte";
</script>

<p class="break-words text-3xl font-bold text-gray-400">
 <StreamSpan decrypt={true} stream={backend.encrypted_animal_time_stream()} />
</p>
<p class="break-words text-xl font-bold text-gray-400">
 {#await backend
  .mail(BigInt(3))
  .then((r) => backend.wasm_mailparse(r)) then result}
  From: {result.from}<br />
  To: {result.to}<br />
  Subject: {result.subject}<br /><br />
  {result.body}<br />
 {:catch error}
  <span style="color: red">{error.message}</span>
 {/await}
</p>
<p class="text-3xl font-bold text-gray-400">
 <!-- <StreamSpan stream={backend.stream_example_result()} /> -->
</p>
