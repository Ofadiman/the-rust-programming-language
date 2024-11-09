use rand::seq::SliceRandom;
use rand::{self, distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone, Debug)]
enum Effect {
    Regeneration,
    Poison,
}

#[derive(Clone, Debug)]
enum Roll {
    One,
    Two,
    Three,
    Four,
    Five,
    Six(Effect),
}

impl Distribution<Roll> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Roll {
        let option = [
            Roll::One,
            Roll::Two,
            Roll::Three,
            Roll::Four,
            Roll::Five,
            Roll::Six(Effect::Poison),
            Roll::Six(Effect::Regeneration),
        ]
        .choose(rng);

        if let Some(roll) = option {
            roll.clone()
        } else {
            panic!("couldn't roll")
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let roll: Roll = rand::random();

    match roll {
        Roll::One => {
            println!("one!");
        }
        Roll::Two => {
            println!("two!");
        }
        Roll::Three => {
            println!("three!");
        }
        Roll::Six(effect) => match effect {
            Effect::Regeneration => {
                println!("six with regeneration!")
            }
            Effect::Poison => {
                println!("six with poison!")
            }
        },
        other => {
            println!("four or five!")
        }
    }
}
