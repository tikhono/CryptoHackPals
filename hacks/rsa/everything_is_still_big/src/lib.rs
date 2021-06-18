#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::Num;

    #[test]
    fn capture_the_flag_db() {
        // I used http://factordb.com, or it can be used Boneh attack

        let n = BigInt::from_str_radix(
            "665166804cd78e8197073f65f58bca14e019982245fcc7cad74535e948a4e0258b2e919bf3720968a00e5240c5e1d6b8831d8fec300d969fccec6cce11dde826d3fbe0837194f2dc64194c78379440671563c6c75267f0286d779e6d91d3e9037c642a860a894d8c45b7ed564d341501cedf260d3019234f2964ccc6c56b6de8a4f66667e9672a03f6c29d95100cdf5cb363d66f2131823a953621680300ab3a2eb51c12999b6d4249dde499055584925399f3a8c7a4a5a21f095878e80bbc772f785d2cbf70a87c6b854eb566e1e1beb7d4ac6eb46023b3dc7fdf34529a40f5fc5797f9c15c54ed4cb018c072168e9c30ca3602e00ea4047d2e5686c6eb37b9",
            16,
        )
            .unwrap();

        let q = BigInt::from_str_radix(
            "131205304707717699800023219057082007986286045823683571663112014612188606710079038751853416273709729039622908861933527111469616900188875912430487264576215232569029320804579614330240773622645122871884209068761138439268551367198798009790636662892148063583135747945604771740458352899202428704645256790931460695949",
            10,
        ).unwrap();
        let p = BigInt::from_str_radix(
            "98444549679044409506244239144443867459824227934526036052949278261505813439015297459200379108752444235232667213138464076415095486907288282630595622287237215801470940146886371515679909322090871473412384894540642399950010296214525469622505798526072170187467562765920044646574445427364231529083610955760228212701",
            10,
        )
            .unwrap();
        let phi: BigInt = (p - 1) * (q - 1);
        let e = BigInt::from_str_radix("2c998e57bc651fe4807443dbb3e794711ca22b473d7792a64b7a326538dc528a17c79c72e425bf29937e47b2d6f6330ee5c13bfd8564b50e49132d47befd0ee2e85f4bfe2c9452d62ef838d487c099b3d7c80f14e362b3d97ca4774f1e4e851d38a4a834b077ded3d40cd20ddc45d57581beaa7b4d299da9dec8a1f361c808637238fa368e07c7d08f5654c7b2f8a90d47857e9b9c0a81a46769f6307d5a4442707afb017959d9a681fa1dc8d97565e55f02df34b04a3d0a0bf98b7798d7084db4b3f6696fa139f83ada3dc70d0b4c57bf49f530dec938096071f9c4498fdef9641dfbfe516c985b27d1748cc6ce1a4beb1381fb165a3d14f61032e0f76f095d", 16).unwrap();
        let ct = BigInt::from_str_radix(
            "503d5dd3bf3d76918b868c0789c81b4a384184ddadef81142eabdcb78656632e54c9cb22ac2c41178607aa41adebdf89cd24ec1876365994f54f2b8fc492636b59382eb5094c46b5818cf8d9b42aed7e8051d7ca1537202d20ef945876e94f502e048ad71c7ad89200341f8071dc73c2cc1c7688494cad0110fca4854ee6a1ba999005a650062a5d55063693e8b018b08c4591946a3fc961dae2ba0c046f0848fbe5206d56767aae8812d55ee9decc1587cf5905887846cd3ecc6fc069e40d36b29ee48229c0c79eceab9a95b11d15421b8585a2576a63b9f09c56a4ca1729680410da237ac5b05850604e2af1f4ede9cf3928cbb3193a159e64482928b585ac",
            16,
        )
            .unwrap();

        let d = mod_inverse(e, phi).unwrap();
        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }

    #[test]
    #[ignore]
    fn capture_the_flag_reduction() {
        use lll_rs::l2::bigl2;
        use lll_rs::{matrix::Matrix, vector::BigVector};

        use rug::Integer;

        let n = Integer::from_str_radix(
            "665166804cd78e8197073f65f58bca14e019982245fcc7cad74535e948a4e0258b2e919bf3720968a00e5240c5e1d6b8831d8fec300d969fccec6cce11dde826d3fbe0837194f2dc64194c78379440671563c6c75267f0286d779e6d91d3e9037c642a860a894d8c45b7ed564d341501cedf260d3019234f2964ccc6c56b6de8a4f66667e9672a03f6c29d95100cdf5cb363d66f2131823a953621680300ab3a2eb51c12999b6d4249dde499055584925399f3a8c7a4a5a21f095878e80bbc772f785d2cbf70a87c6b854eb566e1e1beb7d4ac6eb46023b3dc7fdf34529a40f5fc5797f9c15c54ed4cb018c072168e9c30ca3602e00ea4047d2e5686c6eb37b9",
            16,
        ).unwrap();

        let e = Integer::from_str_radix(
            "2c998e57bc651fe4807443dbb3e794711ca22b473d7792a64b7a326538dc528a17c79c72e425bf29937e47b2d6f6330ee5c13bfd8564b50e49132d47befd0ee2e85f4bfe2c9452d62ef838d487c099b3d7c80f14e362b3d97ca4774f1e4e851d38a4a834b077ded3d40cd20ddc45d57581beaa7b4d299da9dec8a1f361c808637238fa368e07c7d08f5654c7b2f8a90d47857e9b9c0a81a46769f6307d5a4442707afb017959d9a681fa1dc8d97565e55f02df34b04a3d0a0bf98b7798d7084db4b3f6696fa139f83ada3dc70d0b4c57bf49f530dec938096071f9c4498fdef9641dfbfe516c985b27d1748cc6ce1a4beb1381fb165a3d14f61032e0f76f095d",
            16,
        ).unwrap();

        let s = n.clone().sqrt();

        let ct = Integer::from_str_radix(
            "503d5dd3bf3d76918b868c0789c81b4a384184ddadef81142eabdcb78656632e54c9cb22ac2c41178607aa41adebdf89cd24ec1876365994f54f2b8fc492636b59382eb5094c46b5818cf8d9b42aed7e8051d7ca1537202d20ef945876e94f502e048ad71c7ad89200341f8071dc73c2cc1c7688494cad0110fca4854ee6a1ba999005a650062a5d55063693e8b018b08c4591946a3fc961dae2ba0c046f0848fbe5206d56767aae8812d55ee9decc1587cf5905887846cd3ecc6fc069e40d36b29ee48229c0c79eceab9a95b11d15421b8585a2576a63b9f09c56a4ca1729680410da237ac5b05850604e2af1f4ede9cf3928cbb3193a159e64482928b585ac",
            16,
        ).unwrap();
        // Init the matrix with Integer
        let mut basis: Matrix<BigVector> = Matrix::init(2, 2);

        // Populate the matix
        basis[0] = BigVector::from_vector(vec![e, s.clone()]);
        basis[1] = BigVector::from_vector(vec![n.clone(), Integer::from(0)]);

        bigl2::lattice_reduce(&mut basis, 0.501, 0.99);
        // biglll::lattice_reduce(&mut basis);

        println!("{:#?}", basis[0][0]);
        println!("{:#?}", basis[0][1]);
        println!("{:#?}", basis[1][0]);
        println!("{:#?}", basis[1][1]);

        let d1 = &basis[0][1].clone().abs() / s.clone();
        let d2 = &basis[1][1].clone().abs() / s;

        println!("{:#?}", d1);
        println!("{:#?}", d2);

        let pt1 = ct.clone().pow_mod(&d1, &n).unwrap().to_string_radix(10);
        let pt2 = ct.clone().pow_mod(&d2, &n).unwrap().to_string_radix(10);

        let pt1 = BigInt::from_str_radix(&pt1, 10).unwrap();
        let pt2 = BigInt::from_str_radix(&pt2, 10).unwrap();

        println!("{}", String::from_utf8_lossy(&pt1.to_signed_bytes_be()));
        println!("{}", String::from_utf8_lossy(&pt2.to_signed_bytes_be()));
    }
}
