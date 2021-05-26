#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::Num;

    #[test]
    fn capture_the_flag() {
        let n = BigInt::from_str_radix(
            "580642391898843192929563856870897799650883152718761762932292482252152591279871421569162037190419036435041797739880389529593674485555792234900969402019055601781662044515999210032698275981631376651117318677368742867687180140048715627160641771118040372573575479330830092989800730105573700557717146251860588802509310534792310748898504394966263819959963273509119791037525504422606634640173277598774814099540555569257179715908642917355365791447508751401889724095964924513196281345665480688029639999472649549163147599540142367575413885729653166517595719991872223011969856259344396899748662101941230745601719730556631637",
            10,
        )
        .unwrap();
        let e = BigInt::from(65537);
        let ct = BigInt::from_str_radix(
            "320721490534624434149993723527322977960556510750628354856260732098109692581338409999983376131354918370047625150454728718467998870322344980985635149656977787964380651868131740312053755501594999166365821315043312308622388016666802478485476059625888033017198083472976011719998333985531756978678758897472845358167730221506573817798467100023754709109274265835201757369829744113233607359526441007577850111228850004361838028842815813724076511058179239339760639518034583306154826603816927757236549096339501503316601078891287408682099750164720032975016814187899399273719181407940397071512493967454225665490162619270814464",
            10,
        )
        .unwrap();

        // I used http://factordb.com to obtain factors
        // It seems that this solution is correct for this task
        // All solutions posted contains the same approach

        let vec: Vec<BigInt> = vec![
            BigInt::from_str_radix("9282105380008121879", 10).unwrap(),
            BigInt::from_str_radix("9303850685953812323", 10).unwrap(),
            BigInt::from_str_radix("9389357739583927789", 10).unwrap(),
            BigInt::from_str_radix("10336650220878499841", 10).unwrap(),
            BigInt::from_str_radix("10638241655447339831", 10).unwrap(),
            BigInt::from_str_radix("11282698189561966721", 10).unwrap(),
            BigInt::from_str_radix("11328768673634243077", 10).unwrap(),
            BigInt::from_str_radix("11403460639036243901", 10).unwrap(),
            BigInt::from_str_radix("11473665579512371723", 10).unwrap(),
            BigInt::from_str_radix("11492065299277279799", 10).unwrap(),
            BigInt::from_str_radix("11530534813954192171", 10).unwrap(),
            BigInt::from_str_radix("11665347949879312361", 10).unwrap(),
            BigInt::from_str_radix("12132158321859677597", 10).unwrap(),
            BigInt::from_str_radix("12834461276877415051", 10).unwrap(),
            BigInt::from_str_radix("12955403765595949597", 10).unwrap(),
            BigInt::from_str_radix("12973972336777979701", 10).unwrap(),
            BigInt::from_str_radix("13099895578757581201", 10).unwrap(),
            BigInt::from_str_radix("13572286589428162097", 10).unwrap(),
            BigInt::from_str_radix("14100640260554622013", 10).unwrap(),
            BigInt::from_str_radix("14178869592193599187", 10).unwrap(),
            BigInt::from_str_radix("14278240802299816541", 10).unwrap(),
            BigInt::from_str_radix("14523070016044624039", 10).unwrap(),
            BigInt::from_str_radix("14963354250199553339", 10).unwrap(),
            BigInt::from_str_radix("15364597561881860737", 10).unwrap(),
            BigInt::from_str_radix("15669758663523555763", 10).unwrap(),
            BigInt::from_str_radix("15824122791679574573", 10).unwrap(),
            BigInt::from_str_radix("15998365463074268941", 10).unwrap(),
            BigInt::from_str_radix("16656402470578844539", 10).unwrap(),
            BigInt::from_str_radix("16898740504023346457", 10).unwrap(),
            BigInt::from_str_radix("17138336856793050757", 10).unwrap(),
            BigInt::from_str_radix("17174065872156629921", 10).unwrap(),
            BigInt::from_str_radix("17281246625998849649", 10).unwrap(),
        ];
        let phi = vec.iter().fold(BigInt::from(1), |acc, v| acc * (v - 1));

        let d = mod_inverse(e, phi).unwrap();

        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }
}
