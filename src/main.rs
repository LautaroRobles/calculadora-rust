use std::{io::{self, Write}, iter::Peekable, str::Chars};

enum TokenTipo {
    Numero,
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Final,
}

struct Token {
    tipo: TokenTipo,
    valor: Option<i32>,
}

fn main() {
    let mut input = String::new();

    print!("> ");
    io::stdout().flush().expect("No se pudo leer su input");

    io::stdin().read_line(&mut input).expect("No se pudo leer su input");

    let mut iter: Peekable<Chars> = input.chars().peekable();

    parse(&mut iter);
}

fn parse(iter: &mut Peekable<Chars>) {

    let resultado = expresion(iter);

    println!("> {resultado}");
}

fn expresion(iter: &mut Peekable<Chars>) -> i32 {
    let mut resultado = termino(iter);

    loop {
        let token = siguiente_token(iter);

        match token.tipo {
            TokenTipo::Suma     => {resultado += termino(iter)}
            TokenTipo::Resta    => {resultado -= termino(iter)}
            TokenTipo::Final    => break,
            _ => panic!("Se esperaba suma o resta")
        }
    }

    return resultado;
}

fn termino(iter: &mut Peekable<Chars>) -> i32 {
    
    let mut resultado = siguiente_numero(iter);

    loop {
        let token = siguiente_token(&mut iter.clone()); // Veo el siguiente token sin avanzar

        // Pero si el siguiente token es Division o Multiplicacion lo consumo
        match token.tipo {
            TokenTipo::Division | TokenTipo::Multiplicacion => {siguiente_token(iter); ()},
            _ => (),
        }

        match token.tipo {
            TokenTipo::Multiplicacion   => resultado *= siguiente_numero(iter),
            TokenTipo::Division         => resultado /= siguiente_numero(iter),
            _ => break
        }
    }

    return resultado;
}

fn siguiente_numero(iter: &mut Peekable<Chars>) -> i32 {
    let token = siguiente_token(iter);
    
    match token.tipo {
        TokenTipo::Numero => token.valor.unwrap(),
        _ => panic!("Se esperaba un numero"),
    }
}

fn siguiente_token(iter: &mut Peekable<Chars>) -> Token {

    loop {        
        match iter.next() {
            Some(' ')   => continue,
            None        => return Token {tipo: (TokenTipo::Final), valor: None},
            Some('\n')  => return Token {tipo: (TokenTipo::Final), valor: None},
            Some('+')   => return Token { tipo: (TokenTipo::Suma), valor: None },
            Some('-')   => return Token { tipo: (TokenTipo::Resta), valor: None },
            Some('*')   => return Token { tipo: (TokenTipo::Multiplicacion), valor: None },
            Some('/')   => return Token { tipo: (TokenTipo::Division), valor: None },
            Some(caracter) => {
                let mut acumulador = String::from(caracter);
                loop {    
                    let siguiente_caracter = iter.next();
                    match siguiente_caracter {
                        Some(caracter_numerico) if caracter_numerico.is_numeric() => acumulador.push(caracter_numerico),
                        _ => break,
                    }
                }

                match acumulador.parse::<i32>() {
                    Ok(numero) => return Token {tipo: (TokenTipo::Numero), valor: Some(numero)},
                    Err(_) => panic!("Error, se esperaba numero"),
                }

            }
        }
    }
}
