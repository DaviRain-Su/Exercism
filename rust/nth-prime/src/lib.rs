pub fn nth(n: u32) -> u32 {
    let mut cnt = -1i32;
    for num in 0.. {
        if is_prime(num) {
            cnt += 1;
        }

        if cnt as u32 == n {
            return num;
        }
    }
    2  
}


fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }
    let mut i = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}