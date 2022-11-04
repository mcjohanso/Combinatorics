
pub fn gcf(mut a: i32, mut b: i32) -> i32 {
    if a == 0 {
        return b;
    }

    let mut rem = a % b;

    while rem != 0 {
        //print!("gcd({}, {}) = ...", a, b);
        rem = a % b;
        a = b;
        b = rem;
    }


    return a;
}


pub fn pulverize(x: i32, y: i32) -> GcfFactors {
    let mut a = x;
    let mut b = y;

    //assert x > y here
    let mut x0: i32 = 1;
    let mut y0: i32 = 0;
    let mut x1: i32 = 0;
    let mut y1: i32 = 1;
    let mut s: i32 = 0;
    let mut t: i32 = 0;

    let mut rem = x % y;

    while rem > 0 {
        s = x0 - (a / b) * x1;
        t = y0 - (a / b) * y1;

        x0 = x1;
        y0 = y1;
        x1 = s;
        y1 = t;
        a = b;
        b = s * x + t * y;

        rem = a % rem;
    }

    return GcfFactors { s, t };
}

pub fn inverse_modn(k: i32, n: i32) -> i32 {
    if gcf(k, n) != 1 {
        return 0
    }

    let st: GcfFactors = pulverize(k, n);
    return st.s;
}

pub struct GcfFactors {
    pub s: i32,
    pub t: i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcf() {
        assert!(gcf(29, 17) == 1);
        assert!(gcf(29, 24) == 1);
        assert!(gcf(24, 72) == 24);
        assert!(gcf(24, 30) == 6);
        assert!(gcf(15, 95) == 5);

        for a in 2..99 {
            for b in 2..99 {
                assert!(gcf(a, b) > 0);
            }
        }
    }

    #[test]
    fn test_pulversize() {

        let a = 29;
        let b = 24;
        let st: GcfFactors = pulverize(a, b);
        assert!(st.s * a + st.t * b == 1);

        let st: GcfFactors = pulverize(b, a);
        assert!(st.s * b + st.t * a == 1);
    }

    #[test]
    fn test_inverse() {

        let primes = [13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
            61, 67, 71, 73, 79, 83, 89, 97];

        for a in 2..99 {
            for n in  primes{
                if gcf(a, n) != 1 {
                    continue;
                }

                let a_inverse = inverse_modn(a, n);
                assert!((a_inverse * a) % n == 1 || (a_inverse * a) % n == ( 1 - n));
            }
        }
    }

}