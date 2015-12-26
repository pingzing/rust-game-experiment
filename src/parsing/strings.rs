pub fn ordered_inject(base_string: &String, args: Vec<&String>) -> String {
    let mut string_buf = base_string.clone();                
    let mut counter: usize = 0;
    for _ in &args {
        let curr_format_str = format!("{{{}}}", counter.to_string());
        string_buf = string_buf.replace(&curr_format_str, &args[counter]);
        counter = counter + 1;
    }
    return string_buf.to_string();
}