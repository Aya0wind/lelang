; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %0, i32 %1) {
  %3 = alloca i32, align 4
  store i32 %0, i32* %3, align 4
  %4 = alloca i32, align 4
  store i32 %1, i32* %4, align 4
  %5 = alloca i32, align 4
  %6 = load i32, i32* %3, align 4
  %7 = add i32 100, %6
  store i32 %7, i32* %5, align 4
  %8 = load i32, i32* %5, align 4
  %9 = icmp eq i32 %8, 100
  br i1 %9, label %10, label %19

10:                                               ; preds = %2
  %11 = load i32, i32* %3, align 4
  store i32 %11, i32* %3, align 4
  %12 = load i32, i32* %4, align 4
  %13 = icmp eq i32 %12, 2000
  br i1 %13, label %14, label %17

14:                                               ; preds = %10
  %15 = load i32, i32* %5, align 4
  store i32 %15, i32* %5, align 4
  %16 = load i32, i32* %3, align 4
  ret i32 %16

17:                                               ; preds = %10
  br label %18

18:                                               ; preds = %17
  br label %21

19:                                               ; preds = %2
  %20 = load i32, i32* %3, align 4
  store i32 %20, i32* %3, align 4
  br label %21

21:                                               ; preds = %19, %18
  %22 = load i32, i32* %3, align 4
  %23 = load i32, i32* %4, align 4
  %24 = mul i32 1, %23
  %25 = add i32 %22, %24
  %26 = load i32, i32* %4, align 4
  %27 = mul i32 %26, 1111
  %28 = add i32 %27, 2
  %29 = add i32 %25, %28
  ret i32 %29
}
