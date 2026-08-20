fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
	//assert_eq!(5, y); // Error: mismatched types

	/*
	{} Muestra el valor. Funciona para números, texto, valores normales
	{:p} Muestra la dirección de memoria. Solo para referencias, punteros y smart pointers.

	VARIABLE NORMAL (x)
     Código │ Tipo  │ Qué es                          │ Valor
    ────────┼───────┼─────────────────────────────────┼──────────
       x    │ i32   │ El valor                        │ 5
      *x    │ ERROR │ No se puede dereferenciar       │ -
      &x    │ &i32  │ La referencia a x               │ 0x7fff...
	
	VARIABLE REFERENCIA (y = &x)
     Código │ Tipo  │ Qué es                          │ Valor
    ────────┼───────┼─────────────────────────────────┼──────────
       y    │ &i32  │ La referencia misma             │ 0x7fff...
      *y    │ i32   │ El valor dereferenciado         │ 5
      &y    │ &&i32 │ Referencia a la referencia      │ 0x8888...
	*/

	println!("Imprimiendo x: {}", x);
	print!("Imprimiendo x con :p : ");
	println!("No funciona porque x no es una direccion de memoria");
	print!("Imprimiendo *x: ");
	println!("No funciona porque no se puede mostrar el valor de una variable que ya muestra su valor");
	print!("Imprimiendo *x con :p : ");
	println!("No funciona por el principio de *x no importa si se usa :p o no");
	println!("Imprimiendo &x: {}", &x); // Dirección de memoria de x
	println!("Imprimiendo &x con :p : {:p}", &x); // Dirección de memoria de x

	println!("Imprimiendo y: {}", y); // contenido de y, que es la dirección de memoria de x
	println!("Imprimiendo y con :p : {:p}", y); // contenido de y, que es la dirección de memoria de x
	println!("Imprimiendo *y: {}", *y);
	print!("Imprimiendo *y con :p : ");
	println!("No Funciona porque *y es un valor normal, no una dirección de memoria");
	println!("Imprimiendo &y: {}", &y); // Dirección de memoria de y
	println!("Imprimiendo &y con :p : {:p}", &y); // Dirección de memoria de y
}