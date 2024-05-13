use crate::animals::Species::{Carnivore, Herbivore};

pub mod animals;
pub mod island;

fn main() {
    let mut rng = rand::thread_rng();

    let geography: Vec<&str> = vec![
        "WWWWW",
        "WLLLW",
        "WLLLW",
        "WLLLW",
        "WWWWW"
    ];

    let mut isl = island::Island::new(geography, &mut rng);

    isl.add_population(vec![
        ((2, 2), Herbivore, 10),
        ((2, 2), Carnivore, 2)
    ]);

    println!("Metrics: {:#?}", isl.animals());

    isl.yearly_cycle();
    isl.yearly_cycle();
    isl.yearly_cycle();
    isl.yearly_cycle();
    isl.yearly_cycle();

    let metrics = isl.animals();
    println!("Metrics: {:#?}", metrics);


    let mut keys: Vec<_> = metrics.2.keys().collect();
    keys.sort();
    let mut inhabit: Vec<_> = isl.inhabited.iter().collect();
    inhabit.sort();
    assert_eq!(keys, inhabit);
}
