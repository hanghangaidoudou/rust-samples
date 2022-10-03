fn main() {
    let guess: u32 = "42".parse().expect("Not a number");// parse 知道 转成u32类型,如果去掉u32就会报错
    println!("{}", guess);

    /*  整数的字面值 类型可以加后缀，比57u8 30i32
    Number literals | Example  | Number  | Describe
    Decimal           98_222     98222     里面可以加下划线 增加可读性
    Hex               oxff                 十六进制
    Octal             0o77                 八进制
    Binary            0b1111_0000          二进制
    Byte(u8 only)     b'A'                 比特
     */
     let a=6666_6666;//下划线随意放

     let b=55555_555;//下划线随意放
    let c=a-b;
    println!("diff:{}",c);
    /*浮点*/
    //f32, 32位 单精度
    //f64, 64位 双精度
    let y: f32 = 3.0;
    let x = 2.0;//默认f64
    //推断i32
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 54 % 5;
}
