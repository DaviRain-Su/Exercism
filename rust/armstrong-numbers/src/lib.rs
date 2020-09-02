pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len();

    let sum = num.to_string()
                    .chars()
                    .map(|val|{
                        val.to_string().parse::<i32>().unwrap()
                    })
                    .fold(0, |acc, val| {
                        acc + val.pow(len as u32)
                        
                    });

    num == sum as u32
}
