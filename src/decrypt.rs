
// --------- UPDATED CODE -----
// ============================================= DECRYPT  =============================================
//! A particular combination of `Curve25519`, `Blake2B`, `XSalsa20` and `Poly1305`.

extern crate libsodium_sys as ffi;
extern crate std;
extern crate libc;
#[cfg(not(feature = "std"))]
use libc::c_ulonglong;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_;
/// Number of additional bytes in a ciphertext compared to the corresponding
/// plaintext.
pub const SEALBYTES: usize = ffi::crypto_box_SEALBYTES as usize;
use anyhow::{Result};
use std::str;




/// The `decrypt()` function decrypts the ciphertext `c` using the key pair `(pk, sk)`
/// and returns the decrypted message.
///
/// Key pairs are compatible with other
/// `crypto::box_::curve25519xsalsa20poly1305` operations and can be created
/// using `crypto::box::gen_keypair()`.
///
/// This function doesn't require passing the public key of the sender, as the
/// ciphertext already includes this information.
///
/// If decryption fails it returns `Err(())`.
/// 
/// 
 pub fn decrypt(c: Vec<u8>,
                &box_::PublicKey(ref pk): &box_::PublicKey,
                &box_::SecretKey(ref sk): &box_::SecretKey) -> Result<String, ()> {


        println!("decrypt()");
        
        if c.len() < SEALBYTES {
            println!("error!");
            return Err(());
        }

        let mut m = vec![0u8; c.len() - SEALBYTES];

        let ret = unsafe {
            ffi::crypto_box_seal_open(
                // decrypted, ciphertext, CIPHERTEXT_LEN, recipient_pk, recipient_sk
                                    m.as_mut_ptr(),
                                    c.as_ptr(), 
                                    c.len() as c_ulonglong,
                                    pk.as_ptr() as *const u8, 
                                    sk.as_ptr() as *const u8)
        };

        println!("ret: {:?}", ret);

        let m_to_str = str::from_utf8(&m).unwrap();

        if ret == 0 {
            Ok(m_to_str.to_string())
        } else {
            Err(())
        }
    }

