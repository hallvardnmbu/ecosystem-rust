#[path = "../src/animals.rs"] mod animals;
#[path = "../src/island.rs"] mod island;

fn main() {
    let mut rng = rand::thread_rng();

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

    let mut isl = island::Island::new(geography, &mut rng);

    isl.add_population(vec![
        ((4, 4), animals::Species::Herbivore, 100),
        ((4, 4), animals::Species::Carnivore, 10)
    ]);

    // println!("Metrics: {:?}", isl.animals());

    let start = std::time::Instant::now();
    for i in 0..1000 {
        isl.yearly_cycle();
        // if i % 100 == 0 {
        //     println!("{} iteration, n_animals: {:?} {:?}", i, isl.animals().0, isl.animals().1);
        // }
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    println!("n_animals: {:?} {:?}", isl.animals().0, isl.animals().1)

    // let metrics = isl.animals();
    // println!("Metrics: {:#?}", metrics);
}
