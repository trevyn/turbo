<script>
 import * as backend from "./turbocharger_generated";

 let log = "";
 let logNode;

 let foo = backend.animal_time_stream().subscribe((x) => {
  log += x + "\n";
  logNode.scrollTop = logNode.scrollHeight;
 });
</script>

<p class="text-3xl font-bold text-gray-400">ANIMAL LOG</p>
<textarea
 bind:this={logNode}
 style="height:60vh"
 class="mt-3 w-full bg-gray-700 font-mono">{log}</textarea
>
