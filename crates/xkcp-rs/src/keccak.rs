use crate::{Error, Result};
use core::mem::MaybeUninit;
use ffi::Keccak_HashInstance;

/// Keccak hash function instance.
#[derive(Debug)]
#[repr(align(8))]
pub struct KeccakHash {
    inner: Keccak_HashInstance,
}

impl Clone for KeccakHash {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Keccak_HashInstance {
                sponge: ffi::KeccakWidth1600_SpongeInstanceStruct {
                    ..self.inner.sponge
                },
                ..self.inner
            },
        }
    }
}

impl KeccakHash {
    /// Initialize the `Keccak[r, c]` sponge function instance used in sequential hashing mode.
    #[inline]
    pub fn new(rate: u32, capacity: u32, hash_bit_length: u32, suffix: u8) -> Result<Self> {
        let mut instance = MaybeUninit::uninit();
        Error::from_raw(unsafe {
            ffi::Keccak_HashInitialize(
                instance.as_mut_ptr(),
                rate,
                capacity,
                hash_bit_length,
                suffix,
            )
        })?;
        Ok(Self {
            inner: unsafe { instance.assume_init() },
        })
    }

    /// Initializes a SHAKE128 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn shake128() -> Self {
        unwrap_unreachable(Self::new(1344, 256, 0, 0x1F))
    }

    /// Initializes a SHAKE256 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn shake256() -> Self {
        unwrap_unreachable(Self::new(1088, 512, 0, 0x1F))
    }

    /// Initializes a SHA3-224 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn sha3_224() -> Self {
        unwrap_unreachable(Self::new(1152, 448, 224, 0x06))
    }

    /// Initializes a SHA3-256 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn sha3_256() -> Self {
        unwrap_unreachable(Self::new(1088, 512, 256, 0x06))
    }

    /// Initializes a SHA3-384 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn sha3_384() -> Self {
        unwrap_unreachable(Self::new(832, 768, 384, 0x06))
    }

    /// Initializes a SHA3-512 instance as specified in the FIPS 202 standard.
    #[inline]
    pub fn sha3_512() -> Self {
        unwrap_unreachable(Self::new(576, 1024, 512, 0x06))
    }

    /// Initializes a Keccak-224 instance.
    #[inline]
    pub fn keccak224() -> Self {
        unwrap_unreachable(Self::new(1152, 448, 224, 0x01))
    }

    /// Initializes a Keccak-256 instance.
    #[inline]
    pub fn keccak256() -> Self {
        unwrap_unreachable(Self::new(1088, 512, 256, 0x01))
    }

    /// Initializes a Keccak-384 instance.
    #[inline]
    pub fn keccak384() -> Self {
        unwrap_unreachable(Self::new(832, 768, 384, 0x01))
    }

    /// Initializes a Keccak-512 instance.
    #[inline]
    pub fn keccak512() -> Self {
        unwrap_unreachable(Self::new(576, 1024, 512, 0x01))
    }

    /// Absorbs input data.
    #[inline]
    pub fn update(&mut self, data: &[u8]) -> Result<()> {
        Error::from_raw(unsafe {
            ffi::Keccak_HashUpdate(&mut self.inner, data.as_ptr(), data.len() * 8)
        })
    }

    /// Function to call after all input blocks have been input and to get output bits if the length was specified.
    #[inline]
    pub fn finalize(&mut self, out: &mut [u8]) -> Result<()> {
        if self.inner.fixedOutputLength > 0
            && out.len() * 8 != self.inner.fixedOutputLength as usize
        {
            return Err(Error::OutputTooSmall);
        }
        Error::from_raw(unsafe { ffi::Keccak_HashFinal(&mut self.inner, out.as_mut_ptr()) })
    }

    /// Squeezes output data.
    #[inline]
    pub fn squeeze(&mut self, data: &mut [u8]) -> Result<()> {
        Error::from_raw(unsafe {
            ffi::Keccak_HashSqueeze(&mut self.inner, data.as_mut_ptr(), data.len() * 8)
        })
    }
}

#[inline(always)]
fn unwrap_unreachable<T, E>(x: Result<T, E>) -> T {
    #[inline(never)]
    #[cold]
    fn unreachable() -> ! {
        unreachable!()
    }

    match x {
        Ok(x) => x,
        Err(_) => unreachable(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sha3_256() {
        fn run(x: &[u8]) -> [u8; 32] {
            let mut hash = KeccakHash::sha3_256();
            hash.update(x).unwrap();
            let mut out = [0u8; 32];
            hash.finalize(&mut out).unwrap();

            let mut out2 = [0u8; 32];
            crate::sha3_256(x, &mut out2);
            assert_eq!(out2, out);

            out
        }

        assert_eq!(
            run(b""),
            hex!("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a")
        );
        assert_eq!(
            run(b"a"),
            hex!("80084bf2fba02475726feb2cab2d8215eab14bc6bdd8bfb2c8151257032ecd8b")
        );
        assert_eq!(
            run(b"Hello, World!"),
            hex!("1af17a664e3fa8e419b8ba05c2a173169df76162a5a286e0c405b460d478f7ef")
        );
    }

    #[test]
    fn keccak256() {
        fn run(x: &[u8]) -> [u8; 32] {
            let mut hash = KeccakHash::keccak256();
            hash.update(x).unwrap();
            let mut out = [0u8; 32];
            hash.finalize(&mut out).unwrap();

            let mut out2 = [0u8; 32];
            crate::keccak256(x, &mut out2);
            assert_eq!(out2, out);

            out
        }

        assert_eq!(
            run(b""),
            hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
        );
        assert_eq!(
            run(b"a"),
            hex!("3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb")
        );
        assert_eq!(
            run(b"Hello, World!"),
            hex!("acaf3289d7b601cbd114fb36c4d29c85bbfd5e133f14cb355c3fd8d99367964f")
        );
    }
}
