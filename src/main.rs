pub mod animals;
pub mod island;
mod graphics;
mod benchmark;
mod simulation;

fn main() {
    let mut rng = rand::thread_rng();

    // let geography: Vec<&str> = vec![
    //     "WWWWW",
    //     "WLLLW",
    //     "WLLLW",
    //     "WLLLW",
    //     "WWWWW"
    // ];
    let geography: Vec<&str> = vec![
             "WWWWWWWWWWWWW",
             "WWWLHHWWWHHWW",
             "WWLLLHWLWHLLW",
             "WWLLLLLLLMLMW",
             "WWHHLLLHLHMMW",
             "WHHLLLHWHHLMW",
             "WWWHHWWWWWMWW",
             "WWWWWWWWWWWWW",
    ];

    let mut sim = simulation::Simulation::new(
        geography, &mut rng, "graph_new.png"
    );
    sim.add_population(vec![
        ((2, 2), animals::Species::Herbivore, 100),
        ((2, 2), animals::Species::Carnivore, 10)
    ]);

    sim.simulate(1000, true);
}
