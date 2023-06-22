use rand::prelude::*;
const POPULATION_CHANCE: f32 = 0.1;
const SIZE: usize = 10_000;

fn main() {
    let world = vec![vec![false; SIZE]; SIZE];
    let mut world = populate_world(world);
}

fn print_world_to_console(world: &Vec<Vec<bool>>) {
    for vec in world.iter().take(SIZE) {
        println!();
        for bool in vec.iter().take(SIZE) {
            match bool {
                true => print!("1"),
                false => print!(" "),
            }
        }
    }
}
fn populate_world(mut world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rand = thread_rng();
    for i in 0..SIZE {
        for j in 0..SIZE {
            if rand.gen_range(0.0..=1.) <= POPULATION_CHANCE * (chance_calc(&i)) * (chance_calc(&j))
            {
                world[i][j] = true;
            }
        }
    }
    world
}

fn chance_calc(&x: &usize) -> f32 {
    if x <= SIZE / 2 {
        ((x as f32 * 2.) / SIZE as f32) * 2.
    } else {
        ((SIZE as f32 - x as f32) * 2. / SIZE as f32) * 2.
    }
}

#[test]
fn chance_test() {
    assert_eq!(chance_calc(&32), 0.64);
    assert_eq!(chance_calc(&70), chance_calc(&30));
    assert_eq!(chance_calc(&20), chance_calc(&80));
}
