# ASM-SEM-002 — Control Flow Instructions

## الحالة
`PROVEN_FOR_SCOPE`

## الهدف
توسيع نموذج x86 ليشمل تعليمات التحكم في التدفق.

## النطاق
jmp, jcc (16 variants), call, ret, loop

## خارج النطاق
syscall, int, iret, SIMD, floating point

## العقد
- كل قفزة تأخذ عنواناً وتحول الحالة أو تتقدم للتعليمة التالية
- call يدفع عنوان الرجوع إلى المكدس
- ret يسحب عنوان الرجوع من المكدس
- loop ينقص rcx ويقفز إذا لم يكن صفراً
- fail-closed: قفزة إلى عنوان غير صالح تُرجع خطأً صريحاً

## معايير القبول
- 16 متغير jcc كلها تعمل بشكل صحيح
- call/ret يحفظان rsp ويستعيدانه
- loop يتوقف عند rcx=0
- stdout خام وexit code وSHA-256
- عدم تغيير MAL Parser أو Compiler

## قرار السلامة
لا تضاف التعليمات إلى MAL Parser أو AOT compiler في هذه المرحلة.
