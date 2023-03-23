/// エラトステネスの篩  
/// 計算量$`O(n \log n)`$
pub fn eratosthenes(n: usize) -> Vec<bool> {
    let mut vec = vec![true; n+1];
    vec[0] = false; vec[1] = false;
    for i in (2..).take_while(|&i| i*i <= n) {
        for j in (i..=n/i).map(|v| v*i) {
            vec[j] = false;
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(eratosthenes(10),
                   [false, false, true, true, false, true, false, true, false, false, false]);
    }
}
