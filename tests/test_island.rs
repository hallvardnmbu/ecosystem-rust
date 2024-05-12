#[path = "../src/animals.rs"] mod animals;
#[path = "../src/island.rs"] mod island;

#[cfg(test)]
mod tests {
    use crate::animals::Species::{Carnivore, Herbivore};
    use crate::island::*;
    use std::collections::HashMap;
    use rand::thread_rng;

    #[test]
    fn test_new_island() {
        let mut rng = thread_rng();
        let geography: Vec<&str> = vec![
            "WWW",
            "WLW",
            "WLW",
            "WWW"
        ];
        let isl = Island::new(geography, &mut rng);
        assert_eq!(isl.year, 0);
        assert_eq!(isl.inhabited.len(), 0);
    }

    #[test]
    fn test_add_population() {
        let mut rng = thread_rng();
        let geography: Vec<&str> = vec![
            "WWW",
            "WLW",
            "WLW",
            "WWW"
        ];
        let mut isl = Island::new(geography, &mut rng);
        isl.add_population(vec![
            ((1, 1), Herbivore, 10),
            ((1, 1), Carnivore, 2)
        ]);
        assert_eq!(isl.inhabited.len(), 1);
    }

    #[test]
    fn test_yearly_cycle() {
        let mut rng = thread_rng();
        let geography: Vec<&str> = vec![
            "WWW",
            "WLW",
            "WLW",
            "WWW"
        ];
        let mut isl = Island::new(geography, &mut rng);
        isl.add_population(vec![
            ((1, 1), Herbivore, 10),
            ((1, 1), Carnivore, 2)
        ]);
        isl.yearly_cycle();
        assert_eq!(isl.year, 1);
    }

    #[test]
    fn test_cell_grow_fodder() {
        let mut cell = Cell {
            f_max: 300,
            fodder: 200,
            animals: HashMap::from([
                (Herbivore, Vec::new()),
                (Carnivore, Vec::new())
            ])
        };
        cell.grow_fodder();
        assert!(cell.fodder > 200);
    }
}