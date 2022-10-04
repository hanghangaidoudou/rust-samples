fn main() {
    //Rust是一个基于表达式的语句
    let _x = 5 + 6;
    //语句不返回值，不可以用let将一个语句给一个变量 let x= (let y=6); 这么是不支持的
    let y = {
        let x = 1;
        x + 3  //不带分号 就是返回值 ,如果带分号 就是返回一个空的tuple ()
    };
    println!("the value of y is:{}", y);
    let f = five();
    let pf = plus_five(f);
    println!("the value of plus is:{}", pf);
    //控制流
    let condition = true;
    let number = if condition { 5 } else { 6 };//5和6必须是统一类型，如果6换成"6"就不可以
    println!("{}", number);
    //循环系列
    loop_while_sample();
    loop_iter_sample();
    loop_rang_sample();
}
fn plus_five(x:i32)-> i32{ // 用->后面加返回类型
    5+x
}

fn five()-> i32{
    return 5;//如果不用表达式 就加 return
}
//while循环
fn loop_while_sample(){
    let mut number=3;
    while number != 20 {//不等于
        println!("{}:",number);
        number =number +1;
    }
    println!("Lift off!!!")
}
//迭代器循环
fn loop_iter_sample()
{
    let a=[10,20,30,40,50];
    for element in a.iter(){
        println!("the value is:{}",element);
    }
}
//Range 如123，就 1..4，不包含4
fn loop_rang_sample()
{
    //从1到3
    for number in 1..4 {
        println!("the value is:{}",number);
    }
    //从3到1
    for number in (1..4).rev() {
        println!("the value is:{}",number);
    }
}
