﻿// This file is automatically generated.
// Do not directly edit.

use std::fmt;
use num_derive;

/// League of Legend's champions.
///
/// The documentation of each variant specifies:<br>
/// NAME (`IDENTIFIER`, ID).
#[derive(Debug, Copy, Clone)]
#[derive(num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub enum Champion {
    /** Aatrox         (`Aatrox`, 266).      */ Aatrox       = 266,
    /** Ahri           (`Ahri`, 103).        */ Ahri         = 103,
    /** Akali          (`Akali`, 84).        */ Akali        = 84,
    /** Alistar        (`Alistar`, 12).      */ Alistar      = 12,
    /** Amumu          (`Amumu`, 32).        */ Amumu        = 32,
    /** Anivia         (`Anivia`, 34).       */ Anivia       = 34,
    /** Annie          (`Annie`, 1).         */ Annie        = 1,
    /** Ashe           (`Ashe`, 22).         */ Ashe         = 22,
    /** Aurelion Sol   (`AurelionSol`, 136). */ AurelionSol  = 136,
    /** Azir           (`Azir`, 268).        */ Azir         = 268,
    /** Bard           (`Bard`, 432).        */ Bard         = 432,
    /** Blitzcrank     (`Blitzcrank`, 53).   */ Blitzcrank   = 53,
    /** Brand          (`Brand`, 63).        */ Brand        = 63,
    /** Braum          (`Braum`, 201).       */ Braum        = 201,
    /** Caitlyn        (`Caitlyn`, 51).      */ Caitlyn      = 51,
    /** Camille        (`Camille`, 164).     */ Camille      = 164,
    /** Cassiopeia     (`Cassiopeia`, 69).   */ Cassiopeia   = 69,
    /** Cho'Gath       (`Chogath`, 31).      */ ChoGath      = 31,
    /** Corki          (`Corki`, 42).        */ Corki        = 42,
    /** Darius         (`Darius`, 122).      */ Darius       = 122,
    /** Diana          (`Diana`, 131).       */ Diana        = 131,
    /** Dr. Mundo      (`DrMundo`, 36).      */ DrMundo      = 36,
    /** Draven         (`Draven`, 119).      */ Draven       = 119,
    /** Ekko           (`Ekko`, 245).        */ Ekko         = 245,
    /** Elise          (`Elise`, 60).        */ Elise        = 60,
    /** Evelynn        (`Evelynn`, 28).      */ Evelynn      = 28,
    /** Ezreal         (`Ezreal`, 81).       */ Ezreal       = 81,
    /** Fiddlesticks   (`FiddleSticks`, 9).  */ Fiddlesticks = 9,
    /** Fiora          (`Fiora`, 114).       */ Fiora        = 114,
    /** Fizz           (`Fizz`, 105).        */ Fizz         = 105,
    /** Galio          (`Galio`, 3).         */ Galio        = 3,
    /** Gangplank      (`Gangplank`, 41).    */ Gangplank    = 41,
    /** Garen          (`Garen`, 86).        */ Garen        = 86,
    /** Gnar           (`Gnar`, 150).        */ Gnar         = 150,
    /** Gragas         (`Gragas`, 79).       */ Gragas       = 79,
    /** Graves         (`Graves`, 104).      */ Graves       = 104,
    /** Hecarim        (`Hecarim`, 120).     */ Hecarim      = 120,
    /** Heimerdinger   (`Heimerdinger`, 74). */ Heimerdinger = 74,
    /** Illaoi         (`Illaoi`, 420).      */ Illaoi       = 420,
    /** Irelia         (`Irelia`, 39).       */ Irelia       = 39,
    /** Ivern          (`Ivern`, 427).       */ Ivern        = 427,
    /** Janna          (`Janna`, 40).        */ Janna        = 40,
    /** Jarvan IV      (`JarvanIV`, 59).     */ JarvanIV     = 59,
    /** Jax            (`Jax`, 24).          */ Jax          = 24,
    /** Jayce          (`Jayce`, 126).       */ Jayce        = 126,
    /** Jhin           (`Jhin`, 202).        */ Jhin         = 202,
    /** Jinx           (`Jinx`, 222).        */ Jinx         = 222,
    /** Kai'Sa         (`Kaisa`, 145).       */ KaiSa        = 145,
    /** Kalista        (`Kalista`, 429).     */ Kalista      = 429,
    /** Karma          (`Karma`, 43).        */ Karma        = 43,
    /** Karthus        (`Karthus`, 30).      */ Karthus      = 30,
    /** Kassadin       (`Kassadin`, 38).     */ Kassadin     = 38,
    /** Katarina       (`Katarina`, 55).     */ Katarina     = 55,
    /** Kayle          (`Kayle`, 10).        */ Kayle        = 10,
    /** Kayn           (`Kayn`, 141).        */ Kayn         = 141,
    /** Kennen         (`Kennen`, 85).       */ Kennen       = 85,
    /** Kha'Zix        (`Khazix`, 121).      */ KhaZix       = 121,
    /** Kindred        (`Kindred`, 203).     */ Kindred      = 203,
    /** Kled           (`Kled`, 240).        */ Kled         = 240,
    /** Kog'Maw        (`KogMaw`, 96).       */ KogMaw       = 96,
    /** LeBlanc        (`Leblanc`, 7).       */ LeBlanc      = 7,
    /** Lee Sin        (`LeeSin`, 64).       */ LeeSin       = 64,
    /** Leona          (`Leona`, 89).        */ Leona        = 89,
    /** Lissandra      (`Lissandra`, 127).   */ Lissandra    = 127,
    /** Lucian         (`Lucian`, 236).      */ Lucian       = 236,
    /** Lulu           (`Lulu`, 117).        */ Lulu         = 117,
    /** Lux            (`Lux`, 99).          */ Lux          = 99,
    /** Malphite       (`Malphite`, 54).     */ Malphite     = 54,
    /** Malzahar       (`Malzahar`, 90).     */ Malzahar     = 90,
    /** Maokai         (`Maokai`, 57).       */ Maokai       = 57,
    /** Master Yi      (`MasterYi`, 11).     */ MasterYi     = 11,
    /** Miss Fortune   (`MissFortune`, 21).  */ MissFortune  = 21,
    /** Mordekaiser    (`Mordekaiser`, 82).  */ Mordekaiser  = 82,
    /** Morgana        (`Morgana`, 25).      */ Morgana      = 25,
    /** Nami           (`Nami`, 267).        */ Nami         = 267,
    /** Nasus          (`Nasus`, 75).        */ Nasus        = 75,
    /** Nautilus       (`Nautilus`, 111).    */ Nautilus     = 111,
    /** Neeko          (`Neeko`, 518).       */ Neeko        = 518,
    /** Nidalee        (`Nidalee`, 76).      */ Nidalee      = 76,
    /** Nocturne       (`Nocturne`, 56).     */ Nocturne     = 56,
    /** Nunu & Willump (`Nunu`, 20).         */ NunuWillump  = 20,
    /** Olaf           (`Olaf`, 2).          */ Olaf         = 2,
    /** Orianna        (`Orianna`, 61).      */ Orianna      = 61,
    /** Ornn           (`Ornn`, 516).        */ Ornn         = 516,
    /** Pantheon       (`Pantheon`, 80).     */ Pantheon     = 80,
    /** Poppy          (`Poppy`, 78).        */ Poppy        = 78,
    /** Pyke           (`Pyke`, 555).        */ Pyke         = 555,
    /** Qiyana         (`Qiyana`, 246).      */ Qiyana       = 246,
    /** Quinn          (`Quinn`, 133).       */ Quinn        = 133,
    /** Rakan          (`Rakan`, 497).       */ Rakan        = 497,
    /** Rammus         (`Rammus`, 33).       */ Rammus       = 33,
    /** Rek'Sai        (`RekSai`, 421).      */ RekSai       = 421,
    /** Renekton       (`Renekton`, 58).     */ Renekton     = 58,
    /** Rengar         (`Rengar`, 107).      */ Rengar       = 107,
    /** Riven          (`Riven`, 92).        */ Riven        = 92,
    /** Rumble         (`Rumble`, 68).       */ Rumble       = 68,
    /** Ryze           (`Ryze`, 13).         */ Ryze         = 13,
    /** Sejuani        (`Sejuani`, 113).     */ Sejuani      = 113,
    /** Shaco          (`Shaco`, 35).        */ Shaco        = 35,
    /** Shen           (`Shen`, 98).         */ Shen         = 98,
    /** Shyvana        (`Shyvana`, 102).     */ Shyvana      = 102,
    /** Singed         (`Singed`, 27).       */ Singed       = 27,
    /** Sion           (`Sion`, 14).         */ Sion         = 14,
    /** Sivir          (`Sivir`, 15).        */ Sivir        = 15,
    /** Skarner        (`Skarner`, 72).      */ Skarner      = 72,
    /** Sona           (`Sona`, 37).         */ Sona         = 37,
    /** Soraka         (`Soraka`, 16).       */ Soraka       = 16,
    /** Swain          (`Swain`, 50).        */ Swain        = 50,
    /** Sylas          (`Sylas`, 517).       */ Sylas        = 517,
    /** Syndra         (`Syndra`, 134).      */ Syndra       = 134,
    /** Tahm Kench     (`TahmKench`, 223).   */ TahmKench    = 223,
    /** Taliyah        (`Taliyah`, 163).     */ Taliyah      = 163,
    /** Talon          (`Talon`, 91).        */ Talon        = 91,
    /** Taric          (`Taric`, 44).        */ Taric        = 44,
    /** Teemo          (`Teemo`, 17).        */ Teemo        = 17,
    /** Thresh         (`Thresh`, 412).      */ Thresh       = 412,
    /** Tristana       (`Tristana`, 18).     */ Tristana     = 18,
    /** Trundle        (`Trundle`, 48).      */ Trundle      = 48,
    /** Tryndamere     (`Tryndamere`, 23).   */ Tryndamere   = 23,
    /** Twisted Fate   (`TwistedFate`, 4).   */ TwistedFate  = 4,
    /** Twitch         (`Twitch`, 29).       */ Twitch       = 29,
    /** Udyr           (`Udyr`, 77).         */ Udyr         = 77,
    /** Urgot          (`Urgot`, 6).         */ Urgot        = 6,
    /** Varus          (`Varus`, 110).       */ Varus        = 110,
    /** Vayne          (`Vayne`, 67).        */ Vayne        = 67,
    /** Veigar         (`Veigar`, 45).       */ Veigar       = 45,
    /** Vel'Koz        (`Velkoz`, 161).      */ VelKoz       = 161,
    /** Vi             (`Vi`, 254).          */ Vi           = 254,
    /** Viktor         (`Viktor`, 112).      */ Viktor       = 112,
    /** Vladimir       (`Vladimir`, 8).      */ Vladimir     = 8,
    /** Volibear       (`Volibear`, 106).    */ Volibear     = 106,
    /** Warwick        (`Warwick`, 19).      */ Warwick      = 19,
    /** Wukong         (`MonkeyKing`, 62).   */ Wukong       = 62,
    /** Xayah          (`Xayah`, 498).       */ Xayah        = 498,
    /** Xerath         (`Xerath`, 101).      */ Xerath       = 101,
    /** Xin Zhao       (`XinZhao`, 5).       */ XinZhao      = 5,
    /** Yasuo          (`Yasuo`, 157).       */ Yasuo        = 157,
    /** Yorick         (`Yorick`, 83).       */ Yorick       = 83,
    /** Yuumi          (`Yuumi`, 350).       */ Yuumi        = 350,
    /** Zac            (`Zac`, 154).         */ Zac          = 154,
    /** Zed            (`Zed`, 238).         */ Zed          = 238,
    /** Ziggs          (`Ziggs`, 115).       */ Ziggs        = 115,
    /** Zilean         (`Zilean`, 26).       */ Zilean       = 26,
    /** Zoe            (`Zoe`, 142).         */ Zoe          = 142,
    /** Zyra           (`Zyra`, 143).        */ Zyra         = 143,
}

impl Champion {
    pub fn name(self) -> &'static str {
        match self {
            Self::Aatrox       => "Aatrox",
            Self::Ahri         => "Ahri",
            Self::Akali        => "Akali",
            Self::Alistar      => "Alistar",
            Self::Amumu        => "Amumu",
            Self::Anivia       => "Anivia",
            Self::Annie        => "Annie",
            Self::Ashe         => "Ashe",
            Self::AurelionSol  => "Aurelion Sol",
            Self::Azir         => "Azir",
            Self::Bard         => "Bard",
            Self::Blitzcrank   => "Blitzcrank",
            Self::Brand        => "Brand",
            Self::Braum        => "Braum",
            Self::Caitlyn      => "Caitlyn",
            Self::Camille      => "Camille",
            Self::Cassiopeia   => "Cassiopeia",
            Self::ChoGath      => "Cho'Gath",
            Self::Corki        => "Corki",
            Self::Darius       => "Darius",
            Self::Diana        => "Diana",
            Self::DrMundo      => "Dr. Mundo",
            Self::Draven       => "Draven",
            Self::Ekko         => "Ekko",
            Self::Elise        => "Elise",
            Self::Evelynn      => "Evelynn",
            Self::Ezreal       => "Ezreal",
            Self::Fiddlesticks => "Fiddlesticks",
            Self::Fiora        => "Fiora",
            Self::Fizz         => "Fizz",
            Self::Galio        => "Galio",
            Self::Gangplank    => "Gangplank",
            Self::Garen        => "Garen",
            Self::Gnar         => "Gnar",
            Self::Gragas       => "Gragas",
            Self::Graves       => "Graves",
            Self::Hecarim      => "Hecarim",
            Self::Heimerdinger => "Heimerdinger",
            Self::Illaoi       => "Illaoi",
            Self::Irelia       => "Irelia",
            Self::Ivern        => "Ivern",
            Self::Janna        => "Janna",
            Self::JarvanIV     => "Jarvan IV",
            Self::Jax          => "Jax",
            Self::Jayce        => "Jayce",
            Self::Jhin         => "Jhin",
            Self::Jinx         => "Jinx",
            Self::KaiSa        => "Kai'Sa",
            Self::Kalista      => "Kalista",
            Self::Karma        => "Karma",
            Self::Karthus      => "Karthus",
            Self::Kassadin     => "Kassadin",
            Self::Katarina     => "Katarina",
            Self::Kayle        => "Kayle",
            Self::Kayn         => "Kayn",
            Self::Kennen       => "Kennen",
            Self::KhaZix       => "Kha'Zix",
            Self::Kindred      => "Kindred",
            Self::Kled         => "Kled",
            Self::KogMaw       => "Kog'Maw",
            Self::LeBlanc      => "LeBlanc",
            Self::LeeSin       => "Lee Sin",
            Self::Leona        => "Leona",
            Self::Lissandra    => "Lissandra",
            Self::Lucian       => "Lucian",
            Self::Lulu         => "Lulu",
            Self::Lux          => "Lux",
            Self::Malphite     => "Malphite",
            Self::Malzahar     => "Malzahar",
            Self::Maokai       => "Maokai",
            Self::MasterYi     => "Master Yi",
            Self::MissFortune  => "Miss Fortune",
            Self::Mordekaiser  => "Mordekaiser",
            Self::Morgana      => "Morgana",
            Self::Nami         => "Nami",
            Self::Nasus        => "Nasus",
            Self::Nautilus     => "Nautilus",
            Self::Neeko        => "Neeko",
            Self::Nidalee      => "Nidalee",
            Self::Nocturne     => "Nocturne",
            Self::NunuWillump  => "Nunu & Willump",
            Self::Olaf         => "Olaf",
            Self::Orianna      => "Orianna",
            Self::Ornn         => "Ornn",
            Self::Pantheon     => "Pantheon",
            Self::Poppy        => "Poppy",
            Self::Pyke         => "Pyke",
            Self::Qiyana       => "Qiyana",
            Self::Quinn        => "Quinn",
            Self::Rakan        => "Rakan",
            Self::Rammus       => "Rammus",
            Self::RekSai       => "Rek'Sai",
            Self::Renekton     => "Renekton",
            Self::Rengar       => "Rengar",
            Self::Riven        => "Riven",
            Self::Rumble       => "Rumble",
            Self::Ryze         => "Ryze",
            Self::Sejuani      => "Sejuani",
            Self::Shaco        => "Shaco",
            Self::Shen         => "Shen",
            Self::Shyvana      => "Shyvana",
            Self::Singed       => "Singed",
            Self::Sion         => "Sion",
            Self::Sivir        => "Sivir",
            Self::Skarner      => "Skarner",
            Self::Sona         => "Sona",
            Self::Soraka       => "Soraka",
            Self::Swain        => "Swain",
            Self::Sylas        => "Sylas",
            Self::Syndra       => "Syndra",
            Self::TahmKench    => "Tahm Kench",
            Self::Taliyah      => "Taliyah",
            Self::Talon        => "Talon",
            Self::Taric        => "Taric",
            Self::Teemo        => "Teemo",
            Self::Thresh       => "Thresh",
            Self::Tristana     => "Tristana",
            Self::Trundle      => "Trundle",
            Self::Tryndamere   => "Tryndamere",
            Self::TwistedFate  => "Twisted Fate",
            Self::Twitch       => "Twitch",
            Self::Udyr         => "Udyr",
            Self::Urgot        => "Urgot",
            Self::Varus        => "Varus",
            Self::Vayne        => "Vayne",
            Self::Veigar       => "Veigar",
            Self::VelKoz       => "Vel'Koz",
            Self::Vi           => "Vi",
            Self::Viktor       => "Viktor",
            Self::Vladimir     => "Vladimir",
            Self::Volibear     => "Volibear",
            Self::Warwick      => "Warwick",
            Self::Wukong       => "Wukong",
            Self::Xayah        => "Xayah",
            Self::Xerath       => "Xerath",
            Self::XinZhao      => "Xin Zhao",
            Self::Yasuo        => "Yasuo",
            Self::Yorick       => "Yorick",
            Self::Yuumi        => "Yuumi",
            Self::Zac          => "Zac",
            Self::Zed          => "Zed",
            Self::Ziggs        => "Ziggs",
            Self::Zilean       => "Zilean",
            Self::Zoe          => "Zoe",
            Self::Zyra         => "Zyra",
        }
    }

    pub fn identifier(self) -> &'static str {
        match self {
            Self::Aatrox       => "Aatrox",
            Self::Ahri         => "Ahri",
            Self::Akali        => "Akali",
            Self::Alistar      => "Alistar",
            Self::Amumu        => "Amumu",
            Self::Anivia       => "Anivia",
            Self::Annie        => "Annie",
            Self::Ashe         => "Ashe",
            Self::AurelionSol  => "AurelionSol",
            Self::Azir         => "Azir",
            Self::Bard         => "Bard",
            Self::Blitzcrank   => "Blitzcrank",
            Self::Brand        => "Brand",
            Self::Braum        => "Braum",
            Self::Caitlyn      => "Caitlyn",
            Self::Camille      => "Camille",
            Self::Cassiopeia   => "Cassiopeia",
            Self::ChoGath      => "Chogath",
            Self::Corki        => "Corki",
            Self::Darius       => "Darius",
            Self::Diana        => "Diana",
            Self::DrMundo      => "DrMundo",
            Self::Draven       => "Draven",
            Self::Ekko         => "Ekko",
            Self::Elise        => "Elise",
            Self::Evelynn      => "Evelynn",
            Self::Ezreal       => "Ezreal",
            Self::Fiddlesticks => "FiddleSticks",
            Self::Fiora        => "Fiora",
            Self::Fizz         => "Fizz",
            Self::Galio        => "Galio",
            Self::Gangplank    => "Gangplank",
            Self::Garen        => "Garen",
            Self::Gnar         => "Gnar",
            Self::Gragas       => "Gragas",
            Self::Graves       => "Graves",
            Self::Hecarim      => "Hecarim",
            Self::Heimerdinger => "Heimerdinger",
            Self::Illaoi       => "Illaoi",
            Self::Irelia       => "Irelia",
            Self::Ivern        => "Ivern",
            Self::Janna        => "Janna",
            Self::JarvanIV     => "JarvanIV",
            Self::Jax          => "Jax",
            Self::Jayce        => "Jayce",
            Self::Jhin         => "Jhin",
            Self::Jinx         => "Jinx",
            Self::KaiSa        => "Kaisa",
            Self::Kalista      => "Kalista",
            Self::Karma        => "Karma",
            Self::Karthus      => "Karthus",
            Self::Kassadin     => "Kassadin",
            Self::Katarina     => "Katarina",
            Self::Kayle        => "Kayle",
            Self::Kayn         => "Kayn",
            Self::Kennen       => "Kennen",
            Self::KhaZix       => "Khazix",
            Self::Kindred      => "Kindred",
            Self::Kled         => "Kled",
            Self::KogMaw       => "KogMaw",
            Self::LeBlanc      => "Leblanc",
            Self::LeeSin       => "LeeSin",
            Self::Leona        => "Leona",
            Self::Lissandra    => "Lissandra",
            Self::Lucian       => "Lucian",
            Self::Lulu         => "Lulu",
            Self::Lux          => "Lux",
            Self::Malphite     => "Malphite",
            Self::Malzahar     => "Malzahar",
            Self::Maokai       => "Maokai",
            Self::MasterYi     => "MasterYi",
            Self::MissFortune  => "MissFortune",
            Self::Mordekaiser  => "Mordekaiser",
            Self::Morgana      => "Morgana",
            Self::Nami         => "Nami",
            Self::Nasus        => "Nasus",
            Self::Nautilus     => "Nautilus",
            Self::Neeko        => "Neeko",
            Self::Nidalee      => "Nidalee",
            Self::Nocturne     => "Nocturne",
            Self::NunuWillump  => "Nunu",
            Self::Olaf         => "Olaf",
            Self::Orianna      => "Orianna",
            Self::Ornn         => "Ornn",
            Self::Pantheon     => "Pantheon",
            Self::Poppy        => "Poppy",
            Self::Pyke         => "Pyke",
            Self::Qiyana       => "Qiyana",
            Self::Quinn        => "Quinn",
            Self::Rakan        => "Rakan",
            Self::Rammus       => "Rammus",
            Self::RekSai       => "RekSai",
            Self::Renekton     => "Renekton",
            Self::Rengar       => "Rengar",
            Self::Riven        => "Riven",
            Self::Rumble       => "Rumble",
            Self::Ryze         => "Ryze",
            Self::Sejuani      => "Sejuani",
            Self::Shaco        => "Shaco",
            Self::Shen         => "Shen",
            Self::Shyvana      => "Shyvana",
            Self::Singed       => "Singed",
            Self::Sion         => "Sion",
            Self::Sivir        => "Sivir",
            Self::Skarner      => "Skarner",
            Self::Sona         => "Sona",
            Self::Soraka       => "Soraka",
            Self::Swain        => "Swain",
            Self::Sylas        => "Sylas",
            Self::Syndra       => "Syndra",
            Self::TahmKench    => "TahmKench",
            Self::Taliyah      => "Taliyah",
            Self::Talon        => "Talon",
            Self::Taric        => "Taric",
            Self::Teemo        => "Teemo",
            Self::Thresh       => "Thresh",
            Self::Tristana     => "Tristana",
            Self::Trundle      => "Trundle",
            Self::Tryndamere   => "Tryndamere",
            Self::TwistedFate  => "TwistedFate",
            Self::Twitch       => "Twitch",
            Self::Udyr         => "Udyr",
            Self::Urgot        => "Urgot",
            Self::Varus        => "Varus",
            Self::Vayne        => "Vayne",
            Self::Veigar       => "Veigar",
            Self::VelKoz       => "Velkoz",
            Self::Vi           => "Vi",
            Self::Viktor       => "Viktor",
            Self::Vladimir     => "Vladimir",
            Self::Volibear     => "Volibear",
            Self::Warwick      => "Warwick",
            Self::Wukong       => "MonkeyKing",
            Self::Xayah        => "Xayah",
            Self::Xerath       => "Xerath",
            Self::XinZhao      => "XinZhao",
            Self::Yasuo        => "Yasuo",
            Self::Yorick       => "Yorick",
            Self::Yuumi        => "Yuumi",
            Self::Zac          => "Zac",
            Self::Zed          => "Zed",
            Self::Ziggs        => "Ziggs",
            Self::Zilean       => "Zilean",
            Self::Zoe          => "Zoe",
            Self::Zyra         => "Zyra",
        }
    }
}

impl std::str::FromStr for Champion {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        // 4 characters encoded as an int.
        match val.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .take(4)
            .map(|c| c.to_ascii_uppercase() as u32)
            .fold(0u32, |hash, next| hash * 256 + next)
        {
            1094800466 /* AATR */ => Ok(Self::Aatrox),
            1095258697 /* AHRI */ => Ok(Self::Ahri),
            1095450956 /* AKAL */ => Ok(Self::Akali),
            1095518547 /* ALIS */ => Ok(Self::Alistar),
            1095587149 /* AMUM */ => Ok(Self::Amumu),
            1095649622 /* ANIV */ => Ok(Self::Anivia),
            1095650889 /* ANNI */ => Ok(Self::Annie),
            1095977029 /* ASHE */ => Ok(Self::Ashe),
            1096110661 /* AURE */ => Ok(Self::AurelionSol),
            1096436050 /* AZIR */ => Ok(Self::Azir),
            1111577156 /* BARD */ => Ok(Self::Bard),
            1112295764 /* BLIT */ => Ok(Self::Blitzcrank),
            1112686926 /* BRAN */ => Ok(Self::Brand),
            1112686933 /* BRAU */ => Ok(Self::Braum),
            1128352084 /* CAIT */ => Ok(Self::Caitlyn),
            1128353097 /* CAMI */ => Ok(Self::Camille),
            1128354643 /* CASS */ => Ok(Self::Cassiopeia),
            1128812359 /* CHOG */ => Ok(Self::ChoGath),
            4409423    /* CHO  */ => Ok(Self::ChoGath),
            1129271883 /* CORK */ => Ok(Self::Corki),
            1145131593 /* DARI */ => Ok(Self::Darius),
            1145651534 /* DIAN */ => Ok(Self::Diana),
            1146244437 /* DRMU */ => Ok(Self::DrMundo),
            17490      /* DR   */ => Ok(Self::DrMundo),
            1146241366 /* DRAV */ => Ok(Self::Draven),
            1162562383 /* EKKO */ => Ok(Self::Ekko),
            1162627411 /* ELIS */ => Ok(Self::Elise),
            1163281740 /* EVEL */ => Ok(Self::Evelynn),
            1163547205 /* EZRE */ => Ok(Self::Ezreal),
            1179206724 /* FIDD */ => Ok(Self::Fiddlesticks),
            1179209554 /* FIOR */ => Ok(Self::Fiora),
            1179212378 /* FIZZ */ => Ok(Self::Fizz),
            1195461705 /* GALI */ => Ok(Self::Galio),
            1195462215 /* GANG */ => Ok(Self::Gangplank),
            1195463237 /* GARE */ => Ok(Self::Garen),
            1196310866 /* GNAR */ => Ok(Self::Gnar),
            1196572999 /* GRAG */ => Ok(Self::Gragas),
            1196573014 /* GRAV */ => Ok(Self::Graves),
            1212498753 /* HECA */ => Ok(Self::Hecarim),
            1212500301 /* HEIM */ => Ok(Self::Heimerdinger),
            1229737025 /* ILLA */ => Ok(Self::Illaoi),
            1230128460 /* IREL */ => Ok(Self::Irelia),
            1230390610 /* IVER */ => Ok(Self::Ivern),
            1245793870 /* JANN */ => Ok(Self::Janna),
            1245794902 /* JARV */ => Ok(Self::JarvanIV),
            4866392    /* JAX  */ => Ok(Self::Jax),
            1245796675 /* JAYC */ => Ok(Self::Jayce),
            1246251342 /* JHIN */ => Ok(Self::Jhin),
            1246318168 /* JINX */ => Ok(Self::Jinx),
            1262569811 /* KAIS */ => Ok(Self::KaiSa),
            4931913    /* KAI  */ => Ok(Self::KaiSa),
            1262570569 /* KALI */ => Ok(Self::Kalista),
            1262572109 /* KARM */ => Ok(Self::Karma),
            1262572116 /* KART */ => Ok(Self::Karthus),
            1262572371 /* KASS */ => Ok(Self::Kassadin),
            1262572609 /* KATA */ => Ok(Self::Katarina),
            1262573900 /* KAYL */ => Ok(Self::Kayle),
            1262573902 /* KAYN */ => Ok(Self::Kayn),
            1262833230 /* KENN */ => Ok(Self::Kennen),
            1263026522 /* KHAZ */ => Ok(Self::KhaZix),
            4933697    /* KHA  */ => Ok(Self::KhaZix),
            1263095364 /* KIND */ => Ok(Self::Kindred),
            1263289668 /* KLED */ => Ok(Self::Kled),
            1263486797 /* KOGM */ => Ok(Self::KogMaw),
            4935495    /* KOG  */ => Ok(Self::KogMaw),
            1279607372 /* LEBL */ => Ok(Self::LeBlanc),
            1279608147 /* LEES */ => Ok(Self::LeeSin),
            4998469    /* LEE  */ => Ok(Self::LeeSin),
            1279610702 /* LEON */ => Ok(Self::Leona),
            1279873875 /* LISS */ => Ok(Self::Lissandra),
            1280656201 /* LUCI */ => Ok(Self::Lucian),
            1280658517 /* LULU */ => Ok(Self::Lulu),
            5002584    /* LUX  */ => Ok(Self::Lux),
            1296125008 /* MALP */ => Ok(Self::Malphite),
            1296125018 /* MALZ */ => Ok(Self::Malzahar),
            1296125771 /* MAOK */ => Ok(Self::Maokai),
            1296126804 /* MAST */ => Ok(Self::MasterYi),
            1296651091 /* MISS */ => Ok(Self::MissFortune),
            1297044036 /* MORD */ => Ok(Self::Mordekaiser),
            1297044039 /* MORG */ => Ok(Self::Morgana),
            1312902473 /* NAMI */ => Ok(Self::Nami),
            1312904021 /* NASU */ => Ok(Self::Nasus),
            1312904532 /* NAUT */ => Ok(Self::Nautilus),
            1313162571 /* NEEK */ => Ok(Self::Neeko),
            1313424449 /* NIDA */ => Ok(Self::Nidalee),
            1313817428 /* NOCT */ => Ok(Self::Nocturne),
            1314213461 /* NUNU */ => Ok(Self::NunuWillump),
            1330397510 /* OLAF */ => Ok(Self::Olaf),
            1330792769 /* ORIA */ => Ok(Self::Orianna),
            1330794062 /* ORNN */ => Ok(Self::Ornn),
            1346457172 /* PANT */ => Ok(Self::Pantheon),
            1347375184 /* POPP */ => Ok(Self::Poppy),
            1348029253 /* PYKE */ => Ok(Self::Pyke),
            1363761473 /* QIYA */ => Ok(Self::Qiyana),
            1364543822 /* QUIN */ => Ok(Self::Quinn),
            1380010817 /* RAKA */ => Ok(Self::Rakan),
            1380011341 /* RAMM */ => Ok(Self::Rammus),
            1380272979 /* REKS */ => Ok(Self::RekSai),
            5391691    /* REK  */ => Ok(Self::RekSai),
            1380273733 /* RENE */ => Ok(Self::Renekton),
            1380273735 /* RENG */ => Ok(Self::Rengar),
            1380537925 /* RIVE */ => Ok(Self::Riven),
            1381322050 /* RUMB */ => Ok(Self::Rumble),
            1381587525 /* RYZE */ => Ok(Self::Ryze),
            1397049941 /* SEJU */ => Ok(Self::Sejuani),
            1397244227 /* SHAC */ => Ok(Self::Shaco),
            1397245262 /* SHEN */ => Ok(Self::Shen),
            1397250390 /* SHYV */ => Ok(Self::Shyvana),
            1397313095 /* SING */ => Ok(Self::Singed),
            1397313358 /* SION */ => Ok(Self::Sion),
            1397315145 /* SIVI */ => Ok(Self::Sivir),
            1397440850 /* SKAR */ => Ok(Self::Skarner),
            1397706305 /* SONA */ => Ok(Self::Sona),
            1397707329 /* SORA */ => Ok(Self::Soraka),
            1398227273 /* SWAI */ => Ok(Self::Swain),
            1398361153 /* SYLA */ => Ok(Self::Sylas),
            1398361668 /* SYND */ => Ok(Self::Syndra),
            1413564493 /* TAHM */ => Ok(Self::TahmKench),
            1413565513 /* TALI */ => Ok(Self::Taliyah),
            1413565519 /* TALO */ => Ok(Self::Talon),
            1413567049 /* TARI */ => Ok(Self::Taric),
            1413825869 /* TEEM */ => Ok(Self::Teemo),
            1414025797 /* THRE */ => Ok(Self::Thresh),
            1414678867 /* TRIS */ => Ok(Self::Tristana),
            1414681934 /* TRUN */ => Ok(Self::Trundle),
            1414682958 /* TRYN */ => Ok(Self::Tryndamere),
            1415006547 /* TWIS */ => Ok(Self::TwistedFate),
            1415006548 /* TWIT */ => Ok(Self::Twitch),
            1430542674 /* UDYR */ => Ok(Self::Udyr),
            1431455567 /* URGO */ => Ok(Self::Urgot),
            1447121493 /* VARU */ => Ok(Self::Varus),
            1447123278 /* VAYN */ => Ok(Self::Vayne),
            1447381319 /* VEIG */ => Ok(Self::Veigar),
            1447382091 /* VELK */ => Ok(Self::VelKoz),
            5653836    /* VEL  */ => Ok(Self::VelKoz),
            22089      /* VI   */ => Ok(Self::Vi),
            1447643988 /* VIKT */ => Ok(Self::Viktor),
            1447838020 /* VLAD */ => Ok(Self::Vladimir),
            1448037449 /* VOLI */ => Ok(Self::Volibear),
            1463898711 /* WARW */ => Ok(Self::Warwick),
            1465207631 /* WUKO */ => Ok(Self::Wukong),
            1297043019 /* MONK */ => Ok(Self::Wukong),
            1480677697 /* XAYA */ => Ok(Self::Xayah),
            1480938049 /* XERA */ => Ok(Self::Xerath),
            1481199194 /* XINZ */ => Ok(Self::XinZhao),
            5785934    /* XIN  */ => Ok(Self::XinZhao),
            1497453397 /* YASU */ => Ok(Self::Yasuo),
            1498370633 /* YORI */ => Ok(Self::Yorick),
            1498764621 /* YUUM */ => Ok(Self::Yuumi),
            5914947    /* ZAC  */ => Ok(Self::Zac),
            5915972    /* ZED  */ => Ok(Self::Zed),
            1514751815 /* ZIGG */ => Ok(Self::Ziggs),
            1514753093 /* ZILE */ => Ok(Self::Zilean),
            5918533    /* ZOE  */ => Ok(Self::Zoe),
            1515803201 /* ZYRA */ => Ok(Self::Zyra),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Champion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
