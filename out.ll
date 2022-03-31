; ModuleID = 'main'
source_filename = "main"

declare void @print_num(i32)

define i32 @func23(i32 %0, i64 %1) {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %0, i32* %4, align 4
  %5 = alloca i64, align 8
  store i64 %1, i64* %5, align 4
  %6 = load i32, i32* %4, align 4
  %7 = icmp eq i32 %6, 1000
  br i1 %7, label %8, label %23

8:                                                ; preds = %2
  %9 = load i64, i64* %5, align 4
  %10 = icmp eq i64 %9, 200
  br i1 %10, label %11, label %21

11:                                               ; preds = %8
  %12 = load i32, i32* %4, align 4
  %13 = load i64, i64* %5, align 4
  %14 = sext i32 %12 to i64
  %15 = add i64 %14, %13
  %16 = load i64, i64* %5, align 4
  %17 = mul i64 %16, 10
  %18 = add i64 %17, 2
  %19 = add i64 %15, %18
  %20 = trunc i64 %19 to i32
  store i32 %20, i32* %3, align 4
  br label %33

21:                                               ; preds = %8
  br label %22

22:                                               ; preds = %21
  br label %32

23:                                               ; preds = %2
  %24 = load i32, i32* %4, align 4
  %25 = load i64, i64* %5, align 4
  %26 = sext i32 %24 to i64
  %27 = add i64 %26, %25
  %28 = load i64, i64* %5, align 4
  %29 = mul i64 %28, 20
  %30 = add i64 %27, %29
  %31 = trunc i64 %30 to i32
  store i32 %31, i32* %3, align 4
  br label %33

32:                                               ; preds = %22
  store i32 1000, i32* %3, align 4
  br label %33

33:                                               ; preds = %32, %23, %11
  %34 = load i32, i32* %3, align 4
  ret i32 %34
}

define i32 @main() {
  %1 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  %2 = call i32 @func23(i32 1000, i64 40)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  %4 = alloca i32, align 4
  %5 = load i32, i32* %3, align 4
  call void @print_num(i32 %5)
  br label %6

6:                                                ; preds = %9, %0
  %7 = load i32, i32* %1, align 4
  %8 = icmp slt i32 %7, 20
  br i1 %8, label %9, label %13

9:                                                ; preds = %6
  %10 = load i32, i32* %1, align 4
  call void @print_num(i32 %10)
  %11 = load i32, i32* %1, align 4
  %12 = add i32 %11, 1
  store i32 %12, i32* %1, align 4
  br label %6

13:                                               ; preds = %6
  %14 = load i32, i32* %3, align 4
  store i32 %14, i32* %4, align 4
  br label %15

15:                                               ; preds = %13
  %16 = load i32, i32* %4, align 4
  ret i32 %16
}
