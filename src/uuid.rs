#[cfg(target_os = "windows")]
fn random_bytes() -> [u8; 16] {
    #[link(name = "bcrypt")]
    unsafe extern "system" {
        fn BCryptGenRandom(
            h_algorithm: *mut std::ffi::c_void,
            buffer: *mut u8,
            buffer_length: u32,
            flags: u32,
        ) -> i32;
    }

    const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

    let mut bytes = [0u8; 16];
    let status = unsafe {
        BCryptGenRandom(
            std::ptr::null_mut(),
            bytes.as_mut_ptr(),
            bytes.len() as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        )
    };

    if status != 0 {
        panic!("Failed to generate random bytes");
    }

    bytes
}

#[cfg(target_os = "linux")]
fn random_bytes() -> [u8; 16] {
    unsafe extern "C" {
        fn getrandom(buf: *mut u8, buflen: usize, flags: u32) -> isize;
    }
    
    let mut bytes = [0u8; 16];
    let n = unsafe { 
        getrandom(bytes.as_mut_ptr(), bytes.len(), 0) 
    };

    if n < 0 { 
        panic!("getrandom failed (errno set)"); 
    }

    if n as usize != bytes.len() { 
        panic!("getrandom short read: {n} of {}", bytes.len()); 
    }

    bytes
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
fn random_bytes() -> [u8; 16] {
    unsafe extern "C" {
        fn arc4random_buf(buf: *mut core::ffi::c_void, n: usize);
    }

    let mut bytes = [0u8; 16];
    
    unsafe { 
        arc4random_buf(bytes.as_mut_ptr().cast(), bytes.len()) 
    };

    bytes
}

/* UUID version 4 */
pub fn uuid_v4() -> String {
    let mut bytes = random_bytes();

    bytes[6] = (bytes[6] & 0x0f) | 0x40; // UUID version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // UUID variant

    format!(
        "{:02x}{:02x}{:02x}{:02x}-\
         {:02x}{:02x}-\
         {:02x}{:02x}-\
         {:02x}{:02x}-\
         {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12],
        bytes[13], bytes[14], bytes[15]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_valid_v4() {
        let u = super::uuid_v4();
        assert_eq!(u.len(), 36);
        assert_eq!(u.as_bytes()[14], b'4');
        assert!(matches!(u.as_bytes()[19], b'8' | b'9' | b'a' | b'b'));
    }
}