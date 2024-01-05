pub fn literal_operators() {
    println!("-------- LITERAL OPERATORS ---------");
    // Suma de enteros.
    println!("1 + 2 = {}", 1u32 + 2);
    // Resta de enteros.
    println!("1 - 2 = {}", 1i32 - 2);
    // Notación cientifica.
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    // Logica boolena de cortocircuito.
    println!("true AND false es {}", true && false);
    println!("true OR false es {}", true || false);
    println!("NOT true es {}", !true);
    // Operaciones en bits.
    println!("Operaciones en bits");
    println!("0011 AND 0101 es {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 es {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 se {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 es {}", 1u32 << 5);
    println!("0x80 >> 2 es 0x{:x}", 0x80u32 >> 2);
    println!("¡Usa guiones bajos para mejorar la legibilidad!");
    println!("One million is written as {}", 1_000_000u32);
}