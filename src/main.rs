use std::process;

// Основна функція, яка ніколи не повертає значення
fn never_return() -> ! {
    process::exit(0); // Завершує програму
}

// Тестовий модуль
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_never_return() {
        never_return();
        println!("Failed!");
    }
}

fn main() {
    never_return();
    println!("Failed!");
}
