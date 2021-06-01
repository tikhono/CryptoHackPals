use modinverse::modinverse;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Point {
    Zero,
    Coord { x: i64, y: i64 },
}

pub fn addition(p: &Point, q: &Point) -> Point {
    match (p, q) {
        (Point::Zero, Point::Zero) => Point::Zero,
        (Point::Zero, Point::Coord { .. }) => *q,
        (Point::Coord { .. }, Point::Zero) => *p,
        (Point::Coord { x: x1, y: y1 }, Point::Coord { x: x2, y: y2 }) => {
            if x1 == x2 && *y1 == -y2 {
                return Point::Zero;
            }
            let lambda = if x1 == x2 && y1 == y2 {
                (3 * x1.pow(2) + 497) * modinverse(2 * y1, 9739).unwrap() % 9739
            } else {
                (y2 - y1) * modinverse(((x2 - x1) % 9739 + 9739) % 9739, 9739).unwrap() % 9739
            };
            let x = ((lambda.pow(2) - x1 - x2) % 9739 + 9739) % 9739;
            let y = ((lambda * (x1 - x) - y1) % 9739 + 9739) % 9739;
            return Point::Coord { x, y };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::addition;
    use crate::Point;

    #[test]
    fn capture_the_flag() {
        println!(
            "{:?}",
            addition(
                &addition(
                    &Point::Coord { x: 493, y: 5564 },
                    &Point::Coord { x: 1539, y: 4742 },
                ),
                &addition(
                    &Point::Coord { x: 493, y: 5564 },
                    &Point::Coord { x: 4403, y: 5202 }
                )
            )
        );
    }

    #[test]
    fn test_addition() {
        assert_eq!(Point::Zero, addition(&Point::Zero, &Point::Zero));
        assert_eq!(
            Point::Coord { x: 1024, y: 4440 },
            addition(
                &Point::Coord { x: 5274, y: 2841 },
                &Point::Coord { x: 8669, y: 740 }
            )
        );
        assert_eq!(
            Point::Coord { x: 7284, y: 2107 },
            addition(
                &Point::Coord { x: 5274, y: 2841 },
                &Point::Coord { x: 5274, y: 2841 },
            )
        );
    }
}
