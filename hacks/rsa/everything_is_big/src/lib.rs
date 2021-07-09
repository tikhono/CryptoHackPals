use num::bigint::BigInt;
use num::ToPrimitive;

//The best explanation of perfect_square algorithm
// https://stackoverflow.com/a/424936
// https://stackoverflow.com/a/22008277
pub fn is_perfect_square(n: BigInt) -> bool {
    let sq_mod256 = [
        1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ];
    if sq_mod256[(n.clone() % BigInt::from(256)).to_usize().unwrap()] == 0 {
        return false;
    }
    let t = vec![
        vec![1, 1, 0, 0, 1, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1],
    ];

    let a: BigInt = n.clone() % ((((9 * 5) * 7) * 13) * 17);

    for t in t.iter() {
        if t[a.clone().to_usize().unwrap() % t.len()] == 0 {
            return false;
        }
    }
    n.sqrt().pow(2) == n
}

pub fn rational_to_contfrac(mut x: BigInt, mut y: BigInt) -> Vec<BigInt> {
    let mut res = Vec::new();
    while y > BigInt::from(0) {
        let a = x.clone() / y.clone();
        res.push(a.clone());
        let temp_y = y.clone();
        y = x.clone() - a * y.clone();
        x = temp_y;
    }
    res
}

pub fn contfrac_to_rational_iter(contfrac: Vec<BigInt>) -> Vec<(BigInt, BigInt)> {
    let mut res: Vec<(BigInt, BigInt)> = Vec::new();

    let (mut n0, mut d0) = (BigInt::from(0), BigInt::from(1));
    let (mut n1, mut d1) = (BigInt::from(1), BigInt::from(0));
    for q in contfrac {
        let n = (q.clone() * n1.clone()) + n0.clone();
        let d = (q * d1.clone()) + d0.clone();
        res.push((n.clone(), d.clone()));
        n0 = n1.clone();
        d0 = d1.clone();
        n1 = n.clone();
        d1 = d.clone();
    }
    res
}

pub fn convergents_from_contfrac(contfrac: Vec<BigInt>) -> Vec<(BigInt, BigInt)> {
    let mut res = Vec::new();
    let (mut n_, mut d_) = (BigInt::from(1), BigInt::from(0));
    for (i, (n, d)) in contfrac_to_rational_iter(contfrac).iter().enumerate() {
        if (i % 2) == 0 {
            res.push((n + n_.clone(), d + d_.clone()));
        } else {
            res.push((n.clone(), d.clone()));
        }
        n_ = n.clone();
        d_ = d.clone();
    }
    res
}

pub fn attack(e: BigInt, n: BigInt) -> Option<BigInt> {
    let f_ = rational_to_contfrac(e.clone(), n.clone());
    let iter = convergents_from_contfrac(f_);
    for (k, dg) in iter {
        let edg = e.clone() * dg.clone();
        let phi = edg.clone() / k.clone();
        let x = (n.clone() - phi.clone()) + BigInt::from(1);
        if (x.clone() % BigInt::from(2)) == BigInt::from(0)
            && is_perfect_square((x.clone() / BigInt::from(2)).pow(2) - n.clone())
        {
            let g = edg.clone() - (phi.clone() * k.clone());
            return Some(dg.clone() / g.clone());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::{attack, is_perfect_square, rational_to_contfrac};
    use num::bigint::BigInt;
    use num::{Num, One};

    #[test]
    fn capture_the_flag() {
        // used Wienner attack
        // https://github.com/orisano/owiener

        let n = BigInt::from_str_radix(
            "8da7d2ec7bf9b322a539afb9962d4d2ebeb3e3d449d709b80a51dc680a14c87ffa863edfc7b5a2a542a0fa610febe2d967b58ae714c46a6eccb44cd5c90d1cf5e271224aa3367e5a13305f2744e2e56059b17bf520c95d521d34fdad3b0c12e7821a3169aa900c711e6923ca1a26c71fc5ac8a9ff8c878164e2434c724b68b508a030f86211c1307b6f90c0cd489a27fdc5e6190f6193447e0441a49edde165cf6074994ea260a21ea1fc7e2dfb038df437f02b9ddb7b5244a9620c8eca858865e83bab3413135e76a54ee718f4e431c29d3cb6e353a75d74f831bed2cc7bdce553f25b617b3bdd9ef901e249e43545c91b0cd8798b27804d61926e317a2b745",
            16,
        )
            .unwrap();

        let e = BigInt::from_str_radix("86d357db4e1b60a2e9f9f25e2db15204c820b6e8d8d04d29db168c890bc8a6c1e31b9316c9680174e128515a00256b775a1a8ccca9c6936f1b4c2298c03032cda4dd8eca1145828d31466bf56bfcf0c6a8b4a1b2fb27de7a57fae7430048d7590734b2f05b6443ad60d89606802409d2fa4c6767ad42bffae01a8ef1364418362e133fa7b2770af64a68ad50ad8d2bd5cebb99ceb13368fb31a6e7503e753f8638e21a96af1b6498c18578ba89b98d70fa482ad137d28fe701b4b77baa25d5e84c81b26ee9bddf8cbb51a071c60dd57714de379cd4bc14932809ba18524a0a18e4133665cfc46e2c4fcfbc28e0a0957e5513a7307c422b87a6182d0b6a074b4d", 16).unwrap();
        let ct = BigInt::from_str_radix(
            "6a2f2e401a54eeb5dab1e6d5d80e92a6ca189049e22844c825012b8f0578f95b269b19644c7c8af3d544840d380ed75fdf86844aa8976622fa0501eaec0e5a1a5ab09d3d1037e55501c4e270060470c9f4019ced6c4e67673843daf2fd71c64f3dd8939ae322f2b79d283b3382052d076ebe9bb50b0042f1f7dd7beadf0f5686926ade9fc8370283ead781a21896e7a878d99e77c3bb1f470401062c0e0327fd85da1cf12901635f1df310e8f8c7d87aff5a01dbbecd739cd8f36462060d0eb237af8d613e2d9cebb67d612bcfc353ef2cd44b7ac85e471287eb04ae9b388b66ea8eb32429ae96dba5da8206894fa8c58a7440a127fceb5717a2eaa3c29f25f7",
            16,
        )
            .unwrap();

        let d = attack(e, n.clone()).unwrap();
        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }

    #[test]
    fn test_attack() {
        assert_eq!(
            BigInt::from(5),
            attack(BigInt::from(2621), BigInt::from(8927)).unwrap()
        );
        assert_eq!(
            BigInt::from(569),
            attack(
                BigInt::from_str_radix("6792605526025", 10).unwrap(),
                BigInt::from_str_radix("9449868410449", 10).unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            BigInt::from_str_radix("4221909016509078129201801236879446760697885220928506696150646938237440992746683409881141451831939190609743447676525325543963362353923989076199470515758399", 10).unwrap(),
            attack(
                BigInt::from_str_radix("30749686305802061816334591167284030734478031427751495527922388099381921172620569310945418007467306454160014597828390709770861577479329793948103408489494025272834473555854835044153374978554414416305012267643957838998648651100705446875979573675767605387333733876537528353237076626094553367977134079292593746416875606876735717905892280664538346000950343671655257046364067221469807138232820446015769882472160551840052921930357988334306659120253114790638496480092361951536576427295789429197483597859657977832368912534761100269065509351345050758943674651053419982561094432258103614830448382949765459939698951824447818497599", 10).unwrap(),
                BigInt::from_str_radix("109966163992903243770643456296093759130737510333736483352345488643432614201030629970207047930115652268531222079508230987041869779760776072105738457123387124961036111210544028669181361694095594938869077306417325203381820822917059651429857093388618818437282624857927551285811542685269229705594166370426152128895901914709902037365652575730201897361139518816164746228733410283595236405985958414491372301878718635708605256444921222945267625853091126691358833453283744166617463257821375566155675868452032401961727814314481343467702299949407935602389342183536222842556906657001984320973035314726867840698884052182976760066141", 10).unwrap()
            )
                .unwrap()
        );
    }

    #[test]
    fn perfect_square() {
        assert_eq!(true, is_perfect_square(BigInt::from(100)));
        assert_eq!(
            true,
            is_perfect_square(
                BigInt::from_str_radix("2000000000000000000000000000", 10)
                    .unwrap()
                    .pow(2)
            )
        );
        assert_eq!(
            false,
            is_perfect_square(
                BigInt::from_str_radix("2000000000000000000000000000", 10)
                    .unwrap()
                    .pow(2)
                    + BigInt::one()
            )
        );
    }

    #[test]
    fn rational() {
        assert_eq!(
            vec![
                BigInt::from(0),
                BigInt::from(2),
                BigInt::from(1),
                BigInt::from(3)
            ],
            rational_to_contfrac(BigInt::from(4), BigInt::from(11))
        )
    }
}
