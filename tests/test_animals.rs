#[path = "../src/animals.rs"] mod animals;

#[cfg(test)]
mod tests {
    use crate::animals::*;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_birthweight() {
        let mut rng = ThreadRng::default();
        let animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        let birthweight = animal.birthweight(&mut rng);
        assert!(birthweight > 0.0);
    }

    #[test]
    fn test_gain_weight() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        animal.gain_weight(5);
        assert_eq!(animal.weight, 15.0);
    }

    #[test]
    fn test_aging() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        animal.aging();
        assert_eq!(animal.age, 6);
    }

    #[test]
    fn test_lose_weight_year() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        animal.lose_weight_year();
        assert_eq!(animal.weight, 10.0 - Parameters::HERBIVORE.eta * 10.0);
    }

    #[test]
    fn test_lose_weight_birth() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        let result = animal.lose_weight_birth(5.0);
        assert_eq!(result, true);
        assert_eq!(animal.weight, 10.0 - Parameters::HERBIVORE.xi * 5.0);
    }

    #[test]
    fn test_calculate_fitness() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        animal.calculate_fitness();
        assert!(animal.fitness > 0.0);
    }

    #[test]
    fn test_graze() {
        let mut animal = Animal {
            species: Species::Herbivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        let eaten = animal.graze(30);
        assert_eq!(eaten, Parameters::HERBIVORE.hunger);
        assert_eq!(animal.weight, 10.0 + Parameters::HERBIVORE.hunger as f32);
    }

    #[test]
    fn test_predation() {
        let mut rng = ThreadRng::default();
        let mut animal = Animal {
            species: Species::Carnivore,
            weight: 10.0,
            age: 5,
            fitness: 0.5,
        };
        let mut herbivores = vec![
            Animal {
                species: Species::Herbivore,
                weight: 5.0,
                age: 3,
                fitness: 0.3,
            },
            Animal {
                species: Species::Herbivore,
                weight: 7.0,
                age: 4,
                fitness: 0.4,
            },
        ];
        let eaten = animal.predation(&mut rng, &mut herbivores);
        assert!(eaten <= Parameters::CARNIVORE.hunger);
        assert!(animal.weight >= 10.0);
    }
}