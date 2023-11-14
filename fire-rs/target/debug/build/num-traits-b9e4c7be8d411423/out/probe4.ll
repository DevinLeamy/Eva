; ModuleID = 'probe4.3b282679cc950ee4-cgu.0'
source_filename = "probe4.3b282679cc950ee4-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_68bafaf82be4ffd12289b12f2005a32c = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fee5518cdd4435c60a57fe3bb734fc1a14abeb7a/library/core/src/num/mod.rs" }>, align 1
@alloc_c3ad208112d313c240ea5cc9ec109091 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_68bafaf82be4ffd12289b12f2005a32c, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h0b8b24d288d691c2E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hedd56adabd5e5f98E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc83ba2327a09323eE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_c3ad208112d313c240ea5cc9ec109091) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hedd56adabd5e5f98E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hc83ba2327a09323eE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0-nightly (fee5518cd 2023-11-05)"}
