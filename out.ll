; ModuleID = 'main'
source_filename = "main"
target triple = "arm64-apple-darwin21.4.0"

declare void @print_num(i32)

declare void @print_f32(float)

declare void @print_f64(float)

define i32 @func23(i32 %0, float %1) {
  %3 = alloca float, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  store i32 %0, i32* %4, align 4
  store float %1, float* %3, align 4
  %6 = load i32, i32* %4, align 4
  %7 = icmp eq i32 %6, 1000
  br i1 %7, label %8, label %18

8:                                                ; preds = %2
  %9 = load i32, i32* %4, align 4
  %10 = load float, float* %3, align 4
  %11 = sitofp i32 %9 to float
  %12 = fadd float %10, %11
  %13 = load float, float* %3, align 4
  %14 = fdiv float %13, 1.000000e+01
  %15 = fadd float %14, 2.000000e+00
  %16 = fadd float %12, %15
  %17 = fptosi float %16 to i32
  store i32 %17, i32* %5, align 4
  br label %28

18:                                               ; preds = %2
  %19 = load i32, i32* %4, align 4
  %20 = load float, float* %3, align 4
  %21 = sitofp i32 %19 to float
  %22 = fadd float %20, %21
  %23 = load float, float* %3, align 4
  %24 = fmul float %23, 2.000000e+01
  %25 = fadd float %22, %24
  %26 = fptosi float %25 to i32
  store i32 %26, i32* %5, align 4
  br label %28

27:                                               ; No predecessors!
  store i32 1000, i32* %5, align 4
  br label %28

28:                                               ; preds = %27, %18, %8
  %29 = load i32, i32* %5, align 4
  ret i32 %29
}

define float @func24(float %0) {
  %2 = alloca i16, align 2
  %3 = alloca float, align 4
  %4 = alloca float, align 4
  store float %0, float* %3, align 4
  store i16 10, i16* %2, align 2
  br label %5

5:                                                ; preds = %8, %1
  %6 = load i16, i16* %2, align 2
  %7 = icmp slt i16 %6, 20
  br i1 %7, label %8, label %17

8:                                                ; preds = %5
  %9 = load float, float* %3, align 4
  %10 = load i16, i16* %2, align 2
  %11 = sitofp i16 %10 to float
  %12 = fadd float %9, %11
  store float %12, float* %3, align 4
  %13 = load i16, i16* %2, align 2
  %14 = sext i16 %13 to i64
  %15 = add i64 %14, 1
  %16 = trunc i64 %15 to i16
  store i16 %16, i16* %2, align 2
  br label %5

17:                                               ; preds = %5
  %18 = load float, float* %3, align 4
  store float %18, float* %4, align 4
  br label %19

19:                                               ; preds = %17
  %20 = load float, float* %4, align 4
  ret float %20
}

define i32 @func25(i64 %0) {
  %2 = alloca i32, align 4
  %3 = alloca i64, align 8
  %4 = alloca i32, align 4
  store i64 %0, i64* %3, align 4
  %5 = load i64, i64* %3, align 4
  %6 = trunc i64 %5 to i32
  store i32 %6, i32* %2, align 4
  br label %7

7:                                                ; preds = %10, %1
  %8 = load i64, i64* %3, align 4
  %9 = icmp sgt i64 %8, 100
  br i1 %9, label %10, label %14

10:                                               ; preds = %7
  %11 = load i64, i64* %3, align 4
  %12 = sub i64 %11, 100
  store i64 %12, i64* %3, align 4
  %13 = trunc i64 %12 to i32
  call void @print_num(i32 %13)
  br label %7

14:                                               ; preds = %7
  %15 = load i32, i32* %2, align 4
  store i32 %15, i32* %4, align 4
  br label %16

16:                                               ; preds = %14
  %17 = load i32, i32* %4, align 4
  ret i32 %17
}

define i32 @main() {
  %1 = alloca float, align 4
  %2 = alloca i32, align 4
  %3 = call i32 @func23(i32 100, float 8.000000e+01)
  %4 = sitofp i32 %3 to float
  %5 = call float @func24(float %4)
  store float %5, float* %1, align 4
  %6 = load float, float* %1, align 4
  %7 = fptosi float %6 to i64
  %8 = call i32 @func25(i64 %7)
  call void @print_num(i32 %8)
  %9 = load float, float* %1, align 4
  %10 = fptosi float %9 to i32
  store i32 %10, i32* %2, align 4
  br label %11

11:                                               ; preds = %0
  %12 = load i32, i32* %2, align 4
  ret i32 %12
}
