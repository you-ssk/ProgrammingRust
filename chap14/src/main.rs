fn main() {
    #[derive(Debug)]
    struct City {
        name: String,
        population: i64,
        country: String,
    }

    enum Statistic {
        Population,
    }

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

    fn city_population_descending(city: &City) -> i64{
        -city.population
    }

    fn sort_cities(cities: &mut Vec<City>){
        cities.sort_by_key(city_population_descending);
    }
    sort_cities(&mut cities);
    println!("{:?}", cities);

    fn sort_cities2(cities: &mut Vec<City>){
        cities.sort_by_key(|city| city.population);
    }
    sort_cities2(&mut cities);
    println!("{:?}", cities);


}
