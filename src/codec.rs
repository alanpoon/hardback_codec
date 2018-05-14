use serde_json;
use serde::{Deserialize, Deserializer};
use std::time::Instant;
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
    pub literacy_award: usize,
    pub arranged: Vec<(usize, bool, Option<String>, bool)>,
    pub vec_of_cards_to_decide: Vec<usize>,
    pub hand: Vec<usize>, //bool:true=>Inked,false=>Normal
    pub draft: Vec<usize>, //only used in show draft
    pub draftlen: usize,
    pub lockup: Vec<usize>,
    pub timeless_classic: Vec<usize>,
    pub discard:Vec<usize>,
    pub skip_cards: Vec<usize>, // delay resolve
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            vp: 0,
            coin: 0,
            ink: 0,
            remover: 0,
            literacy_award: 0,
            arranged: vec![],
            vec_of_cards_to_decide: vec![],
            hand: vec![],
            draft: vec![],
            draftlen: 5,
            lockup: vec![],
            timeless_classic: vec![],
            skip_cards: vec![],
            discard:vec![]
        }
    }
}
#[derive( Serialize, Deserialize,Debug, Clone,PartialEq)]
pub struct Personal {
    pub hand: Vec<usize>,
    pub arranged: Vec<(usize, bool, Option<String>, bool)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameCommand {
    pub go_to_shuffle: Option<bool>,
    pub take_card_use_ink: Option<bool>,
    pub use_ink: Option<usize>,
    pub use_remover: Option<Vec<usize>>,
    pub arranged: Option<Vec<(usize, bool, Option<String>, bool)>>, //arranged can be used interchangably with personal
    pub personal: Option<Personal>,
    pub submit_word: Option<bool>,
    pub reply: Option<usize>,
    pub buy_offer: Option<(bool, usize)>,
    pub lockup: Option<(bool, usize)>,
    pub buy_lockup: Option<(bool, usize)>,
    pub killserver: Option<bool>,
    pub trash_other: Option<(bool, usize)>,
    pub putback_discard: Option<bool>,
    pub exit_game:Option<bool>
}
impl GameCommand {
    pub fn new() -> Self {
        GameCommand {
            go_to_shuffle: None,
            take_card_use_ink: None,
            use_ink: None,
            use_remover: None,
            arranged: None,
            personal: None,
            submit_word: None,
            reply: None,
            buy_offer: None,
            lockup: None,
            buy_lockup: None,
            killserver: None,
            trash_other: None,
            putback_discard: None,
            exit_game:None
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub struct BoardCodec {
    pub players: Vec<Player>,
    pub gamestates: Vec<GameState>,
    pub offer_row: Vec<usize>,
    pub turn_index: usize,
    pub ticks: Option<u16>,
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum GameState {
    ShowDraft,
    Shuffle,
    PreSpell,
    Spell,
    PreTurnToSubmit,
    TurnToSubmit,
    PreBuy,
    Buy,
    PreDrawCard,
    DrawCard,
    ResolvePurchase,
    PreWaitForReply,
    WaitForReply,
    ResolveAgain(Option<usize>, usize),
    LockUp,
    PreTrashOther(usize),
    TrashOther(usize),
    PrePutBackDiscard(usize,usize),
    PutBackDiscard(usize, usize),
    ShowResult(usize),//winner
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum Replay {
    Play(u16),
    Pause(u16),
    Exit,
}

#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum ConnectionError {
    NotConnectedToInternet,
    CannotFindServer,
    InvalidDestination,
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
#[serde(tag = "connection_status", content = "c")]
pub enum ConnectionStatus {
    None,
    Try(Instant),
    Error(ConnectionError),
    Ok,
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
    (privateInformation,set_private_information,PrivateInformation),
    (boardstate,set_boardstate,Result<BoardCodec,String>),
    (hand,set_hand,Vec<usize>),
    (player_index,set_player_index,usize),
    (turn_index,set_turn_index,usize),
    (request,set_request,(usize,usize,String,Vec<String>,Option<u16>)),
    (notification,set_notification,String),
    (location,set_location,String),
    (sender,set_sender,String),
    (message,set_message,String),
    (log,set_log,String),
    (ready,set_ready,bool),
    (connection_status,set_connection_status,ConnectionStatus)
    },rename_optional:{ (type_name,set_type_name,String,"type"),},else:{}
}
