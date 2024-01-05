use std::mem;

// Esta función toma prestada una rebanada.
fn analizar_slice(slice: &[i32]) {
    println!("primer elemento de la rebanada: {}", slice[0]);
    println!("la rebanada tiene {} elementos", slice.len());
}

pub fn array_slice() {
    println!("-------- ARRAY Y SLICE ---------");
    // Matriz de tamaño fijo (la firma de tipo es superflua).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // Todos los elementos pueden inicializarse con el mismo valor.
    let ys: [i32; 500] = [0; 500];

    // La indexación comienza en 0.
    println!("primer elemento del array: {}", xs[0]);
    println!("segundo elemento del array: {}", xs[1]);
    // `len` devuelve la longitud de elementos de la matriz.
    println!("tamaño del array: {}", xs.len());
    // Las matrices se asignan por pila.
    println!("El array ocupa {} bytes", mem::size_of_val(&xs));

    // Los Arrys pueden tomarse prestadas automáticamente como silces.
    println!("Toma prestada todo el Arrya como un silce.");
    analizar_slice(&xs);

    // Los Slices pueden apuntar a una sección de un Array.
    // Tienen la forma [índice_inicial..índice_final].
    // `starting_index` es la primera posición en la rebanada.
    // `ending_index` es uno más que la última posición de la rebanada.
    println!("Toma prestado una sección del array como un slice.");
    analizar_slice(&ys[1 .. 4]);

    // Ejemplo de un Slice vacío `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Igual pero más verboso

    // Se puede acceder a los arrays de forma segura usando `.get`, que devuelve una 
    // `Option`. Esto puede ser emparejado como se muestra a continuación, o utilizado con
    // `.expect()` si desea que el programa salga con un 
    // mensaje agradable en lugar de continuar felizmente.
    for i in 0..xs.len() + 1 { // ¡Uy, un elemento de más!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("¡Más despacio! {} está demasiado lejos", i),
        }
    }

    // La indexación fuera de límites de un array provoca un error de compilación.
    // println!("{}", xs[5]);
    // La indexación fuera de los límites del Slice provoca un error en tiempo de ejecución.
    // println!("{}", xs[..][5]);
}