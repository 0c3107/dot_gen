use rand::prelude::*;
const POPULATION_CHANCE: f32 = 0.01;

fn main() {
    let world = vec![vec![0u32; 100]; 100];
    let world = populate_world(world);
    println!("{:?}", world);
}

fn populate_world(mut world: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rand = thread_rng();
    for i in 0..100 {
        for j in 0..100 {
            if rand.gen_range(0.0..=1.) <= POPULATION_CHANCE {
                world[i][j] = 1;
            }
        }
    }
    world
}
