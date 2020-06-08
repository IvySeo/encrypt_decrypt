# encrypt_decrypt_text_file
It reads a text file -> encrypts it line by line using a sealed box method of the "libsodium" library -> write a file using UUID -> decrypts it to the original context

**[HOW TO RUN] :**

***1-a) Set up Rustup / Cargo***
  - rust/cargo : https://doc.rust-lang.org/book/ch01-01-installation.html
  
***1-b) install Rust in Linux terminal***
- $ curl https://sh.rustup.rs -sSf | sh
- $ source $HOME/.cargo/env
- $ export PATH="$HOME/.cargo/bin:$PATH"

***2) sodiumoxide library***
- libaray: https://github.com/sodiumoxide/sodiumoxide

***3) type 'cargo run' to run the program***

***4) feel free to change value/message to encrypt/decrypt***
#

**Cryptography**
Cryptography is used to secure and protect data during communication. 
It is helpful to prevent unauthorized person or group of users from accessing any confidential data. 

**Encryption** a process which transforms the original information into an unrecognizable form.

**Decryption** a process of converting encoded/encrypted data in a form that is readable and understood by a human or a computer. 

**Public key** an encryption system which is based on two pairs of keys. Public keys are used to encrypt messages for a receiver.

**Sealed box**: designed to anonymously send messages to a recipient given its public key.
Only the recipient can decrypt these messages, using its private key. 
