#[derive(Debug, Clone)]
struct City {
    name: String,
    population: i64,
    country: String,
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
        },
        City {
            name: "New York".to_string(),
            population: 20000,
            country: "USA".to_string(),
        },
        City {
            name: "London".to_string(),
            population: 15000,
            country: " United Kingdom".to_string(),
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
}

fn ex_14_1() {
    println!("ex_14_1()");
    let mut cities = vec![
        City {
            name: "Tokyo".to_string(),
            population: 10000,
            country: "japan".to_string(),
        },
        City {
            name: "New York".to_string(),
            population: 20000,
            country: "USA".to_string(),
        },
        City {
            name: "London".to_string(),
            population: 15000,
            country: " United Kingdom".to_string(),
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
