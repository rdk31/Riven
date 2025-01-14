#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

const ROUTE: RegionalRoute = RegionalRoute::ASIA;

static MATCHES: &[&str] = &[
    // Regular game:
    "KR_5495121707",
    // `teamPosition` empty:
    // AFK:
    "JP1_312062554",
    "JP1_326464722",
    "JP1_289504387",
    "JP1_285434511",
    "JP1_307559381",
    "JP1_292569767",
    "JP1_310138781",
    "JP1_300507433",
    "JP1_283568774",
    // `individualPosition` is set but `teamPosition` is empty due to AFK slightly after beginning:
    "JP1_285797147",
    // Illegal big `championId`s. https://github.com/RiotGames/developer-relations/issues/553
    "JP1_267647303",
    "JP1_273343663",
];

async_tests! {
    my_runner {
        match_v5_get: async {
            match_v5_get(ROUTE, MATCHES).await
        },
        match_v5_get_timeline: async {
            match_v5_get_timeline(ROUTE, MATCHES).await
        },
    }
}
