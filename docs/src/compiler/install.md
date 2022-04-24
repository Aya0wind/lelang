# 安装与使用

### 1. 编译lelang编译器

+ Linux
    1. Linux下编译lelang compiler需要安装llvm-12和libclang  
       可使用 debian系可使用 apt 进行安装（目前仅在ubuntu下测试），运行即可:
  ```shell
    apt install llvm-12 libclang
  ```
    2. 安装Rust
  ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
    3. 编译项目
  ```shell
   cargo build --release
  ```
    4. 运行项目
  ```shell
  cargo run --release
  ```
+ macos
    1. 使用homebrew安装```llvm12```：
  ```shell
     llvm@12
  ```
    2. 安装Rust
  ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
    3. homebrew默认不会把llvm加入环境变量，需要设置```LLVM_SYS_PREFIX120```环境变量为llvm目录。
    4. 编译项目
   ```shell
    cargo build --release
   ```
    5. 运行项目
   ```shell
   cargo run --release
   ```
+ windows
    1. 编译```llvm12```  
       由于llvm官方在windows下发布的预编译二进制包不含某些需要的静态库，以及llvm-config工具，所以需要自己编译（后续本项目会发布windows下的预编译编译器）
    2. 安装Rust  
       使用[rust-init](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)安装
    3. 设置```LLVM_SYS_PREFIX120```环境变量为llvm目录
    4. 编译项目
  ```shell
   cargo build --release
  ```
  5运行项目
  ```shell
  cargo run --release
  ```

### 使用

运行 ```cargo build --release```后，可在```target/release/```下找到lelang可执行文件，运行```./lelang --help```可打印以下帮助信息

```shell
lelang 
lelang programming language compiler, based on LLVM compiler infrastructure

USAGE:
    lelang [OPTIONS] -i <SOURCE_FILE_PATH>

OPTIONS:
    -h, --help                   Print help information
    -i <SOURCE_FILE_PATH>        Set compiler source file path
    -o <OUTPUT_FILE_PATH>        Set compiler output path [default: ./a.out]
    -O <OPTIMIZE_LEVEL>          Set compiler optimize level [default: 0]
    -S <OUTPUT_FORMAT>           Set compiler output format [default: obj] [possible values: ir,
                                 asm, obj, exe]
```