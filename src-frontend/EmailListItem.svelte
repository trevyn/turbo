<script lang="ts">
 import * as backend from "./turbocharger_generated";
 export let item: backend.ParsedMail;
</script>

<tr>
 <td
  class="w-full max-w-0 py-4 pl-4 pr-3 text-sm font-extralight text-gray-900 sm:w-auto sm:max-w-none sm:pl-6"
 >
  {item.from}
  <dl class="lg:hidden">
   <dd class="mt-1 truncate font-normal text-gray-700">{item.subject}</dd>
   <dd class="mt-1 truncate font-light text-gray-500 sm:hidden">
    {item.body}
   </dd>
  </dl>
 </td>
 <td class="hidden px-3 py-4 text-sm text-gray-500 lg:table-cell"
  >{item.subject}</td
 >
 <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell"
  >{item.body}</td
 >
 <!-- <td class="px-3 py-4 text-sm text-gray-500">Hello</td> -->
</tr>
