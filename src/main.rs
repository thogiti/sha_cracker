/// SHA1 Cracker in Rust

use sha1::Digest;
use std::{env, error::Error, fs::File, io::{BufRead, BufReader},};

const SHA1_HEX_STRING_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {

    // get all the command line parameters together
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("How to Use SHA Cracker Command");
        println!("sha_cracker wordlist.txt sha_hash");
        return Ok(());
    }

    //get the hash value of the sha to crack
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 hash is not valid".into());
    }

    //open the word list file
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_passwd = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_passwd.as_bytes())) {
            println!("Password found: {}", &common_passwd);
            return Ok(());
        }
    }

    println!("Password not found in the wordlist!");
    Ok(())


    
}
