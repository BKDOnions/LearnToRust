extern crate openssl;

use openssl::symm::*;

const KEY: &'static [u8] = b"1234567890ABCDEF";
const IV: &'static [u8] = b"1234567890ABCDEF";
const CIPHERTEXT: &'static [u8] = b"ebaa635663dc5ec836387d922fee4d89a933513039c6236d89b878b8e5bf9a16d7c9c6957430788f9c5a4cf19e95e43e681a03dab930a453c712973b8f4f68f6f529da694477216f14d5e5c4f47a5c53";

pub fn decipher(key: &str, iv: &str, data: &str) {
    let ciphertext = data.as_bytes();

    let t = Cipher::aes_128_cbc();
    let mut d = Crypter::new(t, Mode::Decrypt, key.as_bytes(), Some(iv.as_bytes())).unwrap();
    let mut result = vec![0; CIPHERTEXT.len() + t.block_size()];
    d.update(ciphertext, &mut result).unwrap();
    let len = d.finalize(&mut result).unwrap();
    result.truncate(len);
    println!("{:?}", result);
    println!("{:?}", String::from_utf8_lossy(&result));
}
