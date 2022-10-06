# Rust Samples 
The Rust Programming Language

# Ref
语言入门
lang=cn
https://kaisery.github.io/trpl-zh-cn/

lang=en
https://doc.rust-lang.org/stable/book/

类库
https://docs.rs/
## 安装:
> https://www.rust-lang.org
> 
> Linux or Mac build :
> 
> curl https//sh.rustup.rs -sSf | sh

 更新rust
> rustup update

删除rust
> rustup self uninstall

安装验证
> rustc -version

本地文档
> rustup doc


## Sample

所有权:

    每个值都有一个变量，这个变量是该值的所有者
    每个值同时只能有一个所有者
    当所有者超出作用域(Scope)，该值将被删除
   
    实现copy 的trait 就不会释放旧的变量
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

    一个变量的所有权总是遵循同样的模式：
    把一个值赋给其他变量时就会发生移动
    当一个包含heap数据的变量离开作用域时，
    它的值就会被drop函数清理，除非数据的
    所有权移动到另一个变量上了