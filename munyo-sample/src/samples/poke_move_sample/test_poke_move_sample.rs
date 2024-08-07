use crate::samples::poke_move_sample::{
    basic_move_syntax::{top_to_basic_move_special, Top},
    data_types::BasicMove,
};

const SPECIAL_MOVES: &str = r###"
|| Every move here inflicts special-type damage.
>>>Move
|| MoveName | Type | Power | Additional Information
FlameThrower Fire 90 Burn(10%) PP15  
Overheat     Fire 130 90% JC-2  PP5
Snarl        Dark 55 95% C-1    PP15 Sound 
DarkPulse    Dark 80 Flinch(20%) PP15

|| C-1 means lowering opponent's Special-Atk by 1
|| JC-2 means lowering your Special-Atk by 2
"###;

#[test]
fn test() -> munyo::Result<()> {
    let r: Vec<Top> = munyo::from_str(SPECIAL_MOVES)?;
    let r: Vec<BasicMove> = r.into_iter().map(top_to_basic_move_special).collect();
    println!("{:?}", r);

    Ok(())
}
