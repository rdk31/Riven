mod testutils;
use riven::consts::*;
use testutils::{riven_test, val_content_ranked, val_match_v1_get, val_match_v1_latest};

const ROUTE: ValPlatformRoute = ValPlatformRoute::NA;

static MATCHES: &[&str] = &[
    "b979be9e-aeb1-419e-981d-d7d2472649a2",
    "c086129c-e538-43cf-ae93-7148d0d01182",
];

#[riven_test]
async fn val_match_v1_get_test() -> Result<(), String> {
    val_match_v1_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn val_content_ranked_test() -> Result<(), String> {
    val_content_ranked(ROUTE).await
}

#[riven_test]
async fn val_match_v1_latest_test() -> Result<(), String> {
    val_match_v1_latest(ROUTE).await
}
