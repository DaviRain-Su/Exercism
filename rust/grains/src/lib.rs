// pub fn square(s: u32) -> u64 {
//     if s < 1 || s > 64 {
//         panic!("Square must be between 1 and 64")
//     }
//     let mut ret : Vec<u64> = vec![0; 64];
//     ret[0] = 1;
//     for i in (1..s){
//         ret[i as usize] = ret[(i - 1) as usize] * 2;
//     }
//     println!("ret = {:?}", ret);
//     ret[(s-1) as usize] as u64
// }

// pub fn total() -> u64 {
//     let mut sum = 0;
//     for i in (1..65) {
//         sum += square(i);
//     }
//     sum 
// }

pub fn square(s: u32) -> u64 {
    match s {
        1...64 => 1u64.wrapping_shl(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}