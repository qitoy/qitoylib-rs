use proconio::input;
use qitoy_derive::query_readable;
use qitoy_li_chao_tree::LiChaoTree;

query_readable! {
    Query {
        { a: i64, b: i64, },
        { p: i64, },
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        line: [(i64, i64); n],
        query: [Query; q],
    }

    let mut lct = LiChaoTree::new(-1 << 30..1 << 30);
    for (a, b) in line {
        lct.add_line(a, b);
    }
    for query in query {
        match query {
            Query::Query0 { a, b } => lct.add_line(a, b),
            Query::Query1 { p } => println!("{}", lct.get_min(p)),
        }
    }
}
