pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|value|{
        factors.iter().any(|item|{
            *item != 0 && value % item == 0
        })
    }).sum()
    
}
