use std::{
    fmt,
    io::{self, Write},
    iter::Peekable,
    str::Chars,
};

enum Token {
    Numero(i32),
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Final,
}

impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Token::Numero(numero) => write!(f, "{}", numero),
            Token::Suma => write!(f, "Suma"),
            Token::Resta => write!(f, "Resta"),
            Token::Multiplicacion => write!(f, "Multiplicacion"),
            Token::Division => write!(f, "Division"),
            Token::Final => write!(f, "Final"),
        }
    }
}

fn main() {
    let mut input = String::new();

    print!("Calcular: ");
    io::stdout().flush().expect("No se pudo leer su input");

    io::stdin()
        .read_line(&mut input)
        .expect("No se pudo leer su input");

    let mut iter: Peekable<Chars> = input.chars().peekable();

    parse(&mut iter);
}

fn parse(iter: &mut Peekable<Chars>) {
    let resultado = expresion(iter);

    println!("Resultado: {resultado}");
}

fn expresion(iter: &mut Peekable<Chars>) -> i32 {
    let mut resultado = termino(iter);

    loop {
        match siguiente_token(iter) {
            Token::Suma => resultado += termino(iter),
            Token::Resta => resultado -= termino(iter),
            Token::Final => break,
            token => panic!("Se esperaba Suma, Resta o Final pero se obtuvo {}", token),
        }
    }

    return resultado;
}

fn termino(iter: &mut Peekable<Chars>) -> i32 {
    let mut resultado = siguiente_numero(iter);

    loop {
        let token = siguiente_token(&mut iter.clone()); // Veo el siguiente token sin avanzar

        // Si el siguiente token es Division o Multiplicacion, lo consumo (o sea muto iter)
        match token {
            Token::Division | Token::Multiplicacion => {
                siguiente_token(iter);
            }
            _ => break,
        }

        match token {
            Token::Multiplicacion => resultado *= siguiente_numero(iter),
            Token::Division => resultado /= siguiente_numero(iter),
            token => panic!(
                "Se esperaba Multiplicacion o Division pero se obtuvo {}",
                token
            ),
        }
    }

    return resultado;
}

fn siguiente_numero(iter: &mut Peekable<Chars>) -> i32 {
    let token = siguiente_token(iter);

    match token {
        Token::Numero(numero) => numero,
        _ => panic!("Se esperaba Numero, se obtuvo {token}"),
    }
}

fn siguiente_token(iter: &mut Peekable<Chars>) -> Token {
    loop {
        match iter.next() {
            Some(' ') => continue,
            None => return Token::Final,
            Some('\n') => return Token::Final,
            Some('+') => return Token::Suma,
            Some('-') => return Token::Resta,
            Some('*') => return Token::Multiplicacion,
            Some('/') => return Token::Division,
            Some(caracter) => {
                let mut acumulador = String::from(caracter);
                loop {
                    let siguiente_caracter = iter.clone().next();
                    match siguiente_caracter {
                        Some(caracter_numerico) if caracter_numerico.is_numeric() => {
                            iter.next();
                            acumulador.push(caracter_numerico)
                        }
                        _ => break,
                    }
                }

                match acumulador.parse::<i32>() {
                    Ok(numero) => return Token::Numero(numero),
                    Err(_) => panic!("Error no se pudo parsear [{acumulador}] como numero"),
                }
            }
        }
    }
}
