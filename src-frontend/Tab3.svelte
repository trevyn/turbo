<script lang="ts">
 import * as backend from "./turbocharger_generated";
</script>

{#await backend.animal_log() then info}
 <pre class="text-xs pt-1">{info}</pre>
{:catch error}
 {error}
{/await}
