use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

//trait 就是其他语言的接口
fn main() {
    println!("猜数游戏");
    let secret_number = thread_rng().gen_range(1, 101);
    println!("神秘数字是{}", secret_number);//真正发版的时候，这一行删除
    loop {

        println!("猜一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("等于 赢得胜利");
                break;
            }

        }
    }
}
