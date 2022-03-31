; ModuleID = 'main'
source_filename = "main"

declare void @print_num(i32)

define i32 @func23(i32 %0, i64 %1) {
  %3 = icmp eq i32 %0, 1000
  br i1 %3, label %4, label %13

4:                                                ; preds = %2
  %5 = icmp eq i64 %1, 200
  br i1 %5, label %6, label %19

6:                                                ; preds = %4
  %7 = sext i32 %0 to i64
  %8 = add i64 %7, %1
  %9 = mul i64 %1, 10
  %10 = add i64 %9, 2
  %11 = add i64 %8, %10
  %12 = trunc i64 %11 to i32
  br label %20

13:                                               ; preds = %2
  %14 = sext i32 %0 to i64
  %15 = add i64 %14, %1
  %16 = mul i64 %1, 20
  %17 = add i64 %15, %16
  %18 = trunc i64 %17 to i32
  br label %20

19:                                               ; preds = %4
  br label %20

20:                                               ; preds = %19, %13, %6
  %.0 = phi i32 [ %12, %6 ], [ 1000, %19 ], [ %18, %13 ]
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
