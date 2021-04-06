#[derive(Debug)]
pub enum ItemLexico {
    Numero(u64),
    Operador(char)
}

pub fn lexer(exp: &String) -> Result<Vec<ItemLexico>, String> {
    let mut r = Vec::new();
    let mut iter = exp.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0' ..= '9' => {
                let n = c.to_string().parse().unwrap();
                r.push(ItemLexico::Numero(n));
                iter.next();
            }
            '+' | '-' | '*' | '/' => {
                r.push(ItemLexico::Operador(c));
                iter.next();
            }
            _ => {
                return Err(format!("caractere inesperado: {}", c));
            }
        }
    }

    Ok(r)
}