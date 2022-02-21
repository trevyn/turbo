<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import { fade } from "svelte/transition";
 import Tab1 from "./Tab1.svelte";
 import Tab2 from "./Tab2.svelte";

 (async () => {
  // let person = Object.assign(new backend.Person(), { name: "Bob" });
  // let person2 = new backend.Person({ name: "bob" });
  // let rowid = await backend.insert_person(person);
  // console.log("Inserted rowid ", rowid);
 })();

 let tab = 1;
</script>

<div class="p-5">
 <p>
  {#await backend.check_for_updates()}Checking for updates...{:then info}{info}{:catch error}Error
   checking for updates: {error}{/await}
 </p>

 <p class="pt-3">
  <button
   on:click={() => {
    tab = 1;
   }}
   class="rounded bg-gray-400 py-2 px-4 font-bold text-white hover:bg-gray-500"
  >
   ANIMAL TIME
  </button>

  <button
   on:click={() => {
    tab = 2;
   }}
   class="rounded bg-gray-400 py-2 px-4 font-bold text-white hover:bg-gray-500"
  >
   ANIMAL LOG
  </button>
 </p>

 <p class="pt-3">
  {#if tab == 1}
   <Tab1 />
  {/if}

  {#if tab == 2}
   <Tab2 />
  {/if}
 </p>

 <!-- <p>
  You find yourself in a dark room. You can barely make out a chain in the
  corner.
 </p>
 <p in:fade={{ delay: 1000, duration: 2000 }}>"Psst," it says.</p> -->
 <!-- <p in:fade={{ delay: 4000, duration: 2000 }}>
  "My secret key is
  {#await backend.get_new_secret_key()}...{:then info}{info}{:catch error}...
   actually, I'm not really sure. {error}{/await}."
 </p> -->
 <!-- <p>
  "My height is
  {#await backend.getblockchaininfo()}...{:then info}{JSON.parse(info).result
    .blocks}{:catch error}... actually, I'm not really sure. {error}{/await}."
 </p> -->
 <!-- <p in:fade={{ delay: 7000, duration: 2000 }}>
  "Are you
  {#await backend.get_person(1n)}...{:then person}{person.name}{:catch error}{error}{/await}?"
 </p> -->
</div>
