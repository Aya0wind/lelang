# 编写该语言的动机

## 为什么要设计该语言

+ 因为一直都对编译技术比较感兴趣，在学习了一些编译技术后，就想动手实现一个较为完整的编程语言，并考虑作为长期维护项目。
+ 首先考虑实现一个脚本语言，在参考了Lua语言、Python语言、JavaScript语言后，觉得脚本语言虽然在后端实现上较为灵活，但是都会需要运行时环境，例如虚拟机。
+ 刚好可以作为一个课程实验进行

## 为什么设计成静态检查，基于LLVM的语言？

+ 由于是学习性质的项目，使用LLVM可以接触到更多的底层知识和优化手段，并且可以方便的直接编译到独立运行的可执行文件，并可使用LLVM工具链辅助完成调试，优化等工作，并且如果后续
  需要支持解释执行，借用LLVM的JIT，或者在现有的基础上重写一个解释器作为后端也是十分方便的，所以lelang最终定位为了一个偏向进行更多的静态检查，支持直接编译到机器码，并使用可使用
  标准C ABI和系统进行交互的语言。
+ 而现有的编程语言中，比较流行的类似的分别是C、C++、Go、Rust，lelang的设计对这几个语言均有参考。C语言虽然经典好用，但是语言提供的抽象能力太差，也没有提供任何的安全检查机制和垃圾收集机制，开发难度较大。
  C++语言特性很多，功能强大，但是上手难度较高，历史包袱较重，使用难度也较高。Rust语言上手难度同样较高，继承和改进了很多C++的问题，但是编译检查十分严格，并且内容也非常多，且牵涉到许多C系语言中很少使用的概念，并且也需要手动管理内存。
  最后Go语言虽然满足易用，静态检查，上手简单，自动垃圾回收等需求，但是Go语言某些部分的语法设计过于粗略，例如any类型，使用积类型而不是和类型进行错误处理，不支持任何可变性约束等等。
+ 所以lelang基于Go语言的优点，预计在其基础上增加例如代数数据类型，模式匹配，泛型等语法，基于LLVM的后端可方便支持跨各种操作系统和硬件平台的编译，引入自动垃圾收集器，摆脱手动管理内存的麻烦。

## 为什么使用Rust语言编写前端？

+ Rust语言是我个人比较喜欢的一门语言。但是平时C++，Java，Python等语言使用的更多，一直没有一个使用Rust实践一个完整项目的机会，于是考虑使用Rust
+
Rust语言支持多种函数式编程语言的语法和概念，但是又不像某些纯函数式语言（例如Haskell，Lisp语言）那样完全放弃命令式的语法，并且不需要任何解释器环境就可运行，其中的代数数据类型，模式匹配，和基于和类型的错误处理等特性都十分适合用来编写一个新语言的编译器。
+ Rust的工程化和易用性做的很好，不管是开发还是部署，均比C++要容易，并且严格的编译检查和方便的日志功能能很好的减少Debug的时间。