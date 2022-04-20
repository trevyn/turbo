<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import StreamSpan from "./StreamSpan.svelte";

 let log = "";
 let logNode: HTMLTextAreaElement;

 backend.animal_time_stream().subscribe(async (line) => {
  log += (await line) + "\n";
  logNode.scrollTop = logNode.scrollHeight;
 });
</script>

<p class="break-words text-3xl font-bold text-gray-400">
 <StreamSpan stream={backend.animal_time_stream()} />
</p>
<p class="text-3xl font-bold text-gray-400">
 <StreamSpan stream={backend.stream_example_result()} />
</p>

<p class="text-3xl font-bold text-gray-400">ANIMAL LOG</p>
<textarea
 bind:this={logNode}
 style="height:60vh"
 class="mt-3 w-full bg-gray-700 p-4 pb-16 font-mono outline-none"
 >{log}</textarea
>
