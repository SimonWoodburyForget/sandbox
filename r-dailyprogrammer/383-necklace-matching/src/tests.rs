use super::*;

#[test]
fn necklaces_small_test() {
    assert_eq!(necklaces(3, 10), 5934);
    assert_eq!(necklaces(2, 12), 352);
    assert_eq!(necklaces(3, 7), 315);
    assert_eq!(necklaces(9, 4), 1665);
    assert_eq!(necklaces(21, 3), 3101);
    assert_eq!(necklaces(99, 2), 4950);

    // assert_eq!(prime.necklaces_generic::<u128>(3, 10), 5934);
}

#[test]
fn necklaces_u128_test() {
    assert_eq!(
        necklaces(12345678910, 3),
        627225458787209496560873442940_u128
    );

    assert_eq!(
        necklaces(1234567, 6),
        590115108867910855092196771880677924_u128
    );

    assert_eq!(
        necklaces(123, 18),
        2306850769218800390268044415272597042_u128,
    );
}

#[test]
fn necklaces_large_test() {
    assert_eq!(
        necklaces_big(3, 90),
        BigUint::parse_bytes(b"96977372978752360287715019917722911297222", 10).unwrap()
    );

    assert_eq!(
        necklaces_big(123, 18),
        2306850769218800390268044415272597042_u128.into()
    );

    assert_eq!(
        necklaces_big(1234567, 6),
        590115108867910855092196771880677924_u128.into()
    );

    assert_eq!(
        necklaces_big(12345678910, 3),
        627225458787209496560873442940_u128.into()
    );
}

#[test]
fn phi_test() {
    // assert_eq!(phi(97), 96);

    // hard coded test 1
    // let phis = vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8];
    // for (i, &phi) in phis.iter().enumerate() {
    //     assert_eq!(phi(i + 1), phi);
    // }

    // // primes test
    // for &number in prime.numbers.iter() {
    //     assert_eq!(prime.phi(number), number - 1);
    // }

    // hard coded test 2
    // assert_eq!(phi(20), 8);
    // assert_eq!(phi(36), 12);
    // assert_eq!(phi(81), 54);
    // assert_eq!(phi(90), 24);
}

#[test]
fn primes_numbers() {
    let mut buffer = [true; 100];
    let p: Vec<_> = sieve_erato(&mut buffer).collect();
    assert_eq!(&p[..5], &[2, 3, 5, 7, 11]);
}
