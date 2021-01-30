use blst;
use num_bigint::BigUint;

pub fn generate(secret: &[u8; 48], degree: u32) -> (Vec<blst::blst_p1>, Vec<blst::blst_p2>) {
    // TODO get from blst?
    let modulus = BigUint::parse_bytes(
        b"52435875175126190479447740508185965837690552500527637822603658699938581184513",
        10,
    )
    .unwrap();
    let s = BigUint::from_bytes_be(secret);
    let mut points_in_g1 = vec![];
    let mut points_in_g2 = vec![];

    unsafe {
        let g1 = blst::blst_p1_generator();
        let g2 = blst::blst_p2_generator();

        for i in 0..=degree {
            let i_as_bigint = BigUint::from_slice(&[i]);
            let s_i_as_bigint = s.modpow(&i_as_bigint, &modulus);

            let mut scalar = blst::blst_scalar::default();
            blst::blst_scalar_from_bendian(&mut scalar, s_i_as_bigint.to_bytes_be().as_ptr());

            let mut result = blst::blst_p1::default();
            blst::blst_p1_mult(&mut result, g1, &scalar, 381);
            points_in_g1.push(result);

            let mut result = blst::blst_p2::default();
            blst::blst_p2_mult(&mut result, g2, &scalar, 381);
            points_in_g2.push(result);
        }
    }

    (points_in_g1, points_in_g2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        let secret = [0u8; 48];
        let degree = 16;

        let setup = generate(&secret, degree);
        assert_eq!(setup.0.len(), (degree + 1) as usize);
    }
}
