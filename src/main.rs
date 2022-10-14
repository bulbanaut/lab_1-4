use std::{io::stdin, process::exit};


fn main() {
    println!("Введите значение X, затем Y:");
    let p = [read_var(), read_var()];

    let a = [-4.0, 0.0];
    let b = [0.0, 4.0];
    let c = [4.0, 0.0];
    //перемещаем треугольник точкой а в 0:0
    let a = [a[0] - a[0], a[1] - a[1]];// можно было заменить на let a = [0,0], оставлено для наглядности
    let b = [b[0] - a[0], b[1] - a[1]];
    let c = [c[0] - a[0], c[1] - a[1]];
    let p = [p[0] - a[0], p[1] - a[1]];

    let m: f64 = (p[0]*b[1] - b[0]*p[1]) / (c[0]*b[1] - b[0]*c[1]);//вычисляем мю
    if m >= 0.0 && m <= 1.0 {
        let l: f64 = ((p[0] - m*c[0])/ b[0]).into();
        if l >= 0.0 && m + l <= 1.0 {
            println!("True")
        } else {
            println!("False");
            pause();
            exit(0);
        } 
    }   else {
        println!("False");
        pause();
        exit(0);
    }
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