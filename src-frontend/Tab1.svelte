<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import StreamSpan from "./StreamSpan.svelte";
 import EmailListItem from "./EmailListItem.svelte";
</script>

{#await backend.mailrowidlist() then mailrowidlist}
 {#each mailrowidlist.vec as mailrowid}
  <div class="px-4 sm:px-6 lg:px-8">
   <div
    class="-mx-4 mt-px overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:-mx-6 md:mx-0"
   >
    <table class="min-w-full divide-y divide-gray-300">
     <tbody class="divide-y divide-gray-200 bg-white">
      {#await backend
       .mail(BigInt(mailrowid))
       .then((r) => backend.wasm_mailparse(r)) then result}
       <EmailListItem item={result} />
      {/await}
     </tbody>
    </table>
   </div>
  </div>
 {/each}
{/await}
