use serde_json;
use serde::{Deserialize, Deserializer};
use cards;

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de>
{
    Ok(Some(Option::deserialize(deserializer)?))
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableInfo {
    pub numberOfPlayers: usize,
    pub players: Vec<String>,
}
impl TableInfo {
    pub fn new(players: Vec<String>, numberOfPlayers: usize) -> TableInfo {
        TableInfo {
            numberOfPlayers: numberOfPlayers,
            players: players,
        }
    }
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PrivateInformation {}
#[derive(Serialize, Deserialize,Debug, Clone,PartialEq)]
pub struct Player {
    pub name: String,
    pub vp: usize,
    pub coin: usize,
    pub ink: usize,
    pub remover: usize,
    pub arranged: Vec<usize>,
    pub wild: Vec<Option<String>>,
    pub inked_cards: Vec<usize>,
    pub hand: Vec<usize>,
    pub draft: Vec<usize>,
    pub discard: Vec<usize>,
    pub lockup: Vec<usize>,
    pub rotated_cards: Vec<usize>,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            vp: 0,
            coin: 0,
            ink: 0,
            remover: 0,
            arranged: vec![],
            wild: vec![],
            inked_cards: vec![],
            hand: vec![],
            draft: vec![],
            discard: vec![],
            lockup: vec![],
            rotated_cards: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameCommand {
    pub use_ink: Option<usize>,
    pub use_remover: Option<usize>,
    pub arranged: Option<Vec<usize>>,
    pub wild: Option<(usize, String)>,
    pub submit_word: Option<bool>,
    pub reply: Option<usize>,
    pub buyoffer: Option<usize>,
    pub buylockup: Option<usize>,
    pub killserver: Option<bool>,
}
impl GameCommand {
    pub fn new() -> Self {
        GameCommand {
            use_ink: None,
            use_remover: None,
            arranged: None,
            wild: None,
            submit_word: None,
            reply: None,
            buyoffer: None,
            buylockup: None,
            killserver: None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub struct BoardCodec {
    pub players: Vec<Player>,
}
CGM_codec!{
    structname:ServerReceivedMsg,
    rename:{
    },optional:{
    (gamecommand,set_gamecommand,GameCommand),
   (newTable,set_new_table,bool),
    (ready,set_ready,bool),
    (joinTable,set_join_table,usize),
    (changePlayers,set_change_player,usize),
    (leaveTable,set_leave_table,bool),
    (joinLobby,set_join_lobby,bool),
    (namechange,set_name_change,String),
    (chat,set_chat,String),
    (location,set_location,String),
    },rename_optional:{},else:{}
}
CGM_codec!{
    structname:ClientReceivedMsg,
   rename:{
    },optional:{
    (tables,set_tables,Vec<TableInfo>),
    (tablenumber,set_tablenumber,usize),
    (players,set_players,Vec<Player>),
    (privateInformation,set_private_information,PrivateInformation),
    (boardstate,set_boardstate,Result<BoardCodec,String>),
    (request,set_request,(String,Vec<String>)),
    (reason,set_reason,String),
    (optional,set_optional,bool),
    (location,set_location,String),
    (sender,set_sender,String),
    (message,set_message,String),
    (log,set_log,String),
    },rename_optional:{ (type_name,set_type_name,String,"type"),},else:{}
}
