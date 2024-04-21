/// Non-match Valorant tests for LATAM.

mod testutils;
use riven::consts::*;
use testutils::{riven_test, val_content_ranked};

const ROUTE: ValPlatformRoute = ValPlatformRoute::LATAM;

#[riven_test]
async fn val_content_ranked_test() -> Result<(), String> {
    val_content_ranked(ROUTE).await
}
