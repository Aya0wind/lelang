; ModuleID = 'test.c'
source_filename = "test.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx12.0.0"

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @m(i32 %0, i32 %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  store i32 %0, i32* %4, align 4
  store i32 %1, i32* %5, align 4
  %7 = load i32, i32* %4, align 4
  %8 = add nsw i32 100, %7
  store i32 %8, i32* %6, align 4
  %9 = load i32, i32* %6, align 4
  %10 = icmp eq i32 %9, 100
  br i1 %10, label %11, label %17

11:                                               ; preds = %2
  store i32 10, i32* %4, align 4
  %12 = load i32, i32* %5, align 4
  %13 = icmp eq i32 %12, 2000
  br i1 %13, label %14, label %16

14:                                               ; preds = %11
  store i32 100, i32* %6, align 4
  %15 = load i32, i32* %4, align 4
  store i32 %15, i32* %3, align 4
  br label %27

16:                                               ; preds = %11
  br label %18

17:                                               ; preds = %2
  store i32 20, i32* %4, align 4
  br label %18

18:                                               ; preds = %17, %16
  %19 = load i32, i32* %4, align 4
  %20 = load i32, i32* %5, align 4
  %21 = mul nsw i32 1, %20
  %22 = add nsw i32 %19, %21
  %23 = load i32, i32* %5, align 4
  %24 = mul nsw i32 %23, 1111
  %25 = add nsw i32 %24, 2
  %26 = add nsw i32 %22, %25
  store i32 %26, i32* %3, align 4
  br label %27

27:                                               ; preds = %18, %14
  %28 = load i32, i32* %3, align 4
  ret i32 %28
}

attributes #0 = { noinline nounwind optnone ssp uwtable "frame-pointer"="non-leaf" "min-legal-vector-width"="0" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.5a,+zcm,+zcz" }

!llvm.module.flags = !{!0, !1, !2, !3, !4, !5, !6, !7, !8}
!llvm.ident = !{!9}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 12, i32 3]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 1, !"branch-target-enforcement", i32 0}
!3 = !{i32 1, !"sign-return-address", i32 0}
!4 = !{i32 1, !"sign-return-address-all", i32 0}
!5 = !{i32 1, !"sign-return-address-with-bkey", i32 0}
!6 = !{i32 7, !"PIC Level", i32 2}
!7 = !{i32 7, !"uwtable", i32 1}
!8 = !{i32 7, !"frame-pointer", i32 1}
!9 = !{!"Apple clang version 13.1.6 (clang-1316.0.21.2)"}
