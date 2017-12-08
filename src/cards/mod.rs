use codec::{Player, GameState};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub letter: String,
    pub index: usize,
    pub inked: bool,
}
impl Card {
    pub fn inked(&mut self) {
        self.inked = true;
    }
    /*   pub fn wild_with(&mut self,l:& str){
        self.letter = l;
    }
*/
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum GIVEABLE {
    NONE,
    VP(usize),
    COIN(usize),
    VPCOIN(usize, usize),
    COININK(usize),
    VPINK(usize),
    VPORCOIN(usize),
    VPORCOININK(usize),
    INK,
}
#[derive(Clone,Debug)]
pub enum Genre {
    MYSTERY,
    HORROR,
    ADVENTURE,
    ROMANCE,
    NONE,
}
pub type WaitForSingleInput = (usize,
                               GameState,
                               String,
                               Vec<(GameState,
                                    String,
                                    Box<Fn(&mut Player, &mut Vec<usize>, &mut Vec<usize>)>)>);
//card_index,waitstate,header,Optionvec
pub type WaitForInputType = Vec<Option<WaitForSingleInput>>;
//Option if none, just broadcast boardcodec
pub struct ListCard<T> {
    pub id: usize,
    pub letter: &'static str,
    pub cost: usize,
    pub purchase_giveables: GIVEABLE,
    pub giveables: GIVEABLE,
    pub genre_giveables: GIVEABLE,
    pub trash: GIVEABLE,
    pub genre: Genre,
    pub timeless: bool,
    pub giveablefn: Option<Box<Fn(&mut T, usize, usize, &mut [WaitForInputType; 4])>>,
    pub genrefn: Option<Box<Fn(&mut T, usize, usize, &mut [WaitForInputType; 4])>>,
}
pub trait Board {
    fn two_cent_per_adv(&mut self,
                        player_id: usize,
                        card_index: usize,
                        wait_for_input: &mut [WaitForInputType; 4]);
    fn minus_other_ink(&mut self,
                       player_id: usize,
                       card_index: usize,
                       wait_for_input: &mut [WaitForInputType; 4]);
    fn lockup_offer(&mut self,
                    player_id: usize,
                    card_index: usize,
                    wait_for_input: &mut [WaitForInputType; 4]);
    fn uncover_adjacent(&mut self,
                        player_id: usize,
                        card_index: usize,
                        wait_for_input: &mut [WaitForInputType; 4]);
    fn double_adjacent(&mut self,
                       player_id: usize,
                       card_index: usize,
                       wait_for_input: &mut [WaitForInputType; 4]);
    fn trash_other(&mut self,
                   player_id: usize,
                   card_index: usize,
                   wait_for_input: &mut [WaitForInputType; 4]);
    fn one_vp_per_wild(&mut self,
                       player_id: usize,
                       card_index: usize,
                       wait_for_input: &mut [WaitForInputType; 4]);
    fn putback_or_discard_three(&mut self,
                                player_id: usize,
                                card_index: usize,
                                wait_for_input: &mut [WaitForInputType; 4]);
}
macro_rules! listcard_map {
    (structtype:$s_alias:ty,
cards:{  $(($id:expr,$letter:expr,$cost:expr,$purchase_giveables:expr,$giveables:expr,$genre_giveables:expr,$trash:expr,$genre:expr,$timeless:expr,$giveablefn:expr,$genrefn:expr)),* $(,)*
})
        => {
         let cards:[ListCard<$s_alias>;180] =[
             $(ListCard{
                  id:$id,
                  letter:$letter,
                  cost:$cost,
                  purchase_giveables:$purchase_giveables,
                  giveables:$giveables,
                  genre_giveables:$genre_giveables,
                  trash:$trash,
                  genre:$genre,
                  timeless:$timeless,
                  giveablefn:$giveablefn,
                  genrefn:$genrefn
             },)*
         ];
         cards
    }}
pub fn populate<T: Board>() -> [ListCard<T>; 180] {
    listcard_map!{
        structtype:T,
        cards:{
        (0,"a",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (1,"b",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (2,"c",3,GIVEABLE::VP(1),GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (3,"d",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (4,"e",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (5,"f",8,GIVEABLE::VP(1),GIVEABLE::VP(5),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (6,"g",6,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::COIN(1),GIVEABLE::COIN(4),Genre::ADVENTURE,false,None,None),
        (7,"h",3,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None,None),
        (8,"i",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (9,"j",5,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (10,"k",9,GIVEABLE::VP(2),GIVEABLE::VP(5),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (11,"l",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (12,"m",6,GIVEABLE::VP(3),GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (13,"n",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (14,"o",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //genre, 2cents for every adv
            b.two_cent_per_adv(p,c,w);
        }))),
        (15,"p",4,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (16,"q",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(3),Genre::ADVENTURE,false,None,None),
        (17,"r",3,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (18,"s",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (19,"t",4,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (20,"u",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(3),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (21,"v",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (22,"w",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (23,"x",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (24,"y",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(1),Genre::ADVENTURE,false,None,None),
        (25,"z",5,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (26,"a",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(3),Genre::ADVENTURE,false,None,None),
        (27,"c",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (28,"g",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (29,"i",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (30,"j",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (31,"p",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (32,"l",2,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None,None),
        (33,"w",5,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (34,"y",4,GIVEABLE::VP(4),GIVEABLE::COIN(2),GIVEABLE::VP(2),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (35,"b",6,GIVEABLE::NONE,GIVEABLE::COIN(3),GIVEABLE::COININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (36,"c",5,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (37,"d",9,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (38,"e",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (39,"f",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (40,"g",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (41,"h",7,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(2,1),GIVEABLE::NONE,Genre::HORROR,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //horror, genre other player -1 ink/remover
            b.minus_other_ink(p,c,w);
        }))),
        (42,"i",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (43,"j",5,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (44,"k",2,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (45,"l",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::INK,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (46,"m",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (47,"n",5,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (48,"o",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VPORCOIN(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (49,"p",3,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (50,"q",4,GIVEABLE::NONE,GIVEABLE::COIN(3),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (51,"r",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::COININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (52,"s",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (53,"t",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPINK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (54,"u",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (55,"v",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (56,"w",4,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (57,"x",6,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (58,"y",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (59,"z",3,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VPORCOININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (60,"v",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,true,None,None),
        (61,"x",2,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (62,"w",5,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::COIN(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (63,"u",6,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (64,"n",6,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,true,None,None),
        (65,"s",7,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (66,"c",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::COIN(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (67,"e",5,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (68,"d",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (69,"a",3,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (70,"b",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (71,"c",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover
            b.uncover_adjacent(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen:Lock up offer row
            b.lockup_offer(p,c,w);
        }))),
        (72,"d",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent wild
            b.uncover_adjacent(p,c,w);
        })),None),
        (73,"e",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:uncover adjacent wild
            b.uncover_adjacent(p,c,w);
        }))),
        (74,"f",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (75,"g",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (76,"h",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,) Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (77,"i",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (78,"j",8,GIVEABLE::NONE,GIVEABLE::VP(5),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (79,"k",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (80,"l",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (81,"m",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (82,"n",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:all wild cards +vp
            b.one_vp_per_wild(p,c,w);
        }))),
        (83,"o",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (84,"p",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (85,"q",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (86,"r",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (87,"s",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (88,"t",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (89,"u",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (90,"v",9,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(4),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (91,"w",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (92,"x",3,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (93,"y",7,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (94,"z",5,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
        }))),
        (95,"i",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: lockup after rowcard
            b.lockup_offer(p,c,w);
        }))),
        (96,"a",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,true,None,None),
        (97,"f",5,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
        (98,"m",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: lockup offer row
            b.lockup_offer(p,c,w);
        }))),
        (99,"k",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (100,"q",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (101,"t",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:lockup offer row
            b.lockup_offer(p,c,w);
        }))),
        (102,"r",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:Lockup
            b.lockup_offer(p,c,w);
        }))),
        (103,"p",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:lockup
            b.lockup_offer(p,c,w);
        }))),
        (104,"a",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent
            b.uncover_adjacent(p,c,w);
        })),None),
        (105,"z",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),None),
        (106,"w",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //rommanc,  gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (107,"v",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (108,"u",9,GIVEABLE::NONE,GIVEABLE::VP(5),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (109,"t",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen: thrash other
            b.trash_other(p,c,w);
        }))),
        (110,"s",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (111,"r",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:keep or discard top3
            b.putback_or_discard_three(p,c,w);
        }))),
        (112,"q",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (113,"p",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (114,"o",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (115,"n",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (116,"m",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (117,"l",8,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (118,"k",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (119,"j",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (120,"i",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,None),
        (121,"h",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash
            b.trash_other(p,c,w);
        })),None),
        (122,"g",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (123,"f",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (124,"e",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (125,"d",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (126,"c",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (127,"b",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),None),
        (128,"a",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        (129,"b",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (130,"e",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,None),
        (131,"f",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (132,"h",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:double adjacent
            b.double_adjacent(p,c,w);
        }))),
        (133,"k",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:trash other
            b.trash_other(p,c,w);
        }))),
        (134,"n",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:trash other
            b.trash_other(p,c,w);
        })),None),
        (135,"o",8,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ROMANCE,true,None,None),
        (136,"r",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:trash
            b.trash_other(p,c,w);
        }))),
        (137,"z",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:trash other
            b.trash_other(p,c,w);
        })),None),
        (138,"y",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:thrash other
            b.trash_other(p,c,w);
        }))),
        (139,"x",7,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:trash other card
            b.trash_other(p,c,w);
        })),None),
        (140,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (141,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (142,"c",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (143,"d",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (144,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (145,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (146,"g",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (147,"h",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (148,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (149,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (150,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (151,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (152,"m",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (153,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (154,"o",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (155,"p",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (156,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (157,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (158,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (159,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (160,"u",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (161,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (162,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (163,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (164,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (165,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (166,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (167,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (168,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (169,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (170,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (171,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (172,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (173,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (174,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (175,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (176,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (177,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (178,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (179,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        }
        
    }
}
