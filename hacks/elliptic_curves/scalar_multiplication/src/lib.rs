use point_addition::{addition, Point};

pub fn multiplication(p: &Point, mut n: u64) -> Point {
    let mut q = *p;
    let mut res = Point::Zero;
    while n > 0 {
        if n % 2 == 1 {
            res = addition(&res, &q);
        }
        q = addition(&q, &q);
        n /= 2;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::multiplication;
    use point_addition::Point;

    #[test]
    fn capture_the_flag() {
        println!(
            "{:?}",
            multiplication(&Point::Coord { x: 2339, y: 2213 }, 7863)
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            Point::Coord { x: 1089, y: 6931 },
            multiplication(&Point::Coord { x: 5323, y: 5438 }, 1337)
        );
    }
}
