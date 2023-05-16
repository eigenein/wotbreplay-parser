use prost::Enumeration;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Enumeration, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
#[repr(u16)]
pub enum MapId {
    DesertSands = 2,
    Middleburg = 3,
    Copperfield = 4,
    Mines = 6,
    DeadRail = 7,
    FortDespair = 8,
    Himmelsdorf = 9,
    BlackGoldville = 10,
    OasisPalms = 11,
    GhostFactory = 12,
    Molendijk = 14,
    PortBay = 15,
    WinterMalinovka = 19,
    Castilla = 20,
    Canal = 21,
    Vineyards = 23,
    YamatoHarbor = 25,
    Canyon = 27,
    MayanRuins = 30,
    DynastyPearl = 31,
    NavalFrontier = 35,
    Alpenstadt = 38,
    NewBay = 40,
    Normandy = 42,
    Wasteland = 71,

    /// Default value for Protocol Buffers and fallback value for Serde.
    #[serde(deserialize_with = "serde_with::rust::deserialize_ignore_any")]
    Undefined = 0,
}
