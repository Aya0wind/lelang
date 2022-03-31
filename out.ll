; ModuleID = 'main'
source_filename = "main"

declare void @print_num(i32)

define i32 @func23(i32 %0, i64 %1) {
  %3 = icmp eq i32 %0, 1000
  br i1 %3, label %4, label %11

4:                                                ; preds = %2
  %5 = sext i32 %0 to i64
  %6 = add i64 %5, %1
  %7 = mul i64 %1, 10
  %8 = add i64 %7, 2
  %9 = add i64 %6, %8
  %10 = trunc i64 %9 to i32
  br label %17

11:                                               ; preds = %2
  %12 = sext i32 %0 to i64
  %13 = add i64 %12, %1
  %14 = mul i64 %1, 20
  %15 = add i64 %13, %14
  %16 = trunc i64 %15 to i32
  br label %17

17:                                               ; preds = %11, %4
  %.0 = phi i32 [ %10, %4 ], [ %16, %11 ]
  ret i32 %.0
}

define i32 @main() {
  %1 = call i32 @func23(i32 1000, i64 40)
  call void @print_num(i32 %1)
  br label %2

2:                                                ; preds = %4, %0
  %.0 = phi i32 [ 10, %0 ], [ %5, %4 ]
  %3 = icmp slt i32 %.0, 20
  br i1 %3, label %4, label %6

4:                                                ; preds = %2
  call void @print_num(i32 %.0)
  %5 = add i32 %.0, 1
  br label %2

6:                                                ; preds = %2
  ret i32 %1
}
