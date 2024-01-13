#[derive(Debug, Clone)]
struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f32,
}

#[derive(Clone, Copy)]
enum Statistic {
    Population,
}

impl City {
    fn get_statistics(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
        }
    }
}

fn main() {
    let mut cities = vec![
        City {
            name: "Tokyo".to_string(),
            population: 10000,
            country: "japan".to_string(),
            monster_attack_risk: -1.0,
        },
        City {
            name: "New York".to_string(),
            population: 20000,
            country: "USA".to_string(),
            monster_attack_risk: 1.0,
        },
        City {
            name: "London".to_string(),
            population: 15000,
            country: " United Kingdom".to_string(),
            monster_attack_risk: 2.0,
        },
    ];
    println!("{:?}", cities);

    fn city_population_descending(city: &City) -> i64 {
        -city.population
    }

    fn sort_cities(cities: &mut Vec<City>) {
        cities.sort_by_key(city_population_descending);
    }
    sort_cities(&mut cities);
    println!("{:?}", cities);

    fn sort_cities2(cities: &mut Vec<City>) {
        cities.sort_by_key(|city| city.population);
    }
    sort_cities2(&mut cities);
    println!("{:?}", cities);

    ex_14_1();

    fn city_monster_attack_rist_descending(city: &City) -> i64 {
        city.population
    }
    let mut user_prefs_by_population = false;
    let my_key_fn: fn(&City) -> i64 = if user_prefs_by_population {
        city_population_descending
    } else {
        city_monster_attack_rist_descending
    };
    cities.sort_by_key(my_key_fn);
    println!("{:?}", cities);

    user_prefs_by_population = true;
    cities.sort_by_key(my_key_fn);
    println!("{:?}", cities);

    // fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    //     let mut count = 0;
    //     for city in cities {
    //         if test_fn(city) {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
    where
        F: Fn(&City) -> bool,
    {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }

    fn has_monster_attacks(city: &City) -> bool {
        city.monster_attack_risk > 0.0
    }

    let n = count_selected_cities(&cities, has_monster_attacks);
    println!("monster risk city : {}", n);

    let mut limit = 0.5;
    let n = count_selected_cities(&cities, |city| city.monster_attack_risk > limit);
    println!("monster risk city : limit = {}, {}", limit, n);

    limit = 1.5;
    let n = count_selected_cities(&cities, |city| city.monster_attack_risk > limit);
    println!("monster risk city : limit = {}, {}", limit, n);
}

fn ex_14_1() {
    println!("ex_14_1()");
    let mut cities = vec![
        City {
            name: "Tokyo".to_string(),
            population: 10000,
            country: "japan".to_string(),
            monster_attack_risk: -1.0,
        },
        City {
            name: "New York".to_string(),
            population: 20000,
            country: "USA".to_string(),
            monster_attack_risk: 1.0,
        },
        City {
            name: "London".to_string(),
            population: 15000,
            country: " United Kingdom".to_string(),
            monster_attack_risk: 2.0,
        },
    ];
    fn sort_by_statistics(cities: &mut Vec<City>, stat: Statistic) {
        cities.sort_by_key(|city| -city.get_statistics(stat));
    }
    //println!("{:?}", cities);
    // sort_by_statistics(&mut cities, Statistic::Population);
    // println!("{:?}", cities);

    use std::thread;
    fn start_sorting_thread(
        mut cities: Vec<City>,
        stat: Statistic,
    ) -> thread::JoinHandle<Vec<City>> {
        let key_fn = move |city: &City| -> i64 { -city.get_statistics(stat) };
        thread::spawn(move || {
            cities.sort_by_key(key_fn);
            cities
        })
    }
    println!("{:?}", cities);

    let sorted = start_sorting_thread(cities, Statistic::Population).join();

    println!("{:?}", sorted.ok().unwrap());
}
