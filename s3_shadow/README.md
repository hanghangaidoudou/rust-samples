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

shadow覆盖:



    同一个变量名可以设置多次,和mut不同，它是取最后一个被赋值的内容,变量名相同，类型可以不同,可以不用做成类似space_str和space_usize这样两个变量名去声明

