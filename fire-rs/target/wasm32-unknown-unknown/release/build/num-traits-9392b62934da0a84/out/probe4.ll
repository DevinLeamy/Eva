; ModuleID = 'probe4.3b282679cc950ee4-cgu.0'
source_filename = "probe4.3b282679cc950ee4-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_68bafaf82be4ffd12289b12f2005a32c = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fee5518cdd4435c60a57fe3bb734fc1a14abeb7a/library/core/src/num/mod.rs" }>, align 1
@alloc_c56c7c3c0740fb8be112afd58638b804 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_68bafaf82be4ffd12289b12f2005a32c, [12 x i8] c"K\00\00\00y\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h0b8b24d288d691c2E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hba14ebd1ad38141fE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h4f05fddcabdffb5aE(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_c56c7c3c0740fb8be112afd58638b804) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hba14ebd1ad38141fE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h4f05fddcabdffb5aE(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.75.0-nightly (fee5518cd 2023-11-05)"}
