use blake2_rfc::blake2b::Blake2b;

use jubjub::{JubjubEngine, ToUniform};

pub fn swap_bits_u64(x: u64) -> u64
{
    let mut tmp = 0;
    for i in 0..64 {
        tmp |= ((x >> i) & 1) << (63 - i);
    }
    tmp
}

pub fn hash_to_scalar<E: JubjubEngine>(persona: &[u8], a: &[u8], b: &[u8]) -> E::Fs {
    let mut hasher = Blake2b::with_params(64, &[], &[], persona);
    hasher.update(a);
    hasher.update(b);
    let ret = hasher.finalize();
    E::Fs::to_uniform(ret.as_ref())
}

#[test]
fn test_swap_bits_u64() {
    assert_eq!(swap_bits_u64(17182120934178543809), 0b1000001100011011110000011000111000101111111001001100111001110111);
    assert_eq!(swap_bits_u64(15135675916470734665), 0b1001001011110010001101010010001110110000100111010011000001001011);
    assert_eq!(swap_bits_u64(6724233301461108393),  0b1001010101100000100011100001010111110001011000101000101010111010);
    assert_eq!(swap_bits_u64(206708183275952289),   0b1000010100011010001010100011101011111111111110100111101101000000);
    assert_eq!(swap_bits_u64(12712751566144824320), 0b0000000000100110010110111000001110001100001000110011011000001101);

    let mut a = 15863238721320035327u64;
    for _ in 0..1000 {
        a = a.wrapping_mul(a);

        let swapped = swap_bits_u64(a);
        let unswapped = swap_bits_u64(swapped);

        assert_eq!(a, unswapped);
    }
}
