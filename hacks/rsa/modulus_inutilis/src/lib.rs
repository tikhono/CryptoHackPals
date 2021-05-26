#[cfg(test)]
mod tests {
    use num::bigint::BigInt;
    use num::Num;

    #[test]
    fn capture_the_flag() {
        let e = 3;
        let ct = BigInt::from_str_radix(
            "243251053617903760309941844835411292373350655973075480264001352919865180151222189820473358411037759381328642957324889519192337152355302808400638052620580409813222660643570085177957",
            10,
        )
        .unwrap();
        let pt = ct.nth_root(e);

        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }
}
