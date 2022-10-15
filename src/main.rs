use std::io::stdin;

fn main() {
    println!("Введите координаты точки (X, затем Y):");
    let p: [f64; 2] = [read_var(), read_var()];

    let rp: f64 = (p[0].powi(2) + p[1].powi(2)).sqrt();

    if rp >= 3.0 && rp <= 5.0 && p[0] >= 0.0 {
        println!("True");
    } else {
        println!("False");
    }
    pause();
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}

fn pause() { //фукция паузы
    println!("нажмите Enter чтобы выйти.");
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
}