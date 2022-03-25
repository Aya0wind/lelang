; ModuleID = 'main'
source_filename = "main"

declare void @print_num(i32)

define i32 @func23(i32 %0, i32 %1) {
  %3 = alloca i32, align 4
  store i32 %0, i32* %3, align 4
  %4 = alloca i32, align 4
  store i32 %1, i32* %4, align 4
  %5 = load i32, i32* %3, align 4
  %6 = icmp eq i32 %5, 1000
  br i1 %6, label %7, label %15

7:                                                ; preds = %2
  %8 = load i32, i32* %3, align 4
  %9 = load i32, i32* %4, align 4
  %10 = add i32 %8, %9
  %11 = load i32, i32* %4, align 4
  %12 = mul i32 %11, 10
  %13 = add i32 %12, 2
  %14 = add i32 %10, %13
  ret i32 %14

15:                                               ; preds = %2
  %16 = load i32, i32* %3, align 4
  %17 = load i32, i32* %4, align 4
  %18 = add i32 %16, %17
  %19 = load i32, i32* %4, align 4
  %20 = mul i32 %19, 20
  %21 = add i32 %18, %20
  ret i32 %21

22:                                               ; No predecessors!
}

define i32 @main() {
  %1 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  %2 = alloca i32, align 4
  %call = call i32 @func23(i32 10, i32 20)
  store i32 %call, i32* %2, align 4
  %3 = load i32, i32* %2, align 4
  call void @print_num(i32 %3)
  br label %4

4:                                                ; preds = %7, %0
  %5 = load i32, i32* %1, align 4
  %6 = icmp slt i32 %5, 100
  br i1 %6, label %7, label %10

7:                                                ; preds = %4
  call void @print_num(i32 9999)
  %8 = load i32, i32* %1, align 4
  %9 = add i32 %8, 1
  store i32 %9, i32* %1, align 4
  br label %4

10:                                               ; preds = %4
  %11 = load i32, i32* %2, align 4
  ret i32 %11
}
