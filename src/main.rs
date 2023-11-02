//Подключаем либу, которая позволяет видеть пользовательский ввод
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //Макросы
    println!("Guess the number!");

    /*
        Сначала создаём объект генератора случайных чисел
        Потом вызываем метод генерации случайного числа с
        указаным диапазоном
    */

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        //Создаём переменную для хранения пользовательского ввода (изменяемая)
        //String::new() возвращает объект String для приёма строки
        //:: в синтаксисе перед new() указывает на то, что это ассоциированая функция
        /* 
            Ассоциированная функция - то функция, реализованная для типа данных 
            (В данном случае String)
        */
        let mut guess = String::new();
    
        /* 
            Далее обрабатываем пользовательский ввод с помощью 
            библиотеки std::io.
    
            Символ "&" указывает на то, что переменная guees является ссылкой
            на другую переменную
        */
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        /* 
            Происходит затенение переменной guess.
            Метод trim на экземпляре String удалит любые пробельные символы в начале и конце строки для того, 
            чтобы мы могли сопоставить строку с u32, 
            который содержит только числовые данные.
    
            Метод parse строк преобразует строку в другой тип
        
        */

        /* 
            Обрабатываем ошибку.
            Подчёркивание _ является всеохватывающим выражением. 
            В этой ветке мы говорим, что хотим обработать совпадение всех значений Err, 
            независимо от того, какая информация находится внутри.

        */
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //.expect("Please type a number!");
        
        println!("You guessed: {guess}");
    
        /* 
            Ordering позволяет сравнивать значения
        
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  
            } 
        }
    }
}


// fn main2() {
//     let x = 5;
//     let y = 10;

//     println!("x = {x} and y + 2 = {}", y + 2);
// }