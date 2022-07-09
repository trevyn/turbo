import turbocharger_init, * as backend from "./turbocharger_generated";

let prom = turbocharger_init().then(async () => {
 if (
  window.location.href == "http://127.0.0.1:3000/" ||
  window.location.href == "http://localhost:3000/"
 )
  backend.set_socket_url("ws://127.0.0.1:8080/turbocharger_socket");
 // await backend.wasm_notify_client_pk();
 backend.start_web();
});

(async () => {
 await prom;
})();
