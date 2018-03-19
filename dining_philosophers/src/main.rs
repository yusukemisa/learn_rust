use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String
}
/**
 * strは呼ばれた時にサイズが決まる（サイズ不定型） 
 * Rustではサイズ不定型には制約を持っており、
 * 1. サイズ不定型はポインタを通してのみ操作可能
 * 2. 変数や引数は動的なサイズを持てない
 * 3. structの最後のフィールドのみ動的なサイズを持つことが許される（？）
 * 　　Enumのバリアントはデータとして動的なサイズを持つことができない
 */
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string()
        }
    }
    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let (a,b) = ("aaa","bbb"); // &str
    // str同士は固定サイズなので連結できないのでStringに変換必要
    let c = a.to_owned() + b;
    println!("{}",c);
    
    // vec 
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}