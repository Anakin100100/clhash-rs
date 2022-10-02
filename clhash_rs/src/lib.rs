use clhash_sys;
use rand::{thread_rng, Rng};
use std::ffi::CString;
use std::os::raw::c_void;

pub fn get_random_key_for_clhash(seeds: Option<(u64, u64)>) -> *mut c_void {
    match seeds {
        // Match a single value
        Some(seeds) => unsafe { clhash_sys::get_random_key_for_clhash(seeds.0, seeds.1) },
        // Match several values
        None => {
            let mut rng = thread_rng();
            let seed1: u64 = rng.gen();
            let seed2: u64 = rng.gen();

            unsafe { clhash_sys::get_random_key_for_clhash(seed1, seed2) }
        }
    }
}

pub fn clhash(random_seed: *mut c_void, text: String) -> u64 {
    let text_len: u64 = text.len() as u64;
    unsafe {
        let c_str_text: CString = CString::new(text).unwrap();
        clhash_sys::clhash(random_seed, c_str_text.as_ptr(), text_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clhash_random_seed_no_seeds() {
        get_random_key_for_clhash(None);
    }

    #[test]
    fn test_clhash_random_seed_with_provided_seeds() {
        get_random_key_for_clhash(Some((32, 21)));
    }

    #[test]
    fn test_clhash() {
        let seed: *mut c_void = get_random_key_for_clhash(None);
        let hash_1: u64 = clhash(seed, String::from("sixhvhuidsfhgjkhdfjnkvbbkjnzxjdfjb"));
        let hash_2: u64 = clhash(seed, String::from("sixhvhuidsfhgjkhdfjnkvbbkjnzxjdfjb"));
        assert_eq!(hash_1, hash_2);
    }
}
