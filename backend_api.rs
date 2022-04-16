// This file is auto-generated by Turbocharger.
// Check it into version control to track API changes over time.

async fn _turbonet_self() -> _Turbonet_SelfResponse {}

struct _Turbonet_SelfResponse {
    crypto_box_public_key: [u8; 32],
    bls_public_key: [u8; 96],
    bls_proof_of_possession: [u8; 48],
    base_url: Option<String>,
    build_id: String,
}

async fn animal_log() -> Result<String, tracked::StringError> {}

async fn animal_time() -> String {}

fn animal_time_stream() -> impl Stream<Item = Result<String, tracked::StringError>> {}

struct animal_time_stream_log {
    rowid: Option<i64>,
    timestamp: Option<i64>,
    animal_timestamp: Option<String>,
    remote_addr: Option<String>,
    user_agent: Option<String>,
}

async fn check_for_updates() -> Result<String, tracked::StringError> {}

fn encrypted_animal_time_stream() -> impl Stream<
        Item = Result<String, tracked::StringError>,
    > {}

async fn getblockchaininfo() -> Result<String, tracked::StringError> {}

async fn heartbeat() -> Result<String, tracked::StringError> {}

async fn mail(rowid: i64) -> Result<String, tracked::StringError> {}

async fn mailrowidlist() -> Result<Veci64, tracked::StringError> {}

async fn notify_client_pk(client_pk: Vec<u8>) -> Result<(), tracked::StringError> {}

fn stream_example_result() -> impl Stream<Item = Result<String, tracked::StringError>> {}

struct Veci64 {
    vec: Vec<i64>,
}
