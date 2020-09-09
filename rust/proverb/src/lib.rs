pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    // const STATEM : &'static str = "For want of a {} the {} was lost.";
    // const END_STATEM: &'static str = "And all for the want of a nail";
    let mut result = String::new();

    if list.is_empty() { 
        return String::new(); 
    }else if list.len() == 1 {
        return format!("And all for the want of a {}.", list[0]);
    }else if list.len() == 2 {
        
        let temp = format!("For want of a {} the {} was lost.", list[0], list[1]);
        result.push_str(&temp);
        result.push('\n');
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    } else {
        let temp: Vec<String> =  list.iter().zip(list.iter().skip(1))
            .map(|value|{
                format!("For want of a {} the {} was lost.", value.0, value.1)
            })
            .collect();
        let mut temp = temp.join("\n");
        temp.push('\n');
        temp.push_str(&format!("And all for the want of a {}.", list[0]));
        result = temp;
    }
    // unimplemented!()
    result
}
