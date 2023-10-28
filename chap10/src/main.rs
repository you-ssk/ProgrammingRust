use serde_json::value;

fn main() {
    ex_10_1();
    ex_10_1_1();
    ex_10_1_3();
    ex_10_1_4();
    ex_10_2();
    ex_10_2_2();
    ex_10_2_3();
    ex_10_2_6();
    ex_10_2_8();
    ex_10_2_9();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn ex_10_1() {
    // enum Ordering {
    //     Less,
    //     Equal,
    //     Greater,
    // }

    use std::cmp::Ordering;

    fn compare(n: i32, m: i32) -> Ordering {
        if n < m {
            Ordering::Less
        } else if n > m {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    enum HttpStatus {
        Ok = 200,
        NotModified = 304,
        NotFound = 404,
    }
    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
    assert_eq!(HttpStatus::Ok as i32, 200);
    assert_eq!(HttpStatus::NotModified as i32, 304);
    assert_eq!(HttpStatus::NotFound as i32, 404);

    let t = TimeUnit::Seconds;
    println!("{:?}", t.singular());

    let m = TimeUnit::Months;
    println!("{:?}", m.singular());
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn ex_10_1_1() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

    println!(
        "{:?}, {:?}",
        four_score_and_seven_years_ago, three_hours_from_now
    );

    #[derive(Debug)]
    struct Point3d(f32, f32, f32);
    impl Point3d {
        const ORIGIN: Point3d = Point3d(0.0, 0.0, 0.0);
    }

    #[derive(Debug)]
    enum Shape {
        Sphere { center: Point3d, radius: f32 },
        Cuboid { corner1: Point3d, corner2: Point3d },
    }
    let unit_sphere = Shape::Sphere {
        center: Point3d::ORIGIN,
        radius: 1.0,
    };
    println!("{:?}", unit_sphere);
}

fn ex_10_1_3() {
    use std::collections::HashMap;
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String, Json>>),
    }
}
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn ex_10_1_4() {
    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    println!("{:?}", mars_tree);
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(TimeUnit::Hours, 1) => {
            format!("an {} ago", TimeUnit::Hours.plural())
        }
        RoughTime::InThePast(units, 1) => format!("a {} ago", units.plural()),
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("Just Now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) => {
            format!("an {} from now", TimeUnit::Hours.plural())
        }
        RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.plural()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn ex_10_2() {
    let rtp = RoughTime::InThePast(TimeUnit::Hours, 4);
    println!("{}", rough_time_to_english(rtp));

    let rtp = RoughTime::InThePast(TimeUnit::Months, 1);
    println!("{}", rough_time_to_english(rtp));

    let rtp = RoughTime::InThePast(TimeUnit::Hours, 1);
    println!("{}", rough_time_to_english(rtp));

    let rtj = RoughTime::JustNow;
    println!("{}", rough_time_to_english(rtj));

    let rtf = RoughTime::InTheFuture(TimeUnit::Months, 3);
    println!("{}", rough_time_to_english(rtf));

    let rtf = RoughTime::InTheFuture(TimeUnit::Hours, 1);
    println!("{}", rough_time_to_english(rtf));

    let rtf = RoughTime::InTheFuture(TimeUnit::Years, 1);
    println!("{}", rough_time_to_english(rtf));
}

fn ex_10_2_2() {
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the origin",
            (_, Equal) => "on the x axis",
            (Equal, _) => "on the y axis",
            (Greater, Greater) => "in the first quadrant",
            (Less, Greater) => "in the second quadrant",
            _ => "somewhere else",
        }
    }
    println!("{}", describe_point(0, 0));
    println!("{}", describe_point(-10, -100));
}

fn ex_10_2_3() {
    // 配列パターン
    fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
        match hsl {
            [_, _, 0] => [0, 0, 0],
            [_, _, 255] => [255, 255, 255],
            [_, _, _] => hsl, // fake
        }
    }

    // スライスパターン
    fn greet_people(names: &[&str]) {
        match names {
            [] => {
                println!("Hello, nobody.")
            }
            [a] => {
                println!("Hello, {}.", a)
            }
            [a, b] => {
                println!("Hello, {} and {}.", a, b)
            }
            [a, .., b] => {
                println!("Hello, everyone from {} to {}", a, b)
            }
        }
    }
    let names = ["A", "B", "C"];
    greet_people(&names);
    greet_people(&[]);
    greet_people(&["Yo"]);
    greet_people(&["Sasaki", "Yo"]);
}

fn ex_10_2_6() {
    //let mut chars = ['\n', '\r', 'c'].iter().peekable();
    let mut chars = "\nabcdefg".chars().peekable();
    let at_end = match chars.peek() {
        Some(&'\r' | &'\n') | None => true,
        _ => false,
    };
    println!("{}", at_end);
}

fn ex_10_2_8() {
    struct Track {
        album: String,
        track_number: i32,
        title: String,
    };

    struct Song {
        album: String,
        track_number: i32,
        title: String,
        writer: String,
        composer: String,
    };

    let song = Song {
        album: "an album".to_string(),
        track_number: 9,
        title: "a song".to_string(),
        writer: "famous writer.".to_string(),
        composer: "great composer".to_string(),
    };
    let Song {
        album,
        track_number,
        writer,
        ..
    } = song;
    println!("{} {} {}", album, track_number, writer);

    #[derive(Debug)]
    struct Point3d {
        x: f32,
        y: f32,
        z: f32,
    };
    let p1 = Point3d {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    println!("{:?}", p1);
    let Point3d { x, y, z } = p1;
    println!("Point3d({}, {}, {})", x, y, z);
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

fn ex_10_2_9() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    tree.add("Jupiter");
    tree.add("Earth");
    tree.add("Mars");
    println!("{:?}", tree);
}
