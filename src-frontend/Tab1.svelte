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

<div class="px-4 sm:px-6 lg:px-8">
 <div
  class="-mx-4 mt-8 overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:-mx-6 md:mx-0 md:rounded-lg"
 >
  <table class="min-w-full divide-y divide-gray-300">
   <tbody class="divide-y divide-gray-200 bg-white">
    <tr>
     <td
      class="w-full max-w-0 py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:w-auto sm:max-w-none sm:pl-6"
     >
      Lindsay Walton
      <dl class="font-normal lg:hidden">
       <dt class="sr-only">Title</dt>
       <dd class="mt-1 truncate text-gray-700">Front-end Developer</dd>
       <dt class="sr-only sm:hidden">Email</dt>
       <dd class="mt-1 truncate text-gray-500 sm:hidden">
        lindsay.walton@example.com
       </dd>
      </dl>
     </td>
     <td class="hidden px-3 py-4 text-sm text-gray-500 lg:table-cell"
      >Front-end Developer</td
     >
     <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell"
      >lindsay.walton@example.com</td
     >
     <td class="px-3 py-4 text-sm text-gray-500">Member</td>
    </tr>

    <!-- More people... -->
   </tbody>
  </table>
 </div>
</div>
