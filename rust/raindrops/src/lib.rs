pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    // let mut result = String::new();

    // if n % 3 == 0 {
        // result.push_str("Pling");
    // }

    // if n % 5 == 0 {
        // result.push_str("Plang");
    // }

    // if n % 7 == 0 {
        // result.push_str("Plong");
    // }

    // if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        // let n_str = n.to_string();
        // result.push_str(&n_str);
    // }
    
    //the way 2
    // if result.is_empty() {
        // result.push_str(&n.to_string());
        // return n.to_string();
    // }

    // result

    // let mut out = String::new();
    // let mut if_is_factor_push = | factor, sound | {
        // if n % factor == 0 {
            // out.push_str(sound);
        // }
    // };

    // if_is_factor_push(3, "Pling");
    // if_is_factor_push(5, "Plang");
    // if_is_factor_push(7, "Plong");
    // for (number, text) in [(3, "Pling"), (5,"Plang"), (7, "Plong")].iter() {
        // if n % number == 0 {
            // out.push_str(text);
        // }
    // }

    // if out.is_empty() {
        // n.to_string()
    // } else {
        // out
    // }

    let codes = vec![
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ];  

    let word : String = codes.into_iter()
        .filter(|&(factor, _)| n % factor == 0 )
        .fold(String::new(), |acc, (_, code) | acc + code );
    
    if word.is_empty() {
        n.to_string()
    }else {
        word
    }


}
