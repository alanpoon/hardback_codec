use cards::*;
use cards;
use codec::*;
pub struct BoardStruct {}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self, player_id: usize, card_id: usize) {}
    fn minus_other_ink(&mut self, player_id: usize, card_id: usize) {}
    fn lockup_offer(&mut self, player_id: usize, card_id: usize) {}
    fn uncover_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn double_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn trash_other(&mut self, player_id: usize, card_id: usize) {}
    fn one_vp_per_wild(&mut self, player_id: usize, card_id: usize) {}
    fn keep_or_discard_three(&mut self, player_id: usize, card_id: usize) {}
}
#[test]
fn player_starting() {
    let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
    let mut remaining_deck = vec![];
    let mut _p = Player::new("defaultname".to_owned());
    _p.starting::<BoardStruct>(&cardmeta, &mut remaining_deck);
    assert_eq!(_p.draft.len(), 5);
    assert_eq!(_p.hand.len(), 5);
}
