# 整数与浮点数

与C或C++不同，lelang要求显示指定整数与浮点数的长度，lelang保证在任何实现lelang编译器的平台上均为同样的长度。

#### 整数共有8种类型

+ i8~i32
+ u8~u32

#### 浮点数共有2种类型

+ f32
+ f64

Tips:
lelang禁止不同类型的整数或浮点数发生隐式转换，例如下面的代码无法通过编译

```lldb
{{#include ../test_sources/invalid_conversion.le}}
```

编译它，我们得到👉🏻

```lldb
{{#include ../test_sources/invalid_conversion.error}}
```

**同样**，不同类型的整数或浮点数也不能直接相加

```lldb
{{#include ../test_sources/invalid_conversion1.le}}
```

编译它，我们得到👉🏻

```lldb
{{#include ../test_sources/invalid_conversion1.error}}
```