use prost::Enumeration;
use serde::{Deserialize, Serialize};
use serde_with::rust::deserialize_ignore_any;

#[derive(Debug, Deserialize, Serialize, Enumeration)]
#[serde(untagged)]
pub enum MapId {
    DesertSands = 2,
    Middleburg = 3,
    Copperfield = 4,
    Mines = 6,
    DeadRail = 7,
    FortDespair = 8,
    Himmelsdorf = 9,
    OasisPalms = 11,
    Molendijk = 14,
    PortBay = 15,
    Castilla = 20,
    Canal = 21,
    YamatoHarbor = 25,
    Canyon = 27,
    MayanRuins = 30,
    NavalFrontier = 35,
    Alpenstadt = 38,
    NewBay = 40,
    Normandy = 42,

    /// Default value for Protocol Buffers and fallback value for Serde.
    #[serde(deserialize_with = "deserialize_ignore_any")]
    Undefined = 0,
}
