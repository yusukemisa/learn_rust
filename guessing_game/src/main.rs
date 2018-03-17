extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
/**
 * 書いてなくてもデフォルトで読み込まれ使えるライブラリがある
 * 「プレリュード」というものstd::string::Stringはそう。
 *
*/
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // new() は関数
        let mut guess = String::new();
        //let mut guess = std::string::String::new();こうも書ける
        // stdinインスタンス作成、read_lineメソッド呼び出し
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // エラー処理

        // 異なるunsigned intで変数を再定義 :{型} で型注釈をつけられる
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

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
