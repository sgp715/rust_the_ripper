
pub fn transform(word: String)->Vec<String>{
    append_number(word)
}

#[test]
fn transform_test(){
    assert_eq!(transform("".to_string()), vec!["".to_string()]);
}
pub fn append_number(word: String)->Vec<String>{
    let mut appended = vec![];
    for i in 0..9999 {
        let mut test = format!("{}",word);
        test.push_str(&i.to_string());
        appended.push(test);
    }
    appended
}
