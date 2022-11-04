
use std::cmp::max;
use std::cmp::min;

/**
Choosing a sample of r elements from a set of n elements:
Order matters?   Repeats?   Ways to chose a sample
No                  No          r-Combination C(n, r)
Yes                 No          r-Permutation P(n, r)
No                  Yes         r-Combo Replacement Cr(n, r)
Yes                 Yes         r-Perm Replacment Pr(n, r)

Occupancy Problems
Distict Balls   Distict Cells   Empty Cells     Num ways to place n balls in k cells
Yes             Yes             Yes             k ^ n
Yes             Yes             No              k! S(n, k)

No              Yes             Yes             C(k + n - 1, n)
No              Yes             No              C(n - 1, k - 1)

Yes              No             Yes             S(n, 1) + ...S(n,k)
Yes              No             No              S(n, k)

No              No             Yes              N partitions of n into k or fewer parts
No              No             No               N parititions of n into exactly k parts
 */

pub fn distinct_balls_to_distict_cells(n_balls: u64, k_cells: u64, allow_empty: bool) -> u64{
    if allow_empty{
        // balls: {a, b, c} to cells {1, 2}
        // c1 can have a, b, c or 0 (no balls)

        // ways to assign/choose r balls from k cells
        // each ball has to go somewhere and cells can have "repeats" meaning that the cell can hold
        // more than one ball
        return r_permutation_replacement(k_cells, n_balls);
    }
    else {
        let s_nk = sterling(n_balls, k_cells);
        return s_nk * factorial(k_cells);
    }
}


pub fn indistinct_balls_to_distict_cells(n_balls: u64, k_cells: u64, allow_empty: bool) -> u64{
    if allow_empty{
        //return r_combination(k_cells + n_balls - 1, n_balls);
        return r_combination_replacement(k_cells, n_balls);
    }
    else {
        return r_combination(n_balls - 1, k_cells - 1);
    }
}

pub fn distinct_balls_to_indistict_cells(n_balls: u64, k_cells: u64, allow_empty: bool) -> u64{
    if allow_empty{
        return (1..=k_cells).map(|k| sterling(n_balls, k)).sum();
    }
    else {
        return sterling(n_balls, k_cells);
    }
}


pub fn sterling(n : u64, k: u64 ) -> u64{
    let s_even: u64 = (0..=k).step_by(2).map(|i|  r_combination(k, i) * (k - i).pow(n as u32) ).sum();
    let s_odd: u64 = (1..=k).step_by(2).map(|i|  r_combination(k, i) * (k - i).pow(n as u32) ).sum();

    assert!(s_even >= s_odd);
    return (s_even - s_odd) / factorial(k);

}

pub fn factorial(n: u64) -> u64{
    if n <= 1 {
        return n;
    }

    let f = (1..=n).product();
    return f;
}

pub fn r_combination(n: u64, r: u64) -> u64{
    if n < r {
        return 0;
    }

    if r == 0  || n == r {
        return 1;
    }

    let numerator: u64 = (max(r + 1, n - r + 1)..=n ).product();
    let denominator = factorial(min(r, n - r));  //what if n == r? then this will be 0!

    return numerator / denominator;
}

pub fn r_permutation(n: u64, r: u64) -> u64{
    if n < r {
        return 0;
    }

    (max(r + 1, n - r + 1)..=n ).product()
}

pub fn r_permutation_replacement(n: u64, r: u64) -> u64{
    n.pow(r.try_into().unwrap())
}

pub fn r_combination_replacement(n: u64, r: u64) -> u64{
    r_combination(n + r - 1, r)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_rules() {
        assert!(factorial(5) == 120);
        assert!(r_combination(9,3) == 84);
        assert!(r_combination(9,6) == 84);
        assert!(r_combination(5,3) == 10);
        assert!(r_combination(7,5) == 21);
        assert!(r_permutation(10,4) == 5040);
        assert!(r_permutation_replacement(4,2) == 16);
        assert!(sterling(5, 1)==1);
        assert!(sterling(3, 2)==3);

    }


}