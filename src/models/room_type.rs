use prost::Enumeration;
use serde::Serialize;

#[derive(Debug, Enumeration, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum RoomType {
    Any = 0,
    Regular = 1,
    TrainingRoom = 2,
    Tournament = 4,
    QuickTournament = 5,
    Rating = 7,
    MadGames = 8,
    RealisticBattles = 22,
    Uprising = 23,
    GravityMode = 24,
    Skirmish = 25,
    BurningGames = 26,
}
