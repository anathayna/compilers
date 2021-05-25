use regex::Regex;

fn main() {
    let re = Regex::new(r#"\b123\b"#).unwrap();
    let txt = r#"number 123 appeared here and there"#;

    for capture in re.captures_iter(txt) {
        println!("number = {:?}", capture);
    }

    let s = re.replace_all(txt, "999");
    println!("{}", s);
}
