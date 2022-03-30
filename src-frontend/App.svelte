<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import Tab1 from "./Tab1.svelte";
 import Tab2 from "./Tab2.svelte";
 import Tab3 from "./Tab3.svelte";
 import Tab from "./Tab.svelte";
 import {
  TabGroup,
  TabList,
  TabPanels,
  TabPanel,
 } from "@rgossiaux/svelte-headlessui";

 (async () => {
  // let person = Object.assign(new backend.Person(), { name: "Bob" });
  // let person2 = new backend.Person({ name: "bob" });
  // let rowid = await backend.insert_person(person);
  // console.log("Inserted rowid ", rowid);
 })();
</script>

<div>
 <p>
  {#await backend.check_for_updates()}Checking for updates...{:then info}{info}{:catch error}Error
   checking for updates: {error}{/await}
 </p>

 <TabGroup>
  <TabList class="pt-1.5">
   <Tab>MAIL</Tab><Tab>ANIMAL LOG</Tab><Tab>ANIMAL LOG LOG</Tab>
  </TabList>
  <TabPanels class="pt-3.5">
   <TabPanel class="outline-none"><Tab1 /></TabPanel>
   <TabPanel class="outline-none"><Tab2 /></TabPanel>
   <TabPanel class="outline-none"><Tab3 /></TabPanel>
  </TabPanels>
 </TabGroup>
</div>
