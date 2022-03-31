; ModuleID = 'main'
source_filename = "main"

declare void @print_num(i32)

define i32 @func23(i32 %0, i64 %1) {
  %3 = icmp eq i32 %0, 1000
  br i1 %3, label %4, label %10

4:                                                ; preds = %2
  %5 = trunc i64 %1 to i32
  %6 = mul i64 %1, 10
  %7 = add i64 %6, 2
  %8 = trunc i64 %7 to i32
  %9 = add i32 %8, i64 %7
  br label %15

10:                                               ; preds = %2
  %11 = trunc i64 %1 to i32
  %12 = mul i64 %1, 20
  %13 = trunc i64 %12 to i32
  %14 = add i32 %13, i64 %12
  br label %15

15:                                               ; preds = %10, %4
  %.0 = phi i32 [ %9, %4 ], [ %14, %10 ]
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
