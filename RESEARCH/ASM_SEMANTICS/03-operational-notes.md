# تعليمات Assembly الإضافية بلغة رياضية صرفة

## تذكير بالتعريفات الأساسية

ليكن فضاء الحالة الكاملة:
$$\Sigma = \mathcal{M} \times \mathcal{R} \times \mathcal{F}$$

حيث:
- $\mathcal{M} = \mathbb{Z}_{2^{64}}^{2^{64}}$ — فضاء الذاكرة
- $\mathcal{R} = \{rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp, r8, \ldots, r15, rip, rflags\}$ — المسجلات
- $\mathcal{F} = \{CF, ZF, SF, OF, PF, AF\}$ — الأعلام (Flags)

كل حالة $\sigma \in \Sigma$ هي ثلاثي:
$$\sigma = (mem, regs, flags)$$

---

## 1. PUSH — الدفع إلى المكدس

### التعريف الرسمي

$$\text{push}: (\mathcal{R} \cup \mathbb{Z}_{2^{64}}) \rightarrow \Sigma \rightarrow \Sigma$$

### الحالة 1: `push reg` (مسجل)

$$\text{push\_reg}(r, \sigma) = \sigma''$$

حيث:
$$\sigma' = \text{write\_reg}(rsp, \text{read\_reg}(rsp, \sigma) -_{64} 8, \sigma)$$

$$\sigma'' = \text{write\_mem}(\text{read\_reg}(rsp, \sigma'), \text{read\_reg}(r, \sigma), 8, \sigma')$$

### الحالة 2: `push imm` (قيمة فورية)

$$\text{push\_imm}(v, \sigma) = \sigma''$$

حيث:
$$\sigma' = \text{write\_reg}(rsp, \text{read\_reg}(rsp, \sigma) -_{64} 8, \sigma)$$

$$\sigma'' = \text{write\_mem}(\text{read\_reg}(rsp, \sigma'), v, 8, \sigma')$$

### التفسير

$$\boxed{\text{push}(x) = \text{write\_mem}(\text{dec}(rsp, 8), x, 8)}$$

**الخطوات:**
1. إنزال `rsp` بـ 8 بايتات
2. كتابة القيمة في العنوان الجديد

---

## 2. POP — السحب من المكدس

### التعريف الرسمي

$$\text{pop}: \mathcal{R} \rightarrow \Sigma \rightarrow \Sigma$$

### التنفيذ

$$\text{pop}(r, \sigma) = \sigma''$$

حيث:
$$\sigma' = \text{write\_reg}(r, \text{read\_mem}(\text{read\_reg}(rsp, \sigma), 8, \sigma), \sigma)$$

$$\sigma'' = \text{write\_reg}(rsp, \text{read\_reg}(rsp, \sigma') +_{64} 8, \sigma')$$

### التفسير

$$\boxed{\text{pop}(r) = \text{write\_reg}(r, \text{read\_mem}(rsp)) \circ \text{inc}(rsp, 8)}$$

**الخطوات:**
1. قراءة القيمة من عنوان `rsp`
2. كتابتها في المسجل المستهدف
3. رفع `rsp` بـ 8 بايتات

---

## 3. CMP — المقارنة

### التعريف الرسمي

$$\text{cmp}: (\mathcal{R} \cup \mathbb{Z}_{2^{64}}) \times (\mathcal{R} \cup \mathbb{Z}_{2^{64}}) \rightarrow \Sigma \rightarrow \Sigma$$

### الحالة 1: `cmp reg1, reg2`

$$\text{cmp\_reg\_reg}(r_1, r_2, \sigma) = \sigma'$$

حيث:
$$v_1 = \text{read\_reg}(r_1, \sigma)$$
$$v_2 = \text{read\_reg}(r_2, \sigma)$$
$$d = v_1 -_{64} v_2$$

$$\sigma' = (mem, regs, flags')$$

حيث:
$$flags'(f) = \begin{cases}
1 & \text{if } f = ZF \wedge d = 0 \\
1 & \text{if } f = SF \wedge d \geq 2^{63} \\
1 & \text{if } f = CF \wedge v_1 < v_2 \\
1 & \text{if } f = OF \wedge \text{overflow}(v_1, v_2, d) \\
\text{parity}(d) & \text{if } f = PF \\
flags(f) & \text{otherwise}
\end{cases}$$

### الحالة 2: `cmp reg, imm`

$$\text{cmp\_reg\_imm}(r, v_2, \sigma) = \sigma'$$

حيث:
$$v_1 = \text{read\_reg}(r, \sigma)$$
$$d = v_1 -_{64} v_2$$

ثم تُحسب الأعلام كما في الحالة 1.

### دوال الأعلام

$$\text{overflow}(a, b, r) = (\text{sign}(a) = \text{sign}(b)) \wedge (\text{sign}(r) \neq \text{sign}(a))$$

$$\text{parity}(v) = \left(\sum_{i=0}^{7} \lfloor (v \mod 256) / 2^i \rfloor \mod 2\right) \mod 2 = 0$$

### التفسير

$$\boxed{\text{cmp}(a, b) = \text{compute\_flags}(a -_{64} b)}$$

**ملاحظة:** `cmp` لا تُخزن النتيجة، فقط تُحدّث الأعلام.

---

## 4. TEST — الاختبار المنطقي

### التعريف الرسمي

$$\text{test}: (\mathcal{R} \cup \mathbb{Z}_{2^{64}}) \times (\mathcal{R} \cup \mathbb{Z}_{2^{64}}) \rightarrow \Sigma \rightarrow \Sigma$$

### التنفيذ

$$\text{test}(x, y, \sigma) = \sigma'$$

حيث:
$$r = \text{val}(x, \sigma) \wedge \text{val}(y, \sigma)$$

$$\sigma' = (mem, regs, flags')$$

حيث:
$$flags'(f) = \begin{cases}
1 & \text{if } f = ZF \wedge r = 0 \\
1 & \text{if } f = SF \wedge r \geq 2^{63} \\
0 & \text{if } f \in \{CF, OF\} \\
\text{parity}(r) & \text{if } f = PF \\
flags(f) & \text{otherwise}
\end{cases}$$

### التفسير

$$\boxed{\text{test}(a, b) = \text{compute\_flags}(a \wedge b)}$$

**ملاحظة:** `test` لا تُخزن النتيجة، فقط تُحدّث الأعلام. تُستخدم عادةً لفحص ما إذا كان مسجل صفراً:
$$\text{test}(r, r) \Rightarrow ZF = 1 \iff r = 0$$

---

## 5. LOOP — التكرار

### التعريف الرسمي

$$\text{loop}: \mathbb{Z}_{2^{64}} \rightarrow \Sigma \rightarrow \Sigma$$

### التنفيذ

$$\text{loop}(addr, \sigma) = \sigma''$$

حيث:
$$\sigma' = \text{write\_reg}(rcx, \text{read\_reg}(rcx, \sigma) -_{64} 1, \sigma)$$

$$\sigma'' = \begin{cases}
\text{write\_reg}(rip, addr, \sigma') & \text{if } \text{read\_reg}(rcx, \sigma') \neq 0 \\
\text{write\_reg}(rip, \text{read\_reg}(rip, \sigma') + 2, \sigma') & \text{otherwise}
\end{cases}$$

### المتغيرات

#### `loope` / `loopz` (تكرار مع شرط التساوي)

$$\text{loope}(addr, \sigma) = \begin{cases}
\text{write\_reg}(rip, addr, \sigma') & \text{if } rcx' \neq 0 \wedge ZF = 1 \\
\text{next}(\sigma') & \text{otherwise}
\end{cases}$$

#### `loopne` / `loopnz` (تكرار مع شرط عدم التساوي)

$$\text{loopne}(addr, \sigma) = \begin{cases}
\text{write\_reg}(rip, addr, \sigma') & \text{if } rcx' \neq 0 \wedge ZF = 0 \\
\text{next}(\sigma') & \text{otherwise}
\end{cases}$$

### التفسير

$$\boxed{\text{loop}(addr) = \text{dec}(rcx) \circ \text{jump\_if}(rcx \neq 0, addr)}$$

---

## 6. SYSCALL — استدعاء النظام

### التعريف الرسمي

$$\text{syscall}: \Sigma \rightarrow \Sigma$$

### التنفيذ

$$\text{syscall}(\sigma) = \sigma''$$

حيث:
$$n = \text{read\_reg}(rax, \sigma)$$

$$\sigma' = \text{KernelCall}_n(\sigma)$$

$$\sigma'' = \sigma'$$

### دوال النظام الأساسية

#### `sys_read` (n = 0)

$$\text{sys\_read}(\sigma) = \sigma'$$

حيث:
$$fd = \text{read\_reg}(rdi, \sigma)$$
$$buf = \text{read\_reg}(rsi, \sigma)$$
$$count = \text{read\_reg}(rdx, \sigma)$$

$$\sigma' = \text{write\_mem}(buf, \text{read\_from\_fd}(fd, count), count, \sigma)$$
$$\sigma' = \text{write\_reg}(rax, count, \sigma')$$

#### `sys_write` (n = 1)

$$\text{sys\_write}(\sigma) = \sigma'$$

حيث:
$$fd = \text{read\_reg}(rdi, \sigma)$$
$$buf = \text{read\_reg}(rsi, \sigma)$$
$$count = \text{read\_reg}(rdx, \sigma)$$

$$\sigma' = \text{write\_to\_fd}(fd, \text{read\_mem}(buf, count, \sigma), count, \sigma)$$
$$\sigma' = \text{write\_reg}(rax, count, \sigma')$$

#### `sys_exit` (n = 60)

$$\text{sys\_exit}(\sigma) = \text{HALT}$$

حيث:
$$code = \text{read\_reg}(rdi, \sigma)$$

### التفسير

$$\boxed{\text{syscall} = \text{dispatch}(\text{read\_reg}(rax))}$$

حيث `dispatch` تُوجّه إلى الدالة المناسبة بناءً على رقم الاستدعاء.

---

## الصيغة المجمعة

| التعليمة | التعريف الرياضي |
|---|---|
| `push x` | $\text{write\_mem}(\text{dec}(rsp, 8), x, 8)$ |
| `pop r` | $\text{write\_reg}(r, \text{read\_mem}(rsp, 8)) \circ \text{inc}(rsp, 8)$ |
| `cmp a, b` | $\text{compute\_flags}(a -_{64} b)$ |
| `test a, b` | $\text{compute\_flags}(a \wedge b)$ |
| `loop addr` | $\text{dec}(rcx) \circ \text{jump\_if}(rcx \neq 0, addr)$ |
| `syscall` | $\text{dispatch}(\text{read\_reg}(rax))$ |

---

## مثال: تنفيذ حلقة بلغة رياضية

ليكن البرنامج:
```asm
mov rcx, 5
.loop_start:
    add rax, rbx
    loop .loop_start
```

**الترجمة الرياضية:**

$$\sigma_0 = \text{initial state}$$

$$\sigma_1 = \text{write\_reg}(rcx, 5, \sigma_0)$$

$$\sigma_2 = \text{add\_reg\_reg}(rax, rbx, \sigma_1)$$

$$\sigma_3 = \text{loop}(\text{addr\_of\_loop\_start}, \sigma_2)$$

إذا كان $rcx(\sigma_2) = 5$:
$$\sigma_3 = \text{write\_reg}(rip, \text{addr\_of\_loop\_start}, \text{write\_reg}(rcx, 4, \sigma_2))$$

وهكذا حتى $rcx = 0$.

---

## مثال: طباعة نص باستخدام syscall

ليكن البرنامج:
```asm
mov rax, 1          ; sys_write
mov rdi, 1          ; stdout
mov rsi, msg        ; buffer
mov rdx, 12         ; length
syscall
```

**الترجمة الرياضية:**

$$\sigma_0 = (mem, regs, flags)$$

$$\sigma_1 = \text{write\_reg}(rax, 1, \sigma_0)$$
$$\sigma_2 = \text{write\_reg}(rdi, 1, \sigma_1)$$
$$\sigma_3 = \text{write\_reg}(rsi, \text{addr}(msg), \sigma_2)$$
$$\sigma_4 = \text{write\_reg}(rdx, 12, \sigma_3)$$
$$\sigma_5 = \text{syscall}(\sigma_4) = \text{sys\_write}(\sigma_4)$$

حيث:
$$\text{sys\_write}(\sigma_4) = \text{write\_to\_fd}(1, \text{read\_mem}(\text{addr}(msg), 12, \sigma_4), 12, \sigma_4)$$

---

## الخلاصة

هذا التمثيل الرياضي يسمح بـ:
- ✅ **التحقق الرسمي** من صحة البرامج
- ✅ **إثبات الخصائص** رياضياً (مثل: المكدس متوازن، لا تسرب ذاكرة)
- ✅ **بناء مترجمات صحيحة** (correct-by-construction)
- ✅ **استخدام أدوات مثل** Coq, Agda, Lean, Isabelle

**هل تريد أن أكتب المزيد من التعليمات؟** مثل:
- `lea` (Load Effective Address)
- `mul`, `div` (الضرب والقسمة)
- `ret` (العودة من دالة)
- `int` (القطع البرمجي)
- `nop` (لا عملية)