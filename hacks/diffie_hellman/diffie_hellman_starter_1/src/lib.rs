use num::Integer;

pub fn egcd<T: Clone + Integer>(a: &T, b: &T) -> (T, T, T) {
    match *a == T::zero() {
        true => (b.clone(), T::zero(), T::one()),
        false => {
            let (g, x, y) = egcd(&(b.clone() % a.clone()), a);
            (g, y - (b.clone() / a.clone()) * x.clone(), x)
        }
    }
}

pub fn mod_inverse<T: Clone + Integer>(a: T, m: T) -> Option<T> {
    let (g, x, _) = egcd(&a, &m);
    if g != T::one() {
        None
    } else {
        Some((x % m.clone() + m.clone()) % m)
    }
}

#[cfg(test)]
mod tests {
    use crate::{egcd, mod_inverse};

    #[test]
    fn capture_the_flag() {
        assert_eq!(mod_inverse(209, 991), Some(569));
    }

    #[test]
    fn test_egcd() {
        let a = 26;
        let b = 3;
        let (g, x, y) = egcd(&a, &b);

        assert_eq!(g, 1);
        assert_eq!(x, -1);
        assert_eq!(y, 9);
        assert_eq!((a * x) + (b * y), g);
    }
}
