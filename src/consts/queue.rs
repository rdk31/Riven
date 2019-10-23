﻿// This file is automatically generated.
// Do not directly edit.
// Generated on 2019-10-23T08:20:11.107Z.

/// League of Legends matchmaking queue.
pub enum Queue {

    /// Custom games.
    CustomGames = 0,

    /// 5v5 Blind Pick games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 430
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 430")]
    SummonersRift5v5BlindPickGames2 = 2,
    /// 5v5 Blind Pick games games on Summoner's Rift.
    SummonersRift5v5BlindPickGames430 = 430,

    /// 5v5 Ranked Solo games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in favor of queueId 420
    #[deprecated(note="Deprecated in favor of queueId 420")]
    SummonersRift5v5RankedSoloGames4 = 4,
    /// 5v5 Ranked Solo games games on Summoner's Rift.
    SummonersRift5v5RankedSoloGames420 = 420,

    /// 5v5 Ranked Premade games games on Summoner's Rift.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SummonersRift5v5RankedPremadeGames = 6,

    /// Co-op vs AI games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in favor of queueId 32 and 33
    #[deprecated(note="Deprecated in favor of queueId 32 and 33")]
    SummonersRiftCoOpVsAiGames = 7,

    /// 3v3 Normal games games on Twisted Treeline.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 460
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 460")]
    TwistedTreeline3v3NormalGames = 8,

    /// 3v3 Ranked Flex games games on Twisted Treeline.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 470
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 470")]
    TwistedTreeline3v3RankedFlexGames9 = 9,
    /// 3v3 Ranked Flex games games on Twisted Treeline.
    TwistedTreeline3v3RankedFlexGames470 = 470,

    /// 5v5 Draft Pick games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in favor of queueId 400
    #[deprecated(note="Deprecated in favor of queueId 400")]
    SummonersRift5v5DraftPickGames14 = 14,
    /// 5v5 Draft Pick games games on Summoner's Rift.
    SummonersRift5v5DraftPickGames400 = 400,

    /// 5v5 Dominion Blind Pick games games on Crystal Scar.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CrystalScar5v5DominionBlindPickGames = 16,

    /// 5v5 Dominion Draft Pick games games on Crystal Scar.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CrystalScar5v5DominionDraftPickGames = 17,

    /// Dominion Co-op vs AI games games on Crystal Scar.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CrystalScarDominionCoOpVsAiGames = 25,

    /// Co-op vs AI Intro Bot games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 830
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 830")]
    SummonersRiftCoOpVsAiIntroBotGames31 = 31,
    /// Co-op vs. AI Intro Bot games games on Summoner's Rift.
    SummonersRiftCoOpVsAiIntroBotGames830 = 830,

    /// Co-op vs AI Beginner Bot games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 840
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 840")]
    SummonersRiftCoOpVsAiBeginnerBotGames32 = 32,
    /// Co-op vs. AI Beginner Bot games games on Summoner's Rift.
    SummonersRiftCoOpVsAiBeginnerBotGames840 = 840,

    /// Co-op vs AI Intermediate Bot games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 850
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 850")]
    SummonersRiftCoOpVsAiIntermediateBotGames33 = 33,
    /// Co-op vs. AI Intermediate Bot games games on Summoner's Rift.
    SummonersRiftCoOpVsAiIntermediateBotGames850 = 850,

    /// 3v3 Ranked Team games games on Twisted Treeline.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    TwistedTreeline3v3RankedTeamGames = 41,

    /// 5v5 Ranked Team games games on Summoner's Rift.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SummonersRift5v5RankedTeamGames = 42,

    /// Co-op vs AI games games on Twisted Treeline.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 800
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 800")]
    TwistedTreelineCoOpVsAiGames = 52,

    /// 5v5 Team Builder games games on Summoner's Rift.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SummonersRift5v5TeamBuilderGames = 61,

    /// 5v5 ARAM games games on Howling Abyss.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 450
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 450")]
    HowlingAbyss5v5AramGames65 = 65,
    /// 5v5 ARAM games games on Howling Abyss.
    HowlingAbyss5v5AramGames450 = 450,

    /// ARAM Co-op vs AI games games on Howling Abyss.
    /// # Notes
    /// Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    HowlingAbyssAramCoOpVsAiGames = 67,

    /// One for All games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 8.6 in favor of queueId 1020
    #[deprecated(note="Deprecated in patch 8.6 in favor of queueId 1020")]
    SummonersRiftOneForAllGames70 = 70,
    /// One for All games games on Summoner's Rift.
    SummonersRiftOneForAllGames1020 = 1020,

    /// 1v1 Snowdown Showdown games games on Howling Abyss.
    HowlingAbyss1v1SnowdownShowdownGames = 72,

    /// 2v2 Snowdown Showdown games games on Howling Abyss.
    HowlingAbyss2v2SnowdownShowdownGames = 73,

    /// 6v6 Hexakill games games on Summoner's Rift.
    SummonersRift6v6HexakillGames = 75,

    /// Ultra Rapid Fire games games on Summoner's Rift.
    SummonersRiftUltraRapidFireGames = 76,

    /// One For All: Mirror Mode games games on Howling Abyss.
    HowlingAbyssOneForAllMirrorModeGames = 78,

    /// Co-op vs AI Ultra Rapid Fire games games on Summoner's Rift.
    SummonersRiftCoOpVsAiUltraRapidFireGames = 83,

    /// Doom Bots Rank 1 games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SummonersRiftDoomBotsRank1Games = 91,

    /// Doom Bots Rank 2 games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SummonersRiftDoomBotsRank2Games = 92,

    /// Doom Bots Rank 5 games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SummonersRiftDoomBotsRank5Games = 93,

    /// Ascension games games on Crystal Scar.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 910
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 910")]
    CrystalScarAscensionGames96 = 96,
    /// Ascension games games on Crystal Scar.
    CrystalScarAscensionGames910 = 910,

    /// 6v6 Hexakill games games on Twisted Treeline.
    TwistedTreeline6v6HexakillGames = 98,

    /// 5v5 ARAM games games on Butcher's Bridge.
    ButchersBridge5v5AramGames = 100,

    /// Legend of the Poro King games games on Howling Abyss.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 920
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 920")]
    HowlingAbyssLegendOfThePoroKingGames300 = 300,
    /// Legend of the Poro King games games on Howling Abyss.
    HowlingAbyssLegendOfThePoroKingGames920 = 920,

    /// Nemesis games games on Summoner's Rift.
    SummonersRiftNemesisGames = 310,

    /// Black Market Brawlers games games on Summoner's Rift.
    SummonersRiftBlackMarketBrawlersGames = 313,

    /// Nexus Siege games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 940
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 940")]
    SummonersRiftNexusSiegeGames315 = 315,
    /// Nexus Siege games games on Summoner's Rift.
    SummonersRiftNexusSiegeGames940 = 940,

    /// Definitely Not Dominion games games on Crystal Scar.
    CrystalScarDefinitelyNotDominionGames = 317,

    /// ARURF games games on Summoner's Rift.
    /// # Notes
    /// Deprecated in patch 7.19 in favor of queueId 900
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 900")]
    SummonersRiftArurfGames = 318,

    /// All Random games games on Summoner's Rift.
    SummonersRiftAllRandomGames = 325,

    /// 5v5 Ranked Dynamic games games on Summoner's Rift.
    /// # Notes
    /// Game mode deprecated in patch 6.22
    #[deprecated(note="Game mode deprecated in patch 6.22")]
    SummonersRift5v5RankedDynamicGames = 410,

    /// 5v5 Ranked Flex games games on Summoner's Rift.
    SummonersRift5v5RankedFlexGames = 440,

    /// 3v3 Blind Pick games games on Twisted Treeline.
    TwistedTreeline3v3BlindPickGames = 460,

    /// Blood Hunt Assassin games games on Summoner's Rift.
    SummonersRiftBloodHuntAssassinGames = 600,

    /// Dark Star: Singularity games games on Cosmic Ruins.
    CosmicRuinsDarkStarSingularityGames = 610,

    /// Clash games games on Summoner's Rift.
    SummonersRiftClashGames = 700,

    /// Co-op vs. AI Intermediate Bot games games on Twisted Treeline.
    TwistedTreelineCoOpVsAiIntermediateBotGames = 800,

    /// Co-op vs. AI Intro Bot games games on Twisted Treeline.
    TwistedTreelineCoOpVsAiIntroBotGames = 810,

    /// Co-op vs. AI Beginner Bot games games on Twisted Treeline.
    TwistedTreelineCoOpVsAiBeginnerBotGames = 820,

    /// URF games games on Summoner's Rift.
    SummonersRiftUrfGames = 900,

    /// Doom Bots Voting games games on Summoner's Rift.
    SummonersRiftDoomBotsVotingGames = 950,

    /// Doom Bots Standard games games on Summoner's Rift.
    SummonersRiftDoomBotsStandardGames = 960,

    /// Star Guardian Invasion: Normal games games on Valoran City Park.
    ValoranCityParkStarGuardianInvasionNormalGames = 980,

    /// Star Guardian Invasion: Onslaught games games on Valoran City Park.
    ValoranCityParkStarGuardianInvasionOnslaughtGames = 990,

    /// PROJECT: Hunters games games on Overcharge.
    OverchargeProjectHuntersGames = 1000,

    /// Snow ARURF games games on Summoner's Rift.
    SummonersRiftSnowArurfGames = 1010,

    /// Odyssey Extraction: Intro games games on Crash Site.
    CrashSiteOdysseyExtractionIntroGames = 1030,

    /// Odyssey Extraction: Cadet games games on Crash Site.
    CrashSiteOdysseyExtractionCadetGames = 1040,

    /// Odyssey Extraction: Crewmember games games on Crash Site.
    CrashSiteOdysseyExtractionCrewmemberGames = 1050,

    /// Odyssey Extraction: Captain games games on Crash Site.
    CrashSiteOdysseyExtractionCaptainGames = 1060,

    /// Odyssey Extraction: Onslaught games games on Crash Site.
    CrashSiteOdysseyExtractionOnslaughtGames = 1070,

    /// Teamfight Tactics games games on Convergence.
    ConvergenceTeamfightTacticsGames = 1090,

    /// Ranked Teamfight Tactics games games on Convergence.
    ConvergenceRankedTeamfightTacticsGames = 1100,

    /// Nexus Blitz games games on Nexus Blitz.
    /// # Notes
    /// Deprecated in patch 9.2
    #[deprecated(note="Deprecated in patch 9.2")]
    NexusBlitzNexusBlitzGames = 1200,
}