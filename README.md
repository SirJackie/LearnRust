# Learn Rust

## 安装

1. 需要先安装VS2015或者更高版本，否则会报错

2. 下载并运行rustup-init.exe

   官网默认下载的是在线安装器，从这里获得离线安装包：https://forge.rust-lang.org/infra/other-installation-methods.html

   选择的机型：x86_64-pc-windows-msvc

   即可得到离线版：rust-1.66.1-x86_64-pc-windows-msvc.msi

3. 更新和卸载

   更新：`rustup update`

   卸载：`rustup self uninstall`，（我安装的是离线版）这个命令卸载不干净，还是建议去控制面板卸载。

   如果安装的是离线版，则没有rustup命令

4. 查看Path目录

   有一个新目录进入了Path：`C:\Program Files\Rust stable MSVC 1.66\bin`

   这个目录里面有三个exe，对应三个命令：

   ```
   rustc.exe
   rustdoc.exe
   cargo.exe
   ```

   ```
   C:\Users\Windows To Go>rustc --version
   rustc 1.66.1 (90743e729 2023-01-10)
   
   C:\Users\Windows To Go>rustdoc --version
   rustdoc 1.66.1 (90743e729 2023-01-10)
   
   C:\Users\Windows To Go>cargo --version
   cargo 1.66.1 (ad779e08b 2023-01-10)
   ```

## 命令行

### 单文件编译

```
rustc --version
rustup doc（如果安装的是离线版，则没有rustup命令）
```

编译：`rustc main.rs`

### 使用Cargo创建项目

创建项目：`cargo new hello_cargo`

检查语法：`cargo check`（比`cargo build`要快很多）

### 编译Debug

编译项目：`cargo build`

编译 + 执行项目：`cargo run`

### 编译Release

编译正式版项目：`cargo build --release`

编译 + 执行正式版项目：`cargo run --release`

## 语法

### 输出字符串

```rust
fn main() {
    println!("Your String.");
}
```

主函数：`fn main() {`

打印宏：`println!("Your String.");`

注意！`println!(a);`不行，`println!("{}", a);`可以。

### 输入字符串

```rust
// 文件开头
use std::io;
// 主函数内
let mut guess = String::new();
io::stdin().read_line(&mut guess).expect("Cannot Read Line.");
```

### 输出含参字符串

```rust
println!("The number you guessed is: {}", guess);
```

### 创建可变 or 不可变变量

```rust
// 不可变变量
let a = 1;
let b = a;
let c = "String";
let d = c;

// 可变变量
let mut a = 1;
let mut b = a;
let mut c = "String";
let mut d = c;
```

### 生成随机数

```toml
[dependencies]
rand = "0.7.3"
```

```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1, 101);  // 左闭右开
println!("Secret Number is: {}", secret_number);
```



## 库：Crates

### Crates

Cargo的库叫做Crate，你可以在官网上找到任何一个包：https://crates.io/

比如：https://crates.io/crates/rand

还可以具体到特定版本，比如：https://crates.io/crates/rand/0.7.3

### 添加方法

在Cargo.toml文件中的dependencies下面添加一行：

```toml
[dependencies]
rand = "0.7.3"
```

只需要Build一下，Cargo就会自动下载所有依赖。

### 第一次下载依赖：会先下载索引

```
D:\Projects\Rust\guessing_game>cargo build
    Updating crates.io index
       Fetch [==>                      ]  12.79%, 1.14MiB/s
```

如果你想要的更新索引，可以：`cargo update`

### Cargo的编译可重现机制：使用Cargo.lock确保依赖版本不更变

Cargo.toml：只储存了你需要的Crates的具体版本号

Cargo.lock：不仅储存你需要的Crates，还储存这些Crates的依赖Crates，确保依赖链正确。