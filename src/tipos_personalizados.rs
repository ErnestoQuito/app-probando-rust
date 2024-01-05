// Un atributo para ocultar las advertencias de código no utilizado.
#![allow(dead_code)]

#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u8,
}

// Una estructura unitaria.
struct Unidad;

// Una 'tuple struct'.
struct Par(i32, f32);

// Una struc con dos campos.
#[derive(Debug)]
struct Punto {
    x: f32,
    y: f32,
}

// Los struct pueden reutilizarse como campos de otro struct.
#[derive(Debug)]
struct Rectangulo {
    // Un rectángulo se puede especificar por dónde están en el espacio
    // las esquinas superior izquierda e inferior derecha.
    top_left: Punto,
    bottom_right: Punto,
}

fn rect_area(rect: Rectangulo) -> f32 {
    let Rectangulo { top_left: Punto { x: x1, y: y1 }, bottom_right: Punto { x: x2, y: y2 } } = rect;
    (x2 - x1) * (y2 - y1)
}

pub fn tipos_personalizados() {
    print!("-------- TIPOS PERSONALIZADOS ---------\n");
    print!("-------- STRUCTS ---------\n");
    // Crear estructura con la abreviatura init de campo.
    let nombre = String::from("Peter");
    let edad = 27;
    let peter = Persona { nombre, edad};
    // Imprimir debug struct.
    println!("{:?}", peter);

    // Instanciar un `Punto`.
    let punto: Punto = Punto { x: 10.3, y: 0.4 };

    // Acceder a los campos del Punto.
    println!("Coordenadas del punto: ({}, {})", punto.x, punto.y);

    // Crea un nuevo Punto utilizando la sintaxis struct update
    // para utilizar los campos de nuestro otro punto
    let bottom_right = Punto { x: 5.2, ..punto };
    // `bottom_right.y` será el mismo que `point.y` porque usamos ese campo de `punto`.
    println!("Segundo punto: ({}, {})", bottom_right.x, bottom_right.y);

    // Desestructurar el punto utilizando un enlace `let.
    let Punto { x: left_edge, y: top_edge } = punto;

    let rectangulo = Rectangulo {
        // La instanciación de struct también es una expresión.
        top_left: Punto { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("Rectangulo: {:?}", rectangulo);
    println!("Área del Rectangulo: {}", rect_area(rectangulo));

    // Instanciar una estructura de Unidad.
    let _unidad = Unidad;

    // Instanciar una tuple struct.
    let par = Par(1, 0.1);

    // Acceder a los campos de una tuple struct.
    println!("Par contiene {:?} y {:?}", par.0, par.1);

    // Desestructurar una tuple struct.
    let Par( integer, decimal ) = par;
    println!("Par contiene {} y {}", integer, decimal);
}    