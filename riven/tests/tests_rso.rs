mod testutils;

use riven::consts::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::AMERICAS;

/// https://developer.riotgames.com/apis#account-v1/GET_getByAccessToken
#[riven_test]
async fn account_v1_getbyaccesstoken() -> Result<(), String> {
    let Ok(access_token) = std::env::var("RGAPI_ACCESS_TOKEN") else {
        eprintln!("`RGAPI_ACCESS_TOKEN` env var not set, cannot test RSO.");
        return Ok(());
    };

    let account = riot_api()
        .account_v1()
        .get_by_access_token(ROUTE, access_token)
        .await
        .map_err(|e| format!("Failed to get account by riot ID: {}", e))?;

    println!("{:#?}", account);

    Ok(())
}
