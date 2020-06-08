
// ============================================= ENCRYPT  =============================================

extern crate libsodium_sys as ffi;
extern crate std;
#[cfg(not(feature = "std"))]

use std::fs::File;
use std::io::{BufRead, BufReader};

use sodiumoxide::crypto::{
    box_::{gen_keypair},
    sealedbox,
};
use uuid::Uuid; // Provides support for Universally Unique Identifiers (UUIDs)
use std::io::{BufWriter, Write};
use super::*;
extern crate libc;

    pub fn encrypt_start(){
    // ------------------------------------------ GET JSON TEXT FILE CONTENTS ------------------------------------------------------------------------
    
            // specify the filename and its path to open and read   -- just for now, so just focus on encrypt and decrypt the file - later work on recieve the file I guess?
            let filename = "./example_json_txt_file_to_encrypt";
    
            // Open the file in read-only mode (ignoring errors).
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);
    
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (index, line) in reader.lines().enumerate() {
                let message = line.unwrap(); // Ignore errors.
                println!("read file: {:?}", message);
    
    // ------------------------------------------ PUBLIC KEY AND ENCRYPT IT ------------------------------------------------------------------------
    
                let (pk, sk) = gen_keypair();
                println!("pk: {:?}", pk);
    
                let encrypted_result = sealedbox::seal(message.as_bytes(), &pk);
                println!("encrypted_result: {:?}", encrypted_result);
    // ------------------------------------------ WRITE A FILE ------------------------------------------------------------------------
             
                let encrypted_text = format!("{:?}", encrypted_result);
                write_file(encrypted_text);  
    // ------------------------------------------ CONVERT TO STR ------------------------------------------------------------------------

                //let encrypted_str = str::from_utf8(&encrypted_result).unwrap();

    // ------------------------------------------ DECRYPT IT ------------------------------------------------------------------------

                let opened = decrypt::decrypt(encrypted_result, &pk, &sk);
                println!("decrypted: {:?}", opened);
            }
        }
    

    // ------------------------------------------ WRITE A FILE ------------------------------------------------------------------------
    
    pub fn write_file(encrypted:  String){
        // UUID : unique file name
        let my_uuid = Uuid::new_v4();
        let file_created = File::create(my_uuid.to_string()).expect("create file failed");
    
        // buffer write
        let mut file_generated = BufWriter::new(file_created);
    
        //  write file
        file_generated.write_all(format!("{:?}", encrypted).as_bytes()).expect("write failed");

        println!("encrypted txt file generated. uuid : {}", my_uuid);

    }
   
 