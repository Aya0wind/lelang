# 函数

和许多其他系统级编程语言一样，函数是lelang运行程序的基本单位，一个lelang函数由四部分组成。

+ 函数声明关键字
+ 函数名字
+ 参数列表
+ 返回值

```lldb

## le -->function define
## fibonacci -->function name 
## step -->parameter name
## i32 -->parameter type
## -->i32 -->return type
## {...} -->function body
le fibonacci(step:i32)->i32{   
    if(step<3){                
        ret 1;
    }el{
        ret fibonacci(step-1)+fibonacci(step-2);
    }
}
```


