use nalgebra::Matrix4;

pub fn add_round_key(state: &mut Matrix4<u8>, round_key: &Matrix4<u8>) {
    state.zip_apply(round_key, |x, y| x ^ y);
}

#[cfg(test)]
mod tests {
    use crate::add_round_key;
    use ascii::IntoAsciiString;
    use nalgebra::Matrix4;
    use structure_of_aes::matrix2bytes;

    #[test]
    fn capture_the_flag() {
        let mut state = Matrix4::new(
            206, 243, 61, 34, 171, 11, 93, 31, 16, 200, 91, 108, 150, 3, 194, 51,
        );
        let round_key = Matrix4::new(
            173, 129, 68, 82, 223, 100, 38, 109, 32, 189, 53, 8, 253, 48, 187, 78,
        );
        add_round_key(&mut state, &round_key);
        println!("{}", matrix2bytes(state).into_ascii_string().unwrap());
    }
}
