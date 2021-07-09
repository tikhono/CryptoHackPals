use point_addition::Point;

pub trait Getter {
    fn x(&self) -> Option<i64>;
    fn y(&self) -> Option<i64>;
}

impl Getter for point_addition::Point {
    fn x(&self) -> Option<i64> {
        match self {
            Point::Zero => None,
            Point::Coord { x, .. } => Some(*x),
        }
    }
    fn y(&self) -> Option<i64> {
        match self {
            Point::Zero => None,
            Point::Coord { y, .. } => Some(*y),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Getter;
    use point_addition::Point;
    use scalar_multiplication::multiplication;
    use sha1::{Digest, Sha1};

    #[test]
    fn capture_the_flag() {
        let qa = Point::Coord { x: 815, y: 3190 };
        let nb = 1829;
        let shared_secret = multiplication(&qa, nb).x().unwrap().to_string();

        let mut hasher = Sha1::new();
        hasher.update(shared_secret);
        let flag = hasher.finalize().to_vec();
        println!("crypto{{{}}}", hex::encode(flag));
    }
}
