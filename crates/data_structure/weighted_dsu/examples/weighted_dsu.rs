// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
use ac_library::Additive;
use proconio::input;
use qitoy_derive::query_readable;
use qitoy_weighted_dsu::WeightedDsu;

query_readable! {
    Query {
        { x: usize, y: usize, z: i64 },
        { x: usize, y: usize }
    }
}

fn main() {
    input! {
        n: usize, q: [Query],
    }
    let mut wdsu = WeightedDsu::<Additive<i64>>::new(n);
    for q in q {
        match q {
            Query::Query0 { x, y, z } => {
                wdsu.merge(x, y, z);
            }
            Query::Query1 { x, y } => {
                if let Some(z) = wdsu.diff(x, y) {
                    println!("{z}");
                } else {
                    println!("?");
                }
            }
        }
    }
}
