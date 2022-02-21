<script lang="ts">
 import * as backend from "./turbocharger_generated";
 let animal_time_stream = backend.animal_time_stream();
 let stream_example_result = backend.stream_example_result();
</script>

<p class="text-3xl font-bold text-gray-400">
 {#await $animal_time_stream then result}
  {result || ""}
 {:catch error}
  {error}
 {/await}
</p>
<p class="text-3xl font-bold text-gray-400">
 {#await $stream_example_result then result}
  {result || ""}
 {:catch error}
  <p style="color: red">{error.message}</p>
 {/await}
</p>
