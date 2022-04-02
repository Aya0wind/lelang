; ModuleID = 'main'
source_filename = "main"
target triple = "arm64-apple-darwin21.4.0"

declare void @print_int(i64)

declare void @print_float(float)

define void @no_ret() {
  %1 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  br label %2

2:                                                ; preds = %5, %0
  %3 = load i32, i32* %1, align 4
  %4 = icmp slt i32 %3, 10
  br i1 %4, label %5, label %12

5:                                                ; preds = %2
  %6 = load i32, i32* %1, align 4
  %7 = sext i32 %6 to i64
  call void @print_int(i64 %7)
  %8 = load i32, i32* %1, align 4
  %9 = sext i32 %8 to i64
  %10 = add i64 %9, 1
  %11 = trunc i64 %10 to i32
  store i32 %11, i32* %1, align 4
  br label %2

12:                                               ; preds = %2
  br label %13

13:                                               ; preds = %12
  ret void
}

define i32 @reti32(i32 %0) {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  br label %4

4:                                                ; preds = %17, %1
  %5 = load i32, i32* %2, align 4
  %6 = icmp sgt i32 %5, 0
  br i1 %6, label %7, label %18

7:                                                ; preds = %4
  %8 = load i32, i32* %2, align 4
  %9 = icmp eq i32 %8, 5
  br i1 %9, label %10, label %12

10:                                               ; preds = %7
  %11 = load i32, i32* %2, align 4
  store i32 %11, i32* %3, align 4
  br label %20

12:                                               ; preds = %7
  %13 = load i32, i32* %2, align 4
  %14 = sext i32 %13 to i64
  %15 = sub i64 %14, 1
  %16 = trunc i64 %15 to i32
  store i32 %16, i32* %2, align 4
  br label %17

17:                                               ; preds = %12
  br label %4

18:                                               ; preds = %4
  %19 = load i32, i32* %2, align 4
  store i32 %19, i32* %3, align 4
  br label %20

20:                                               ; preds = %18, %10
  %21 = load i32, i32* %3, align 4
  ret i32 %21
}

define i32 @Fibonacci(i32 %0) {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %4 = load i32, i32* %2, align 4
  %5 = icmp slt i32 %4, 3
  br i1 %5, label %6, label %7

6:                                                ; preds = %1
  store i32 1, i32* %3, align 4
  br label %20

7:                                                ; preds = %1
  %8 = load i32, i32* %2, align 4
  %9 = sext i32 %8 to i64
  %10 = sub i64 %9, 1
  %11 = trunc i64 %10 to i32
  %12 = call i32 @Fibonacci(i32 %11)
  %13 = load i32, i32* %2, align 4
  %14 = sext i32 %13 to i64
  %15 = sub i64 %14, 2
  %16 = trunc i64 %15 to i32
  %17 = call i32 @Fibonacci(i32 %16)
  %18 = add i32 %12, %17
  store i32 %18, i32* %3, align 4
  br label %20

19:                                               ; No predecessors!
  br label %20

20:                                               ; preds = %19, %7, %6
  %21 = load i32, i32* %3, align 4
  ret i32 %21
}

define i32 @main(float %0) {
  %2 = alloca float, align 4
  %3 = alloca float, align 4
  %4 = alloca i32, align 4
  store float %0, float* %3, align 4
  store float 0x402411DB40000000, float* %2, align 4
  call void @no_ret()
  %5 = call i32 @reti32(i32 20)
  %6 = sext i32 %5 to i64
  call void @print_int(i64 %6)
  call void @print_float(float 0xC0FE0F41C0000000)
  %7 = call i32 @Fibonacci(i32 20)
  %8 = sext i32 %7 to i64
  call void @print_int(i64 %8)
  store i32 0, i32* %4, align 4
  br label %9

9:                                                ; preds = %1
  %10 = load i32, i32* %4, align 4
  ret i32 %10
}
