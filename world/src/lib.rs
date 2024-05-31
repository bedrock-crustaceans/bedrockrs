mod world;
pub mod world_db;
pub mod level_dat;
pub mod error;

fn str_to_ascii_i8(s: &str) -> Result<Vec<i8>, &'static str> {

    if !s.is_ascii() {
        return Err("Input string contains non-ASCII characters");
    }

    let bytes = s.as_bytes();

    let ascii_i8: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();

    Ok(ascii_i8)
}

fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {

    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {

    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}