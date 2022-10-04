fn main() {
    let guess: u32 = "42".parse().expect("Not a number");// parse 知道 转成u32类型,如果去掉u32就会报错
    println!("{}", guess);
    /** isize usize 平常使用不多，主要用于索引操作
    */
    let is=66isize;
    let us=77usize;
    /**  整数的字面值 类型可以加后缀，比57u8 30i32
    Number literals | Example  | Number  | Describe
    Decimal           98_222     98222     里面可以加下划线 增加可读性
    Hex               oxff                 十六进制
    Octal             0o77                 八进制
    Binary            0b1111_0000          二进制
    Byte(u8 only)     b'A'                 比特
     */

    let a = 6666_6666;//下划线随意放
    let b = 55555_555;//下划线随意放
    let c = a - b;
    println!("diff:{}", c);

    /** u8类型主要是0～255
          如果256的值
          调试模式，会溢出报错
          Release模式，就会变成0（环绕操作 比如 256=0 257=1 258 =2）
     */
    /**  浮点:IEE754
     */
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
    /** 布尔
      */
    let t=true;//自动推断
    let f:bool=false;//手动类型
    /** 字符类型
    char 描述语言最基础的单个字符
    Unicode标量值可以是拼音，中日韩文，表情包等
    */
    let x='y';
    let y:char ='µ';
    let  z='😀';
    /**符合类型
    Tuple
     */
    let tup:(i32,f64,u8)=(500,6.4,1);
    println!("{},{},{}",tup.0,tup.1,tup.2);//.标记法 索引 0 1 2
    //解构 模式匹配
    let (x,y,z)= tup;

    println!("{},{},{}",x,y,z);
    /**符合类型
        数组
     */
    let a=[1,2,3,4,5];
    //存放在栈内存，而不是放在堆内存上，可以使用数组，或者使用固定长度的集合，可以使用数组
    //Vector长度 是可变的

    //数组类型
    let b:[i32;9]=[1,2,3,4,5,6,7,8,9];
    let c=[3;5];//它相当于 let c=[3,3,3,3,3];
    //访问 stack上
    let first=b[0];
    let second=c[3];
    // 索引编译时候会通过，运行时报错
    let index=[12,13,14,15];
    let third=c[index[1]];
    println!("third {}",third);
}
