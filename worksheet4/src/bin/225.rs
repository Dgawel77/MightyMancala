fn f1(r: &str) -> [&str; 3] {
    [&r[0..4], &r[4..8], &r[8..]]
}

// returns a temporary variable owned by this function
// not allowed
// fn f2(r: &str) -> &[&str] {
//     &[&r[0..4], &r[4..8], &r[8..]]
// }

fn f3(r: &str) -> Vec<&str> {
    vec![&r[0..4], &r[4..8], &r[8..]]
}

fn g1(r: &str) -> [String; 3] {
    [
        String::from(&r[0..4]),
        String::from(&r[4..8]),
        String::from(&r[8..]),
    ]
}

// returns a temporary variable owned by this function
// not allowed
// fn g2(r: &str) -> &[String] {
//     &[
//         String::from(&r[0..4]),
//         String::from(&r[4..8]),
//         String::from(&r[8..]),
//     ]
// }

fn g3(r: &str) -> Vec<String> {
    vec![
        String::from(&r[0..4]),
        String::from(&r[4..8]),
        String::from(&r[8..]),
    ]
}

fn main() {
    dbg!(f1("the rain in Spain"));
    //dbg!(f2("the rain in Spain"));
    dbg!(f3("the rain in Spain"));
    dbg!(g1("the rain in Spain"));
    //dbg!(g2("the rain in Spain"));
    dbg!(g3("the rain in Spain"));
}
