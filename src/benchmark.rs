use super::animals;
use super::island;

fn main() {
    let mut times = Vec::new();
    for _ in 0..15 {
        let mut rng = rand::thread_rng();
        let geography: Vec<&str> = vec![
            "WWWWWWWWWWWWWWWWWWWWW",
            "WHHHHHLLLLWWLLLLLLLWW",
            "WHHHHHLLLLWWLLLLLLLWW",
            "WHHHHHLLLLWWLLLLLLLWW",
            "WWHHLLLLLLLWWLLLLLLLW",
            "WWHHLLLLLLLWWLLLLLLLW",
            "WWWWWWWWHWWWWLLLLLLLW",
            "WHHHHHLLLLWWLLLLLLLWW",
            "WHHHHHHHHHWWLLLLLLWWW",
            "WHHHHHMMMMMLLLLLLLWWW",
            "WHHHHHMMMMMLLLLLLLWWW",
            "WHHHHHMMMMMLLLLLLLWWW",
            "WHHHHHMMMMMWWLLLLLWWW",
            "WHHHHMMMMMMLLLLWWWWWW",
            "WWHHHHMMMMMMLWWWWWWWW",
            "WWHHHHMMMMMLLLWWWWWWW",
            "WHHHHHMMMMMLLLLLLLWWW",
            "WHHHHMMMMMMLLLLWWWWWW",
            "WWHHHHMMMMMLLLWWWWWWW",
            "WWWHHHHLLLLLLLWWWWWWW",
            "WWWHHHHHHWWWWWWWWWWWW",
            "WWWWWWWWWWWWWWWWWWWWW",
        ];

        let mut isl = island::Island::new(geography, &mut rng);

        isl.add_population(vec![
            ((4, 4), animals::Species::Herbivore, 100),
            ((4, 4), animals::Species::Carnivore, 10)
        ]);

        let start = std::time::Instant::now();
        for _ in 0..5000 {
            isl.yearly_cycle();
        }
        let duration = start.elapsed();
        times.push(duration);
    }
    let mean = times.iter().sum::<std::time::Duration>() / times.len() as u32;
    println!("Optimized mean time {:?}", mean);
}
