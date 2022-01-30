<script>
 import * as backend from "./turbocharger_generated";
 import { fade } from "svelte/transition";

 (async () => {
  // let person = Object.assign(new backend.Person(), { name: "Bob" });
  // let person2 = new backend.Person({ name: "bob" });
  // let rowid = await backend.insert_person(person);
  // console.log("Inserted rowid ", rowid);
 })();

 let animal_time_stream = backend.animal_time_stream();
</script>

<div class="p-5">
 <p class="text-3xl text-purple-500 font-bold">{$animal_time_stream}</p>
 <p>
  {#await backend.check_for_updates()}Checking for updates...{:then info}{info}{:catch error}Error
   checking for updates: {error}{/await}
 </p>
 <button
  on:click={() => alert("POOSHED")}
  class="bg-lime-400 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
 >
  POOSH ME
 </button>
 <p>
  You find yourself in a dark room. You can barely make out a blockchain in the
  corner.
 </p>
 <p in:fade={{ delay: 1000, duration: 2000 }}>"Psst," it says.</p>
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
