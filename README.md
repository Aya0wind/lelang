# lelang
leang使用Rust与LLVM(inkwell safe binding library)实现，有：
+ 基本的函数定义，调用，声明外部函数语法
+ 外部链接
+ 变量定义
+ 静态，编译期检查类型
+ C like语法

### TODO
+ 支持块级变量定义
+ 支持函数内的函数声明与函数定义
+ 整合LLVM后端进可执行文件
+ 优化前端生成的代码
+ 优化性能，减少过程中不必要的重复判断与复制
