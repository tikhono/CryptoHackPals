#[cfg(test)]
mod tests {
    use ascii::IntoAsciiString;
    use nalgebra::Matrix4;

    #[test]
    fn capture_the_flag() {
        let matrix = Matrix4::new(
            99, 114, 121, 112, 116, 111, 123, 105, 110, 109, 97, 116, 114, 105, 120, 125,
        );
        let vec = matrix.transpose().as_slice().to_vec();
        println!("{}", vec.into_ascii_string().unwrap());
    }
}
