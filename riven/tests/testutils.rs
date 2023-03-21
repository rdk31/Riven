#![allow(dead_code)]

use lazy_static::lazy_static;

use riven::consts::RegionalRoute;
use riven::{RiotApi, RiotApiConfig};

lazy_static! {
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY")
            .ok()
            .or_else(|| std::fs::read_to_string("apikey.txt").ok())
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::new(RiotApiConfig::with_key(api_key.trim()).preconfig_burst())
    };
}

pub async fn tft_match_v1_get(route: RegionalRoute, matches: &[impl AsRef<str>]) -> Result<(), String> {
    let futures = matches.iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = RIOT_API.tft_match_v1().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err("Match should have participants (metadata).".to_owned());
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err("Match participants do not line up with participant UUIDs.".to_owned());
        }
        if m.info.participants.is_empty() {
            return Err("Match should have participants (info).".to_owned());
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

pub async fn match_v5_get(route: RegionalRoute, matches: &[impl AsRef<str>]) -> Result<(), String> {
    let futures = matches.iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = RIOT_API.match_v5().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err("Match should have participants.".to_owned());
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err("Match participants do not line up with participant UUIDs.".to_owned());
        }
        for participant in &m.info.participants {
            participant.champion().map_err(|e| format!("Failed to determine champion: {}", e))?;
        }
        if m.info.teams.is_empty() {
            return Err("Match should have teams.".to_owned());
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

pub async fn match_v5_get_timeline(
    route: RegionalRoute,
    matches: &[impl AsRef<str>],
) -> Result<(), String> {
    let futures = matches.iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = RIOT_API.match_v5().get_timeline(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;
        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err("Match should have participants.".to_owned());
        }
        if let Some(game_id) = m.info.game_id {
            if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err("Match number ID should match.".to_owned());
            }
        }
        if m.info.frames.is_empty() {
            return Err("Match timleine should have frames.".to_owned());
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}
