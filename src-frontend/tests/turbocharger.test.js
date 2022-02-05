import turbocharger_init, * as backend from "../turbocharger_generated";

it("does stuff", async () => {
 await turbocharger_init();
 backend.set_socket_url("ws://localhost:8080/turbocharger_socket");
 console.log("backend.heartbeat(): ", await backend.heartbeat());
});
