use serde_json;
use serde::{Deserialize, Deserializer};

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
    pub arranged: Vec<(usize, Option<String>)>,
    pub vec_of_cards_to_decide: Vec<usize>,
    pub inked_cards: Vec<usize>,
    pub hand: Vec<usize>,
    pub draft: Vec<usize>,
    pub discard: Vec<usize>,
    pub lockup: Vec<usize>,
    pub rotated_cards: Vec<usize>,
    pub skip_cards:Vec<usize>
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
            vec_of_cards_to_decide: vec![],
            inked_cards: vec![],
            hand: vec![],
            draft: vec![],
            discard: vec![],
            lockup: vec![],
            rotated_cards: vec![],
            skip_cards:vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameCommand {
    pub use_ink: Option<usize>,
    pub use_remover: Option<usize>,
    pub arranged: Option<Vec<(usize, Option<String>)>>,
    pub submit_word: Option<bool>,
    pub reply: Option<usize>,
    pub buy_offer: Option<(bool,usize)>,
    pub lockup:Option<(bool,usize)>,
    pub buy_lockup: Option<(bool,usize)>,
    pub killserver: Option<bool>,
    pub trash_other:Option<(bool,usize)>,
    pub putback_discard:Option<bool>
}
impl GameCommand {
    pub fn new() -> Self {
        GameCommand {
            use_ink: None,
            use_remover: None,
            arranged: None,
            submit_word: None,
            reply: None,
            buy_offer: None,
            lockup:None,
            buy_lockup: None,
            killserver: None,
            trash_other:None,
            putback_discard:None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub struct BoardCodec {
    pub players: Vec<Player>,
    pub gamestates: Vec<GameState>,
    pub offer_row: Vec<usize>,
    pub turn_index:usize,
    pub ticks:Option<u16>
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum GameState {
    Spell,
    TurnToSubmit,
    Buy,
    DrawCard,
    ResolvePurchase,
    WaitForReply,
    ResolveAgain(Option<usize>,usize),
    LockUp,
    TrashOther,
    PutBackDiscard(usize,usize)
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum Replay{
    Play(u16),
    Pause(u16),
    Exit
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum ClientError{
    NotConnectedToInternet,
    CannotFindServer
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
    (message,set_message,String),
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
    (turn_index,set_turn_index,usize),
    (request,set_request,(usize,usize,String,Vec<String>,Option<u16>)),
    (reason,set_reason,String),
    (optional,set_optional,bool),
    (location,set_location,String),
    (sender,set_sender,String),
    (message,set_message,String),
    (log,set_log,String),
    (error,set_error,ClientError)
    },rename_optional:{ (type_name,set_type_name,String,"type"),},else:{}
}