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

    ex_14_4();

    ex_14_4_3();

    ex_14_4_4();

    ex_14_5();
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

fn ex_14_4() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);
    f();
    // note: closure cannot be invoked more than once because it moves the variable `my_str` out of its environment
    // f();

    ex_14_4_2();
}

fn ex_14_4_2() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);

    fn call_twice<F>(closure: F)
    where
        F: Fn(),
    {
        closure();
        closure();
    }
    // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
    //call_twice(f);
}

fn ex_14_4_3() {
    fn call_twice<F>(mut closure: F)
    where
        F: FnMut(),
    {
        closure();
        closure();
    }
    let mut i = 0;
    call_twice(|| i += 1);
    assert_eq!(i, 2);
    println!("after call_twice {}", i)
}

fn ex_14_4_4() {
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;
    assert_eq!(add_y(copy_of_add_y(22)), 42);

    let mut x = 0;
    let mut add_to_x = |n: i32| {
        x += n;
        x
    };
    let copy_of_add_to_x = add_to_x;

    //assert_eq!(add_to_x(copy_of_add_to_x(1)), 2); // cannot borrow as mutable

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}

fn ex_14_5() {
    use std::collections::HashMap;

    struct Request {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    #[derive(Debug)]
    struct Response {
        code: u32,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    // struct BasicRouter<C>
    // where
    //     C: Fn(&Request) -> Response,
    // {
    //     routes: HashMap<String, C>,
    // }
    //
    // impl<C> BasicRouter<C>
    // where
    //     C: Fn(&Request) -> Response,
    // {
    //     // Create an empty router.
    //     fn new() -> BasicRouter<C> {
    //         BasicRouter {
    //             routes: HashMap::new(),
    //         }
    //     }

    //     fn add_route(&mut self, url: &str, callback: C) {
    //         self.routes.insert(url.to_string(), callback);
    //     }
    // }

    type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

    struct BasicRouter {
        routes: HashMap<String, BoxedCallback>,
    }

    impl BasicRouter {
        fn new() -> BasicRouter {
            BasicRouter {
                routes: HashMap::new(),
            }
        }

        fn add_route<C>(&mut self, url: &str, callback: C)
        where
            C: Fn(&Request) -> Response + 'static,
        {
            self.routes.insert(url.to_string(), Box::new(callback));
        }

        fn handle_request(&self, request: &Request) -> Response {
            match self.routes.get(&request.url) {
                None => not_found_response(),
                Some(callback) => callback(request),
            }
        }
    }
    fn not_found_response() -> Response {
        Response {
            code: 404,
            headers: HashMap::new(),
            body: b"<h1>Page not found</h1>".to_vec(),
        }
    }

    fn get_from_response() -> Response {
        Response {
            code: 200,
            headers: HashMap::new(),
            body: b"<form>".to_vec(),
        }
    }

    fn get_gcd_response(request: &Request) -> Response {
        Response {
            code: 500,
            headers: HashMap::new(),
            body: b"<h1>Internal Server Error</h1>".to_vec(),
        }
    }

    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_from_response());
    router.add_route("/gcd", |req| get_gcd_response(req));

    {
        let req = Request {
            method: "GET".to_string(),
            url: "/piano".to_string(),
            headers: HashMap::new(),
            body: vec![],
        };
        let res = router.handle_request(&req);
        println!("{}", String::from_utf8(res.body).unwrap());
    }
    {
        let req = Request {
            method: "GET".to_string(),
            url: "/".to_string(),
            headers: HashMap::new(),
            body: vec![],
        };
        let res = router.handle_request(&req);
        println!("{}", String::from_utf8(res.body).unwrap());
    }
    {
        let req = Request {
            method: "GET".to_string(),
            url: "/gcd".to_string(),
            headers: HashMap::new(),
            body: vec![],
        };
        let res = router.handle_request(&req);
        println!("{}", String::from_utf8(res.body).unwrap());
    }

    struct FnPointerRouter {
        routes: HashMap<String, fn(&Request) -> Response>,
    }

    impl FnPointerRouter {
        fn new() -> FnPointerRouter {
            FnPointerRouter {
                routes: HashMap::new(),
            }
        }
        fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
            self.routes.insert(url.to_string(), callback);
        }
    }
}
