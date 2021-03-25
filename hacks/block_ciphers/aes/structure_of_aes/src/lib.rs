use nalgebra::Matrix4;

pub fn matrix2bytes(matrix: Matrix4<u8>) -> Vec<u8> {
    matrix.transpose().as_slice().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::matrix2bytes;
    use ascii::IntoAsciiString;
    use nalgebra::Matrix4;

    #[test]
    fn capture_the_flag() {
        let matrix = Matrix4::new(
            99, 114, 121, 112, 116, 111, 123, 105, 110, 109, 97, 116, 114, 105, 120, 125,
        );
        let vec = matrix2bytes(matrix);
        println!("{}", vec.into_ascii_string().unwrap());
    }
}
