/* Using a hash map and vectors, create a text interface to allow a user to add employee names
to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
Then let the user retrieve a list of all people in a department or all people in the company by 
department, sorted alphabetically. */

use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

fn main() {
	let mut company: HashMap<String, Vec<String>> = HashMap::new();

	loop {
		//clear_terminal_screen();
		println!("\nOptions:");
		println!("1. Add employee to department");
		println!("2. List employees in a department");
		println!("3. List all employees by department");
		println!("4. Exit");
		print!("Enter your choice: ");
		let _ = io::stdout().flush();

		let mut input_text = String::new();
		
		io::stdin()
			.read_line(&mut input_text)	// actually read the line
			.expect("Failed to read line from stdin.");	// which can fail, however

		let choice: i32 = match input_text
			.trim()	// ignore whitespace around input and remove line jump \n
			.parse()	// convert to integers
			//.expect("Input not an integer");	// which, again, can fail
			{
				Ok(num) => num,
				Err(_) => {
					println!("Invalid option, please enter a number.");
					continue;
				}
			};

		match choice {
			1 => add_employee(&mut company),
			2 => list_department(&company),
			3 => list_all(&company),
			4 => {
				println!("\nGoodbye!\n");
				break;
			}
			_ => println!("Invalid option, select one of above."),
		}
	}
}

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
	let mut text = String::new();
	let name: String;
	let department: String;
	let mut words: Vec<&str>;

	print!(">> ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut text).expect("Failed to read line from stdin.");

	// Divide el texto en palabras
	words = text.split_whitespace().collect();

	while words.len() != 4 || words[0] != "Add" || words[2] != "to" {
		if words.len() != 4 {
			println!("Error: The text does not contain 4 words.");
			println!("{:?}", text);
		}
		else if words[0] != "Add" {
			println!("Error: First bad argument. Type \"Add\"");
			println!("{:?}", text);
		}
		else if words[2] != "to" {
			println!("Error: Third bad argument. Type \"to\"");
			println!("{:?}", text);
		}

		// I have to create a new string because stdin adds content and does not put a new one
		text = String::new();
		print!(">> ");
		let _ = io::stdout().flush();
		io::stdin().read_line(&mut text).expect("Failed to read line from stdin.");
		words = text.split_whitespace().collect();
	}

	name = words[1].to_string();
	department = words[3].to_string();

	company.entry(department.clone()).or_insert_with(Vec::new).push(name.clone());
	println!("Employee {} added to {} department!", name, department);
}

fn list_department(company: &HashMap<String, Vec<String>>) {
	// Verificar si el HashMap está vacío
    if company.is_empty() {
        println!("No departments available.");
        return; // Salir de la función y regresar a main
    }

    // Crear un vector de departamentos, ordenado alfabéticamente
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();

    // Imprimir los departamentos enumerados
    for (i, department) in departments.iter().enumerate() {
        println!("{}. {}", i + 1, department);
    }

    // Leer la elección del usuario
    let mut input = String::new();
    let choice: usize;

    loop {
        print!("Enter department number: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        // Intentar convertir la entrada a un número
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= departments.len() => {
                choice = num;
                break;
            }
            _ => {
                println!("Invalid option, please enter a valid number.");
                input.clear(); // Reiniciar el input para leer nuevamente
            }
        }
    }

    // Obtener el departamento seleccionado
    let department_name = departments[choice - 1];
    let employees = &company[department_name];

    // Imprimir el contenido del departamento
    println!("Department: {}", department_name);
    println!("Employees:");
    for employee in employees {
        println!("- {}", employee);
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
	// Verificar si el HashMap está vacío
    if company.is_empty() {
        println!("No departments available.");
        return; // Salir de la función y regresar a main
    }

	let mut all_departments: Vec<_> = company.keys().collect();
	all_departments.sort();

	for department in all_departments {
		println!("\n{} department:", department);
		let mut employees = company[department].clone();
		employees.sort();
		for employee in employees {
			println!("- {}", employee);
		}
	}
}

// Clear (wipe) the terminal screen
pub fn clear_terminal_screen() {
	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(&["/C", "cls"])
			.status()
			.expect("Failed to clear screen");
	} else {
		Command::new("clear")
			.status()
			.expect("Failed to clear screen");
	}
}