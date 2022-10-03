fn main() {
    println!("同一个变量名--------和mut不同，它是取最后一个被赋值的内容");
    let x=5;
    println!("shadow layer 1 {}",x);
    let x=x+1;
    println!("shadow layer 2 {}",x);
    let x=x*2;
    println!("shadow layer 3 {}",x);
    println!("变量名相同，类型可以不同--------可以不用做成类似space_str和space_usize这样两个变量名去声明");
    let space ="    ";
    println!(" {}",space);
    let space=space.len();
    println!(" {}",space);
}
