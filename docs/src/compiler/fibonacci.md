# 打印斐波那契数

1. 新建文件main.le

```
{{#include ../test_sources/fibonacci.le}}
```

2. 执行```lelang -i main.le -S ir -o out.ll```得到

```lldb
{{#include ../test_sources/fibonacci.ll}}
```

3. 执行```lelang -i main.le -S exe -o out```，并运行out，程序打印

```lldb
6765
```

```editable
{{#include ../test_sources/fibonacci.le}}
```

