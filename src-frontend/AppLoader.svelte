<script lang="ts">
 import turbocharger_init, * as backend from "./turbocharger_generated";
 import App from "./App.svelte";

 let prom = turbocharger_init().then(async () => {
  backend.turbo_start_web();
  if (
   window.location.href == "http://127.0.0.1:3000/" ||
   window.location.href == "http://localhost:3000/"
  )
   backend.set_socket_url("ws://127.0.0.1:8080/turbocharger_socket");
  await backend.wasm_notify_client_pk();
 });
</script>

{#await prom then _}
 <App />
{:catch error}
 <p style="color: red">Error loading WASM: {error.message}</p>
{/await}
