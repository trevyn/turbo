<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import StreamSpan from "./StreamSpan.svelte";
 import Span from "./Span.svelte";
</script>

<p class="break-words text-3xl font-bold text-gray-400">
 <StreamSpan decrypt={true} stream={backend.encrypted_animal_time_stream()} />
</p>
<p class="break-words text-xl font-bold text-gray-400">
 <Span decrypt={true} span={backend.mail(BigInt(3))} />
</p>
<p class="text-3xl font-bold text-gray-400">
 <!-- <StreamSpan stream={backend.stream_example_result()} /> -->
</p>
