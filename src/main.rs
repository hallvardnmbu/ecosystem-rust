use rand::Rng;

pub mod animals;
mod island;

fn main() {
    let mut rng = rand::thread_rng();

    let mut herb = animals::Herbivore {
            weight: 5.0,
            age: 0,
            fitness: None,
    };
    herb.aging();

    println!("Fitness: {}", herb.fitness());

    let geography = vec![vec!['W', 'W', 'W'], vec!['W', 'L', 'W'], vec!['W', 'W', 'W']];

    let mut isl = island::Island::new(geography, &mut rng);

    println!("Cells: {:?}", isl.cells);
    println!("Checking the rng of Island: u8: {} f32: {}", isl.rng.gen::<u8>(), isl.rng.gen::<f32>
    ());

    isl.procreate();
}
