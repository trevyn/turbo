import turbocharger_init, * as backend from "../turbocharger_generated";

it("does stuff", async () => {
 await turbocharger_init();
 console.log(
  "backend.check_for_updates(): ",
  await backend.check_for_updates()
 );
});
