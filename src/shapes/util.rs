fn min_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    if a <= b && a <= c {
        return a;
    } else if b <= a && b <= c {
        return b;
    } else {
        return c;
    }
}

fn mid_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    if a > b {
        if b > c {
            return b;
        } else if a > c {
            return c;
        } else {
            return a;
        }
    } else {
        if a > c {
            return a;
        } else if b > c {
            return c;
        } else {
            return b;
        }
    }
}

fn max_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    if a >= b {
        if a >= c {
            return a;
        } else {
            return c;
        }
    }
    else {
        if b >= c {
            return b;
        } else {
            return c;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{f32, u32};
    use crate::*;
    
    const permutations: [[u32; 3]; 6] = [
        [1, 2, 3], 
        [1, 3, 2], 
        [2, 1, 3], 
        [2, 3, 1], 
        [3, 1, 2],
        [3, 2, 1]];
    
    #[test] 
    fn test_simple() {
        assert_eq!(min_of_three(4, 1, 9), 1);
        assert_eq!(min_of_three(-999, 0, 999), -999);
        assert_eq!(mid_of_three(4.3, -1.0, f32::consts::PI), f32::consts::PI);
        assert_eq!(mid_of_three(u32::MIN, 0, u32::MAX), 0);
        assert_eq!(max_of_three(f32::consts::E, 0.111111, 0.111111), f32::consts::E);
        assert_eq!(max_of_three(-333.333333333333333_f64, -222.222222222222222_f64, -111.111111111111111_f64), -111.111111111111111_f64);
    }
    
    #[test]
    fn min_of_three_with_permutations() {
        for comb in permutations {
            let [x, y, z] = comb;
            assert_eq!(min_of_three(x, y, z), 1);
        }
    }
    
    #[test]
    fn mid_of_three_with_permutations() {
        for comb in permutations {
            let [x, y, z] = comb;
            assert_eq!(mid_of_three(x, y, z), 2);
        }
    }
    
    #[test]
    fn max_of_three_with_permutations() {
        for comb in permutations {
            let [x, y, z] = comb;
            assert_eq!(max_of_three(x, y, z), 3);
        }
    }
}