<script lang="ts">
 import * as backend from "./turbocharger_generated";

 let log = "";
 let logNode;

 backend.animal_time_stream().subscribe(async (line) => {
  log += (await line) + "\n";
  logNode.scrollTop = logNode.scrollHeight;
 });
</script>

<p class="text-3xl font-bold text-gray-400">ANIMAL LOG</p>
<textarea
 bind:this={logNode}
 style="height:60vh"
 class="mt-3 w-full bg-gray-700 p-4 pb-16 font-mono">{log}</textarea
>
