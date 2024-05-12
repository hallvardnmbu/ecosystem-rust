use crate::animals::Species::{Carnivore, Herbivore};

pub mod animals;
pub mod island;

fn main() {
    let mut rng = rand::thread_rng();

    let geography: Vec<&str> = vec![
        "WWW",
        "WLW",
        "WLW",
        "WWW"
    ];

    let mut isl = island::Island::new(geography, &mut rng);

    isl.add_population(vec![
        ((1, 1), Herbivore, 10),
        ((1, 1), Carnivore, 2)
    ]);

    println!("{:?}", isl.geography);

    println!("{:?}", isl.inhabited);
    isl.yearly_cycle();
    println!("{:?}", isl.inhabited);
    isl.yearly_cycle();
    println!("{:?}", isl.inhabited);

    println!("{:#?}", isl.cells);
    println!("{:#?}", isl.cells[&(1, 1)].animals);
    println!("{:#?}", isl.cells[&(2, 1)].animals);
}
