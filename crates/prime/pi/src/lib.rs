pub fn pi(x: usize) -> usize {
    let x_13 = (x as f64).cbrt() as usize;
    let x_12 = (x as f64).sqrt() as usize;
    let x_23 = x / x_13;
    let mut f: Vec<_> = (0..=x_13).collect();
    let mut mu = vec![1; x_13 + 1];
    let mut primes = vec![0];

    // initial sieve
    for p in 2..=x_13 {
        if f[p] == p {
            primes.push(p);
            for i in 1..=x_13 / p {
                f[p * i] = f[p * i].min(p);
                mu[p * i] *= if i % p == 0 { 0 } else { -1 };
            }
        }
    }

    let a = primes.len() - 1;
    let mut ret = a as isize - 1;

    // ordinary leaf
    for (n, mu) in mu.iter().enumerate().skip(1) {
        ret += mu * (x / n) as isize;
    }

    let mut is_prime = vec![true; x_23 + 1];
    let mut phi = ac_library::FenwickTree::new(x_23 + 1, 0);
    for i in 1..=x_23 {
        phi.add(i, 1);
    }

    // special leaf
    for b in 0..=a {
        // sieve
        if b != 0 {
            let pb = primes[b];
            phi.add(pb, -1);
            for i in pb..=x_23 / pb {
                if is_prime[pb * i] {
                    phi.add(pb * i, -1);
                }
                is_prime[pb * i] = false;
            }
        }

        // sum
        if b + 1 < a {
            for m in 1..=x_13 {
                let pb1 = primes[b + 1];
                let n = m * pb1;
                if x_13 < n && f[m] > pb1 {
                    ret += -mu[m] * phi.sum(0..=x / n);
                }
            }
        }
    }

    for p in (x_13 + 1..=x_12).filter(|&p| is_prime[p]) {
        primes.push(p);
    }
    let pi_sqrt = primes.len() - 1;
    let mut pi_x = pi_sqrt as isize;
    let mut p = x_12 + 1;

    // P_2
    for j in (a + 1..=pi_sqrt).rev() {
        while p <= x / primes[j] {
            if is_prime[p] {
                pi_x += 1;
            }
            p += 1;
        }
        ret -= pi_x;
    }
    let a = a as isize;
    let pi_sqrt = pi_sqrt as isize;
    ret += -a * (a - 1) / 2 + pi_sqrt * (pi_sqrt - 1) / 2;

    ret as usize
}
