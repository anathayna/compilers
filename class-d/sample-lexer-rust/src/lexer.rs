#[derive(Debug)]
pub enum ItemLexico {
    Numero(u64)
}

pub fn lexer(exp: &String) -> Result<Vec<ItemLexico>, String> {
    //unimplemented!();
    let mut r = Vec::new();
    let numero_result = exp.parse();
    match numero_result {
        Ok(numero) => {
            r.push(ItemLexico::Numero(numero));
            return Ok(r);
        }
        Err(_) => {
            return Err(format!("valor inesperado: {}", exp));
        }
    }
}