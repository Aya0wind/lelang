# lelang
leang是一门使用Rust编写，基于LLVM(inkwell llvm safe binding library)实现的编程语言，起初作为课程实验项目，现在为个人长期维护项目。

### Target Features

+ 支持8至64位的整形类型和32/64位浮点
+ 基本的函数定义，调用，声明外部函数语法
+ 链接外部函数
+ 可解释或编译执行
+ 变量定义
+ 静态类型，编译期检查类型
+ C like语法

### TODO

+ ✅支持块级变量定义
+ ✅支持函数内的函数声明与函数定义嵌套
+ ✅整合LLVM后端，可跨平台使用和生成可执行文件
+ ✅优化性能，减少过程中不必要的重复判断与复制
+ ✅支持分支嵌套，循环嵌套，减少过程中不必要的重复判断与复制
+ ❌提供命令行交互式解释执行环境
+ ❌支持内置的数组类型和结构类型
+ ❌支持引用类型
+ ❌支持匿名函数
+ ❌计划引入GC

### Build

1. 安装LLVM 12

+ homebrew

> brew install llvm@12

+ binary  
  https://github.com/llvm/llvm-project/releases/tag/llvmorg-12.0.0

2. 安装rust

+ unix-like  
  ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
+ windows  
  下载[rust-init](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

3. 将llvm bin文件夹加入环境变量
4. 编译
   ```cargo build --release```
5. 编译lelang源文件，帮助可运行```lelang --help```查看（示例见main.le)

```
lelang programming language compiler, based on LLVM compiler infrastructure

USAGE:
    lelang [OPTIONS] -i <SOURCE_FILE_PATH>

OPTIONS:
    -h, --help                   Print help information
    -i <SOURCE_FILE_PATH>        Set compiler source file path
    -o <OUTPUT_FILE_PATH>        Set compiler output path [default: ./a.out]
    -O <OPTIMIZE_LEVEL>          Set compiler optimize level [default: 0]
    -S <OUTPUT_FORMAT>           Set compiler output format [default: obj] [possible values: ir,
                                 asm, obj]
```