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

define i32 @main(float %0) {
  %2 = alloca i32, align 4
  %3 = alloca float, align 4
  %4 = alloca float, align 4
  store float %0, float* %4, align 4
  store float 0x402411DB40000000, float* %3, align 4
  store i32 -123400080, i32* %2, align 4
  call void @print_float(float 0xC0FE0F41C0000000)
  %5 = load float, float* %3, align 4
  %6 = fptosi float %5 to i32
  store i32 %6, i32* %8, align 4
  br label %7

7:                                                ; preds = %1
  %8 = alloca i32, align 4
  %9 = load i32, i32* %8, align 4
  ret i32 %9
}
