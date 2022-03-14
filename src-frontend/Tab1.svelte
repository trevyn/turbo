<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import StreamSpan from "./StreamSpan.svelte";
</script>

<p class="break-words text-3xl font-bold text-gray-400">
 <StreamSpan decrypt={true} stream={backend.encrypted_animal_time_stream()} />
</p>
<p class="text-3xl font-bold text-gray-400">
 <!-- <StreamSpan stream={backend.stream_example_result()} /> -->
</p>
