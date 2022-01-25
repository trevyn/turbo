<script>
 import * as backend from "./turbocharger_generated";
 import { fade } from "svelte/transition";

 (async () => {
  // let person = Object.assign(new backend.Person(), { name: "Bob" });
  // let person2 = new backend.Person({ name: "bob" });
  // let rowid = await backend.insert_person(person);
  // console.log("Inserted rowid ", rowid);
 })();
</script>

<div class="p-5">
 {#await backend.check_for_updates()}Checking for updates...{:then info}{info}{:catch error}Error
  checking for updates: {error}{/await}
 <!-- <button on:click={async () => console.log(await backend.check_for_updates())}
  >Check for updates</button
 > -->
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
