fn main() {
    ex_10_1();
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
                TimeUnit::Days=> "days",
                TimeUnit::Months => "months",
                TimeUnit::Years => "years",
            }
        }
        fn singular(self) -> &'static str {
            self.plural().trim_end_matches('s')
        }
    }

    let t = TimeUnit::Seconds;
    println!("{:?}", t.singular());

    let m = TimeUnit::Months;
    println!("{:?}", m.singular());
}
