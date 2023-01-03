use rand::prelude::*;
/* want to find the expected value of the threes game
 * roll 5 dice
 * must keep at least one dice each roll
 * 3s count as 0
 */
fn main() {
    let runs = 10000;
    let run_values: Vec<u64> = (1..runs).map(|_| run_round()).collect();
    println!("{:?}", run_values);

}

fn value_of(throw: u64) -> u64 {
    match throw {
        3 => 0,
        x => x,
    }
}

use rand::distributions::Uniform;

fn run_round() -> u64 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    let mut num_dice = 5;
    let mut total = 0;
    while num_dice > 0 {
        let mut throw_values: Vec<u64> = (1..num_dice+1).map(|_| value_of(rng.sample(&die))).collect();
        throw_values.sort();
        // grab the first element(least value). 
        // If there are more 0s, grab all of them.
        if throw_values[0] == 0 {
            let num_threes = 
                throw_values.iter().filter(|&n| *n == 0).count();
            num_dice -= num_threes;
        } else {
            // no threes
            total += throw_values[0];
            num_dice -= 1;
        }
    }
    total
}

