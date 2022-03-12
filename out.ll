; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %0, i32 %1) {
entry:
  %addtemp = mul i32 1, %1
  %addtemp1 = add i32 %0, %addtemp
  %addtemp2 = mul i32 %1, 1111
  %addtemp3 = add i32 %addtemp2, 2
  %addtemp4 = add i32 %addtemp1, %addtemp3
  ret i32 %addtemp4
}
