#![cfg_attr(any(), rustfmt::skip)]
///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version 9c1b8bdd90aa4346eea65d7b95a418855deac47c

//! Metadata about the Riot API and Riven.
//!
//! Note: this modules is automatically generated.

/// Metadata for endpoints. Each tuple corresponds to one endpoint and contains
/// the HTTP [`Method`](reqwest::Method), `str` path, and the method's `str` ID.
pub static ALL_ENDPOINTS: [(reqwest::Method, &str, &str); 87] = [
    (reqwest::Method::GET, "/riot/account/v1/accounts/by-puuid/{puuid}", "account-v1.getByPuuid"),
    (reqwest::Method::GET, "/riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}", "account-v1.getByRiotId"),
    (reqwest::Method::GET, "/riot/account/v1/accounts/me", "account-v1.getByAccessToken"),
    (reqwest::Method::GET, "/riot/account/v1/active-shards/by-game/{game}/by-puuid/{puuid}", "account-v1.getActiveShard"),
    (reqwest::Method::GET, "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}", "champion-mastery-v4.getAllChampionMasteriesByPUUID"),
    (reqwest::Method::GET, "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}/by-champion/{championId}", "champion-mastery-v4.getChampionMasteryByPUUID"),
    (reqwest::Method::GET, "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}/top", "champion-mastery-v4.getTopChampionMasteriesByPUUID"),
    (reqwest::Method::GET, "/lol/champion-mastery/v4/scores/by-puuid/{encryptedPUUID}", "champion-mastery-v4.getChampionMasteryScoreByPUUID"),
    (reqwest::Method::GET, "/lol/platform/v3/champion-rotations", "champion-v3.getChampionInfo"),
    (reqwest::Method::GET, "/lol/clash/v1/players/by-puuid/{puuid}", "clash-v1.getPlayersByPUUID"),
    (reqwest::Method::GET, "/lol/clash/v1/players/by-summoner/{summonerId}", "clash-v1.getPlayersBySummoner"),
    (reqwest::Method::GET, "/lol/clash/v1/teams/{teamId}", "clash-v1.getTeamById"),
    (reqwest::Method::GET, "/lol/clash/v1/tournaments", "clash-v1.getTournaments"),
    (reqwest::Method::GET, "/lol/clash/v1/tournaments/by-team/{teamId}", "clash-v1.getTournamentByTeam"),
    (reqwest::Method::GET, "/lol/clash/v1/tournaments/{tournamentId}", "clash-v1.getTournamentById"),
    (reqwest::Method::GET, "/lol/league-exp/v4/entries/{queue}/{tier}/{division}", "league-exp-v4.getLeagueEntries"),
    (reqwest::Method::GET, "/lol/league/v4/challengerleagues/by-queue/{queue}", "league-v4.getChallengerLeague"),
    (reqwest::Method::GET, "/lol/league/v4/entries/by-summoner/{encryptedSummonerId}", "league-v4.getLeagueEntriesForSummoner"),
    (reqwest::Method::GET, "/lol/league/v4/entries/{queue}/{tier}/{division}", "league-v4.getLeagueEntries"),
    (reqwest::Method::GET, "/lol/league/v4/grandmasterleagues/by-queue/{queue}", "league-v4.getGrandmasterLeague"),
    (reqwest::Method::GET, "/lol/league/v4/leagues/{leagueId}", "league-v4.getLeagueById"),
    (reqwest::Method::GET, "/lol/league/v4/masterleagues/by-queue/{queue}", "league-v4.getMasterLeague"),
    (reqwest::Method::GET, "/lol/challenges/v1/challenges/config", "lol-challenges-v1.getAllChallengeConfigs"),
    (reqwest::Method::GET, "/lol/challenges/v1/challenges/percentiles", "lol-challenges-v1.getAllChallengePercentiles"),
    (reqwest::Method::GET, "/lol/challenges/v1/challenges/{challengeId}/config", "lol-challenges-v1.getChallengeConfigs"),
    (reqwest::Method::GET, "/lol/challenges/v1/challenges/{challengeId}/leaderboards/by-level/{level}", "lol-challenges-v1.getChallengeLeaderboards"),
    (reqwest::Method::GET, "/lol/challenges/v1/challenges/{challengeId}/percentiles", "lol-challenges-v1.getChallengePercentiles"),
    (reqwest::Method::GET, "/lol/challenges/v1/player-data/{puuid}", "lol-challenges-v1.getPlayerData"),
    (reqwest::Method::GET, "/lol/rso-match/v1/matches/ids", "lol-rso-match-v1.getMatchIds"),
    (reqwest::Method::GET, "/lol/rso-match/v1/matches/{matchId}", "lol-rso-match-v1.getMatch"),
    (reqwest::Method::GET, "/lol/rso-match/v1/matches/{matchId}/timeline", "lol-rso-match-v1.getTimeline"),
    (reqwest::Method::GET, "/lol/status/v4/platform-data", "lol-status-v4.getPlatformData"),
    (reqwest::Method::GET, "/lor/deck/v1/decks/me", "lor-deck-v1.getDecks"),
    (reqwest::Method::POST, "/lor/deck/v1/decks/me", "lor-deck-v1.createDeck"),
    (reqwest::Method::GET, "/lor/inventory/v1/cards/me", "lor-inventory-v1.getCards"),
    (reqwest::Method::GET, "/lor/match/v1/matches/by-puuid/{puuid}/ids", "lor-match-v1.getMatchIdsByPUUID"),
    (reqwest::Method::GET, "/lor/match/v1/matches/{matchId}", "lor-match-v1.getMatch"),
    (reqwest::Method::GET, "/lor/ranked/v1/leaderboards", "lor-ranked-v1.getLeaderboards"),
    (reqwest::Method::GET, "/lor/status/v1/platform-data", "lor-status-v1.getPlatformData"),
    (reqwest::Method::GET, "/lol/match/v5/matches/by-puuid/{puuid}/ids", "match-v5.getMatchIdsByPUUID"),
    (reqwest::Method::GET, "/lol/match/v5/matches/{matchId}", "match-v5.getMatch"),
    (reqwest::Method::GET, "/lol/match/v5/matches/{matchId}/timeline", "match-v5.getTimeline"),
    (reqwest::Method::GET, "/lol/spectator/tft/v5/active-games/by-puuid/{encryptedPUUID}", "spectator-tft-v5.getCurrentGameInfoByPuuid"),
    (reqwest::Method::GET, "/lol/spectator/tft/v5/featured-games", "spectator-tft-v5.getFeaturedGames"),
    (reqwest::Method::GET, "/lol/spectator/v5/active-games/by-summoner/{encryptedPUUID}", "spectator-v5.getCurrentGameInfoByPuuid"),
    (reqwest::Method::GET, "/lol/spectator/v5/featured-games", "spectator-v5.getFeaturedGames"),
    (reqwest::Method::GET, "/fulfillment/v1/summoners/by-puuid/{rsoPUUID}", "summoner-v4.getByRSOPUUID"),
    (reqwest::Method::GET, "/lol/summoner/v4/summoners/by-account/{encryptedAccountId}", "summoner-v4.getByAccountId"),
    (reqwest::Method::GET, "/lol/summoner/v4/summoners/by-puuid/{encryptedPUUID}", "summoner-v4.getByPUUID"),
    (reqwest::Method::GET, "/lol/summoner/v4/summoners/me", "summoner-v4.getByAccessToken"),
    (reqwest::Method::GET, "/lol/summoner/v4/summoners/{encryptedSummonerId}", "summoner-v4.getBySummonerId"),
    (reqwest::Method::GET, "/tft/league/v1/challenger", "tft-league-v1.getChallengerLeague"),
    (reqwest::Method::GET, "/tft/league/v1/entries/by-summoner/{summonerId}", "tft-league-v1.getLeagueEntriesForSummoner"),
    (reqwest::Method::GET, "/tft/league/v1/entries/{tier}/{division}", "tft-league-v1.getLeagueEntries"),
    (reqwest::Method::GET, "/tft/league/v1/grandmaster", "tft-league-v1.getGrandmasterLeague"),
    (reqwest::Method::GET, "/tft/league/v1/leagues/{leagueId}", "tft-league-v1.getLeagueById"),
    (reqwest::Method::GET, "/tft/league/v1/master", "tft-league-v1.getMasterLeague"),
    (reqwest::Method::GET, "/tft/league/v1/rated-ladders/{queue}/top", "tft-league-v1.getTopRatedLadder"),
    (reqwest::Method::GET, "/tft/match/v1/matches/by-puuid/{puuid}/ids", "tft-match-v1.getMatchIdsByPUUID"),
    (reqwest::Method::GET, "/tft/match/v1/matches/{matchId}", "tft-match-v1.getMatch"),
    (reqwest::Method::GET, "/tft/status/v1/platform-data", "tft-status-v1.getPlatformData"),
    (reqwest::Method::GET, "/tft/summoner/v1/summoners/by-account/{encryptedAccountId}", "tft-summoner-v1.getByAccountId"),
    (reqwest::Method::GET, "/tft/summoner/v1/summoners/by-puuid/{encryptedPUUID}", "tft-summoner-v1.getByPUUID"),
    (reqwest::Method::GET, "/tft/summoner/v1/summoners/me", "tft-summoner-v1.getByAccessToken"),
    (reqwest::Method::GET, "/tft/summoner/v1/summoners/{encryptedSummonerId}", "tft-summoner-v1.getBySummonerId"),
    (reqwest::Method::POST, "/lol/tournament-stub/v5/codes", "tournament-stub-v5.createTournamentCode"),
    (reqwest::Method::GET, "/lol/tournament-stub/v5/codes/{tournamentCode}", "tournament-stub-v5.getTournamentCode"),
    (reqwest::Method::GET, "/lol/tournament-stub/v5/lobby-events/by-code/{tournamentCode}", "tournament-stub-v5.getLobbyEventsByCode"),
    (reqwest::Method::POST, "/lol/tournament-stub/v5/providers", "tournament-stub-v5.registerProviderData"),
    (reqwest::Method::POST, "/lol/tournament-stub/v5/tournaments", "tournament-stub-v5.registerTournament"),
    (reqwest::Method::POST, "/lol/tournament/v5/codes", "tournament-v5.createTournamentCode"),
    (reqwest::Method::GET, "/lol/tournament/v5/codes/{tournamentCode}", "tournament-v5.getTournamentCode"),
    (reqwest::Method::PUT, "/lol/tournament/v5/codes/{tournamentCode}", "tournament-v5.updateCode"),
    (reqwest::Method::GET, "/lol/tournament/v5/games/by-code/{tournamentCode}", "tournament-v5.getGames"),
    (reqwest::Method::GET, "/lol/tournament/v5/lobby-events/by-code/{tournamentCode}", "tournament-v5.getLobbyEventsByCode"),
    (reqwest::Method::POST, "/lol/tournament/v5/providers", "tournament-v5.registerProviderData"),
    (reqwest::Method::POST, "/lol/tournament/v5/tournaments", "tournament-v5.registerTournament"),
    (reqwest::Method::GET, "/val/match/console/v1/matches/{matchId}", "val-console-match-v1.getMatch"),
    (reqwest::Method::GET, "/val/match/console/v1/matchlists/by-puuid/{puuid}", "val-console-match-v1.getMatchlist"),
    (reqwest::Method::GET, "/val/match/console/v1/recent-matches/by-queue/{queue}", "val-console-match-v1.getRecent"),
    (reqwest::Method::GET, "/val/console/ranked/v1/leaderboards/by-act/{actId}", "val-console-ranked-v1.getLeaderboard"),
    (reqwest::Method::GET, "/val/content/v1/contents", "val-content-v1.getContent"),
    (reqwest::Method::GET, "/val/match/v1/matches/{matchId}", "val-match-v1.getMatch"),
    (reqwest::Method::GET, "/val/match/v1/matchlists/by-puuid/{puuid}", "val-match-v1.getMatchlist"),
    (reqwest::Method::GET, "/val/match/v1/recent-matches/by-queue/{queue}", "val-match-v1.getRecent"),
    (reqwest::Method::GET, "/val/ranked/v1/leaderboards/by-act/{actId}", "val-ranked-v1.getLeaderboard"),
    (reqwest::Method::GET, "/val/status/v1/platform-data", "val-status-v1.getPlatformData"),
];
