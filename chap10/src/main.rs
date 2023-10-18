fn main() {
    ex_10_1();
    ex_10_1_1();
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
