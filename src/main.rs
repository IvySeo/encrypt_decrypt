
/*
Cryptography is used to secure and protect data during communication. 
It is helpful to prevent unauthorized person or group of users from accessing any confidential data. 

Encryption: a process which transforms the original information into an unrecognizable form.
Decryption: a process of converting encoded/encrypted data in a form that is readable and understood by a human or a computer. 
Public key: an encryption system which is based on two pairs of keys. Public keys are used to encrypt messages for a receiver.
Sealed box: designed to anonymously send messages to a recipient given its public key.
Only the recipient can decrypt these messages, using its private key. 

-- how to run --
- set up Rust
- to start the program to encrypt/decrypt: simply type 'cargo run' in a terminal
*/

mod encrypt;
mod decrypt;

    fn main() {
        println!("main start");

        // start encrypting and decrypting - the function located in encrypt.rs mod
        encrypt::encrypt_start();

    }

