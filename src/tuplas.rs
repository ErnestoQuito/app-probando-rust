use std::fmt;

// Las tuplas se pueden usar como argumentos de función y como valores de retorno.
fn revertir(pair: (i32, bool)) -> (bool, i32) {
    // `let` se puede utilizar para vincular los miembros de una tupla a variables.
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn transponer(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// La siguiente estructura es para la actividad.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

pub fn tuplas() {
    println!("-------- TUPLAS ---------");
    // Una tupla con varios tipos diferentes.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    // Values can be extracted from the tuple using tuple indexing.
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Las tuplas pueden ser miembros de tuplas.
    let tuplas_de_tuplas = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // Las tuplas son imprimibles.
    println!("tuplas de tuplas: {:?}", tuplas_de_tuplas);
    // Pero las tuplas largas (más de 12 elementos) no se pueden imprimir.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    //TODO ^ Uncomment the above 2 lines to see the compiler error
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", revertir(pair));

    // Para crear tuplas de un elemento, 
    // la coma es necesaria para distinguirlas de un literal rodeado de paréntesis.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Las tuplas pueden desestructurarse para crear enlaces.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix\n{}", matrix);
    println!("Transpose\n{}", transponer(matrix));
}