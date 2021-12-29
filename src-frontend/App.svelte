<script>
 import * as backend from "./turbocharger_generated";

 (async () => {
  let person = Object.assign(new backend.Person(), { name: "Bob" });

  // let person2 = new backend.Person({ name: "bob" });

  let rowid = await backend.insert_person(person);
  console.log("Inserted rowid ", rowid);
 })();
</script>

<div class="p-5">
 <p>
  You find yourself in a dark room. You can barely make out a blockchain in the
  corner.
 </p>
 <p>"Psst," it says.</p>
 <p>
  "My height is
  {#await backend.getblockchaininfo()}...{:then info}{JSON.parse(info).result
    .blocks}{:catch error}... actually, I'm not really sure. {error}{/await}."
 </p>
 <p>
  "Are you
  {#await backend.get_person(1n)}...{:then person}{person.name}{:catch error}{error}{/await}?"
 </p>
</div>
