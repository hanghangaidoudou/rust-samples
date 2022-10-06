use String;
fn main() {
    //整型的深拷贝与浅拷贝
    let x=5;
    let y=x;
    println!("x  value:{}",x);

    println!("y  value:{}",y);
    //字符串的深度拷贝与浅拷贝，如果字符串发生了赋值，就相当于发生了指针转移，老的变量就不存在了

    let s1=String::from("Hello, world!");
    let s2=s1.clone();
    let s3=s1;//move 到了s3
    //deep copy
    println!("深拷贝 value:{}",s2);
    //shallow copy
    //println!("浅拷贝 value:{}",s1);//由于 s1在赋值给s2的时候就move了 所以打开注释会报错在此

    /**实现copy 的trait 就不会释放旧的变量
    拥有Copy trait的类型
    任何简单标量的组合类型都可以是Copy的
    任何需要分配内存或某种资源的都不是Copy的
    具体类型:
    所有整数类型,例如u32
    bool
    char
    所有浮点类型，例如 f64
    Tuple(元组)，如果其所有的字段都是Copy的
    [i32,i32] 就是copy的
    [i32,String] 就不是copy的
    */

    let s=String::from("Hello, world!");
    take_ownership(s);
   // println!("s:{}",s); //由于take_ownership 会释放内存，所以这个注释打开，就用不了了
    let x5=5;
    makes_copy(x5);
    println!("x:{}",x5);
    /** 返回值与作用域
    */
    let g1=gives_ownership();
    let g2=String::from("hello2");
    let g3=takes_and_give_back(g2);//g2就被移动到 takes_and_give_back里，在之后使用g2就会报错
    println!("{},{}",g1,g3);
    let (g4,len)=calculate_length(g3);//如果想把所有权交出去在要回来，并且usize 本身就是整型，被copy出来
    println!("the length of {} is {}",g4,len);

   let g5= calculate_ref_length(&g1);//引用
    println!("borrow:{}",g5);

    let mut g6= String::from("hello");

    let  g7= calculate_ref_mut_length(&mut g6);//可变引用
    println!("borrow:{}",g7);


    /**数据竞争
    1、两个或多个指针同时访问一个数据的时候
    2、至少有一个指针用于写入数据
    3、没有任何机制来同步数据的访问
     */
    let mut mug6= String::from("hello");//可变引用的个数显示 (数据竞争)
    let g8=&mut mug6;
    //let g9=&mut mug6;//可变的引用不可以同时超过一个 打开注释编译就会出现问题
    //println!("mut borrow:{},{}",g8,g9);//可变的引用不可以同时超过一个 打开注释编译就会出现问题 ，防止数据竞争，多个指针访问同一个数据
    /**多个作用域
        1、可以解决数据竞争，非同时的数据访问
     */

    let mut mug7= String::from("hello");
    {
        let s3=&mut mug7;
        println!("非同时的 mut 数据获取1:{}",s3);
    }
    let s4=&mut mug7;
    println!("非同时的 mut 数据获取2:{}",s4);

    mutable_and_immutable();//不可同时拥有一个可变引用和一个不可变引用

    /**切片 对部分内容的引用 不持有所有权的类型(slice)

     */
    let mut mug8= String::from("hello world");

   let world_index= first_word(&mug8);
    mug8.clear();
    println!("index :{}",world_index);//这个时间上mug8已经释放了，但是world_index还是不变，所以为了解决这个问题，引入了切片 slice
    let mut mug9= String::from("hello world");
    let hello=&mug9[0..world_index];//语法唐1，&mug9[..world_index]
    let world =&mug9[6..11];//语法唐2，&mug9[6..]
    let whole=&mug9[..];//语法唐3
    println!("1:{},2:{},3{}",hello,world,whole);
    //字符串的切片索引 ，必须发生在有效的utf-8字符边界内
    let mut mug10= String::from("hello world");
    let slicestr=first_word_slice(&mug10);

   // mug10.clear();//打开既见错误 immutable borrow occurs here由于 first_word_slice里对可变引用 as_bytes变成不可变引用，所以后续的引用就使用不了
    println!("返回第一个字符串单词 1:{}",slicestr);

    //字符串字面值 既切片
    let g11="hello world";//既切片 不可变
    println!("{}",g11);
    // 有经验的开发者会采用&str作为参数类型，因为这样做可以同时接收String和&str类型参数
     let p_str=  first_word_slice_by_str(g11);
    let   g12= String::from("hello world");
    let p_string=  first_word_slice_by_str(&g12[..]);
    println!("1:{},2:{}",p_str,p_string);
    //其他类型的切片
    let arr=[1,2,3,4,5,6,7,8];
    let slice=&arr[1..3];

    println!("1:{},2:{}",slice[0],slice[1]);
}

fn take_ownership(some_string:String){
    println!("{}",some_string)
}// drop 释放了some_string
fn makes_copy(some_number:i32){
    println!("{}",some_number)
}//整型 不会释放some_number

fn gives_ownership()->String{//返回值的例子
    let some_string=String::from("hello1");
    some_string
}
fn takes_and_give_back(a_string:String)->String{//返回值的例子
    a_string
}
fn calculate_length(s:String)->(String,usize){//所有权原封不动返回去
    let length=s.len();
    (s,length)
}

fn calculate_ref_length(s:&String)->usize{//引用 s的所有权是调用方的，没有获得
    //s.push_str(",world");// 借用的变量是不可变的
    s.len()
}

fn calculate_ref_mut_length(s:&mut String)->usize{//可变引用
    s.push_str(",world");//
    s.len()
}
fn mutable_and_immutable(){
   let mut s=String::from("hello");
    let r1=&s;
    let r2=&s;
    // let s1=&mut s;//不可同时拥有一个可变引用和一个不可变引用 打开注释即可看到
    // println!("{},{},{}",r1,r2,s1);//不可同时拥有一个可变引用和一个不可变引用 打开注释即可看到
}
// fn dangling_ref() -> &String{//悬垂引用(一个指针引用来内存某个地址，而这一块内存已经释放被释放并分配给其他人使用了)
//    let s=String::from("hello");
//     &s
// }//在出作用域的时候 s 被释放了，&s被返回了，就成了悬垂引用，会报错 expected named lifetime parameter


fn first_word(s:&String)->usize{//切片 例子 找到一个字符串数组里第一个单词
    let bytes=s.as_bytes();//字节数组
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
         return i;
        }
    }
    s.len()
}

fn first_word_slice(s:&String)->&str{//切片 例子 找到一个字符串数组里第一个单词
    let bytes=s.as_bytes();//字节数组
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_slice_by_str(s:&str)->&str{//使用&str 更加通用
    let bytes=s.as_bytes();//字节数组
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}