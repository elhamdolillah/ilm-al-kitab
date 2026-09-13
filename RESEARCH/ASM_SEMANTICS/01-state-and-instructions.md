# لغة التجميع الكاملة بلغة رياضية صرفة

## الجزء الأول: الأسس الرياضية

### 1.1 فضاء الحالة الكامل

ليكن:
$$\mathcal{M} = \mathbb{Z}_{2^{64}}^{2^{64}} \quad \text{(فضاء الذاكرة)}$$

$$\mathcal{R} = \{rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp, r8, \ldots, r15, rip, rflags\} \quad \text{(المسجلات)}$$

$$\mathcal{F} = \{CF, ZF, SF, OF, PF, AF\} \quad \text{(الأعلام)}$$

$$\mathcal{S} = \mathbb{Z}_{2^{64}}^* \quad \text{(المكدس)}$$

$$\mathcal{I} = \{0, 1, 2, \ldots, 255\} \quad \text{(أرقام مقاطعة النظام)}$$

فضاء الحالة الكاملة:
$$\Sigma = \mathcal{M} \times \mathcal{R} \times \mathcal{F} \times \mathcal{S}$$

### 1.2 دوال الوصول الأساسية

$$\text{rd}: \mathcal{R} \times \Sigma \rightarrow \mathbb{Z}_{2^{64}} \quad \text{(قراءة مسجل)}$$

$$\text{wr}: \mathcal{R} \times \mathbb{Z}_{2^{64}} \times \Sigma \rightarrow \Sigma \quad \text{(كتابة مسجل)}$$

$$\text{rm}: \mathbb{Z}_{2^{64}} \times \mathbb{N} \times \Sigma \rightarrow \mathbb{Z}_{2^{64}} \quad \text{(قراءة ذاكرة)}$$

$$\text{wm}: \mathbb{Z}_{2^{64}} \times \mathbb{Z}_{2^{64}} \times \mathbb{N} \times \Sigma \rightarrow \Sigma \quad \text{(كتابة ذاكرة)}$$

### 1.3 العمليات الحسابية المعيارية

$$+_{64}: \mathbb{Z}_{2^{64}} \times \mathbb{Z}_{2^{64}} \rightarrow \mathbb{Z}_{2^{64}} \quad a +_{64} b = (a + b) \mod 2^{64}$$

$$-_{64}: \mathbb{Z}_{2^{64}} \times \mathbb{Z}_{2^{64}} \rightarrow \mathbb{Z}_{2^{64}} \quad a -_{64} b = (a - b) \mod 2^{64}$$

$$\times_{64}: \mathbb{Z}_{2^{64}} \times \mathbb{Z}_{2^{64}} \rightarrow \mathbb{Z}_{2^{64}} \quad a \times_{64} b = (a \cdot b) \mod 2^{64}$$

---

## الجزء الثاني: تعليمات نقل البيانات

### 2.1 MOV — النقل

$$\text{mov}: \text{Operand} \times \text{Operand} \rightarrow \Sigma \rightarrow \Sigma$$

$$\text{mov}(dst, src, \sigma) = \text{wr}(dst, \text{val}(src, \sigma), \sigma)$$

حيث:
$$\text{val}(x, \sigma) = \begin{cases}
\text{rd}(x, \sigma) & \text{if } x \in \mathcal{R} \\
x & \text{if } x \in \mathbb{Z}_{2^{64}} \\
\text{rm}(x, 8, \sigma) & \text{if } x = [addr]
\end{cases}$$

### 2.2 XCHG — التبادل

$$\text{xchg}(a, b, \sigma) = \text{wr}(a, \text{val}(b, \sigma), \text{wr}(b, \text{val}(a, \sigma), \sigma))$$

### 2.3 LEA — تحميل العنوان الفعال

$$\text{lea}(r, expr, \sigma) = \text{wr}(r, \text{compute\_addr}(expr, \sigma), \sigma)$$

حيث:
$$\text{compute\_addr}(base + idx \cdot scale + disp, \sigma) = \text{rd}(base, \sigma) + \text{rd}(idx, \sigma) \cdot scale + disp$$

### 2.4 MOVZX / MOVSX — التمدد

$$\text{movzx}(r, src_{8/16/32}, \sigma) = \text{wr}(r, \text{val}(src, \sigma), \sigma)$$

$$\text{movsx}(r, src_{8/16/32}, \sigma) = \text{wr}(r, \text{sign\_ext}(\text{val}(src, \sigma)), \sigma)$$

حيث:
$$\text{sign\_ext}(v, n) = \begin{cases}
v & \text{if } v < 2^{n-1} \\
v - 2^n & \text{otherwise}
\end{cases}$$

---

## الجزء الثالث: تعليمات المكدس

### 3.1 PUSH — الدفع

$$\text{push}(x, \sigma) = \text{wm}(\text{rd}(rsp, \sigma') , \text{val}(x, \sigma), 8, \sigma')$$

حيث:
$$\sigma' = \text{wr}(rsp, \text{rd}(rsp, \sigma) -_{64} 8, \sigma)$$

### 3.2 POP — السحب

$$\text{pop}(r, \sigma) = \text{wr}(rsp, \text{rd}(rsp, \sigma') +_{64} 8, \sigma')$$

حيث:
$$\sigma' = \text{wr}(r, \text{rm}(\text{rd}(rsp, \sigma), 8, \sigma), \sigma)$$

### 3.3 PUSHF / POPF — الأعلام

$$\text{pushf}(\sigma) = \text{push}(\text{rflags}(\sigma), \sigma)$$

$$\text{popf}(\sigma) = \text{pop}(\text{rflags}, \sigma)$$

### 3.4 ENTER / LEAVE — إطار المكدس

$$\text{enter}(size, \sigma) = \text{push}(rbp, \text{wr}(rbp, rsp, \text{wr}(rsp, rsp - size, \sigma)))$$

$$\text{leave}(\sigma) = \text{wr}(rsp, rbp, \text{pop}(rbp, \sigma))$$

---

## الجزء الرابع: العمليات الحسابية

### 4.1 ADD — الجمع

$$\text{add}(dst, src, \sigma) = \sigma'$$

حيث:
$$a = \text{val}(dst, \sigma), \quad b = \text{val}(src, \sigma)$$
$$r = a +_{64} b$$

$$\sigma' = \text{wr}(dst, r, \text{set\_flags}(a, b, r, \sigma))$$

### 4.2 SUB — الطرح

$$\text{sub}(dst, src, \sigma) = \sigma'$$

حيث:
$$r = a -_{64} b$$

### 4.3 INC / DEC — الزيادة والنقصان

$$\text{inc}(dst, \sigma) = \text{add}(dst, 1, \sigma)$$

$$\text{dec}(dst, \sigma) = \text{sub}(dst, 1, \sigma)$$

### 4.4 NEG — النفي

$$\text{neg}(dst, \sigma) = \text{wr}(dst, -_{64} \text{val}(dst, \sigma), \sigma)$$

### 4.5 MUL / IMUL — الضرب

$$\text{mul}(src, \sigma) = \sigma'$$

حيث:
$$a = \text{rd}(rax, \sigma), \quad b = \text{val}(src, \sigma)$$
$$p = a \cdot b \quad \text{(128-bit result)}$$

$$\sigma' = \text{wr}(rax, p \mod 2^{64}, \text{wr}(rdx, \lfloor p / 2^{64} \rfloor, \sigma))$$

### 4.6 DIV / IDIV — القسمة

$$\text{div}(src, \sigma) = \sigma'$$

حيث:
$$a = \text{rd}(rax, \sigma), \quad b = \text{val}(src, \sigma)$$
$$q = \lfloor a / b \rfloor, \quad r = a \mod b$$

$$\sigma' = \text{wr}(rax, q, \text{wr}(rdx, r, \sigma))$$

### 4.7 CMP — المقارنة

$$\text{cmp}(a, b, \sigma) = \text{set\_flags}(\text{val}(a, \sigma), \text{val}(b, \sigma), \text{val}(a, \sigma) -_{64} \text{val}(b, \sigma), \sigma)$$

**ملاحظة:** لا تُخزن النتيجة، فقط تُحدّث الأعلام.

---

## الجزء الخامس: العمليات المنطقية

### 5.1 AND / OR / XOR / NOT

$$\text{and}(dst, src, \sigma) = \text{wr}(dst, \text{val}(dst, \sigma) \wedge \text{val}(src, \sigma), \sigma)$$

$$\text{or}(dst, src, \sigma) = \text{wr}(dst, \text{val}(dst, \sigma) \vee \text{val}(src, \sigma), \sigma)$$

$$\text{xor}(dst, src, \sigma) = \text{wr}(dst, \text{val}(dst, \sigma) \oplus \text{val}(src, \sigma), \sigma)$$

$$\text{not}(dst, \sigma) = \text{wr}(dst, \neg \text{val}(dst, \sigma), \sigma)$$

### 5.2 TEST — الاختبار

$$\text{test}(a, b, \sigma) = \text{set\_flags}(0, 0, \text{val}(a, \sigma) \wedge \text{val}(b, \sigma), \sigma)$$

### 5.3 SHL / SHR / SAR — الإزاحة

$$\text{shl}(dst, n, \sigma) = \text{wr}(dst, (\text{val}(dst, \sigma) \cdot 2^n) \mod 2^{64}, \sigma)$$

$$\text{shr}(dst, n, \sigma) = \text{wr}(dst, \lfloor \text{val}(dst, \sigma) / 2^n \rfloor, \sigma)$$

$$\text{sar}(dst, n, \sigma) = \text{wr}(dst, \text{sign\_ext}(\lfloor \text{val}(dst, \sigma) / 2^n \rfloor), \sigma)$$

### 5.4 ROL / ROR — الدوران

$$\text{rol}(dst, n, \sigma) = \text{wr}(dst, \text{rotate\_left}(\text{val}(dst, \sigma), n), \sigma)$$

$$\text{ror}(dst, n, \sigma) = \text{wr}(dst, \text{rotate\_right}(\text{val}(dst, \sigma), n), \sigma)$$

حيث:
$$\text{rotate\_left}(v, n) = ((v \ll n) \lor (v \gg (64-n))) \mod 2^{64}$$

---

## الجزء السادس: التحكم في التدفق

### 6.1 JMP — القفز غير المشروط

$$\text{jmp}(addr, \sigma) = \text{wr}(rip, addr, \sigma)$$

### 6.2 JCC — القفز المشروط

$$\text{jcc}(addr, cond, \sigma) = \begin{cases}
\text{wr}(rip, addr, \sigma) & \text{if } cond(\sigma) = 1 \\
\text{wr}(rip, \text{rd}(rip, \sigma) + 2, \sigma) & \text{otherwise}
\end{cases}$$

#### الشروط الأساسية

| التعليمة | الشرط | الصيغة الرياضية |
|---|---|---|
| `je` / `jz` | متساوٍ | $ZF = 1$ |
| `jne` / `jnz` | غير متساوٍ | $ZF = 0$ |
| `jl` / `jnge` | أقل (موقّع) | $SF \oplus OF = 1$ |
| `jge` / `jnl` | أكبر أو يساوي (موقّع) | $SF \oplus OF = 0$ |
| `jb` / `jc` | أقل (غير موقّع) | $CF = 1$ |
| `jae` / `jnc` | أكبر أو يساوي (غير موقّع) | $CF = 0$ |
| `jle` / `jng` | أقل أو يساوي (موقّع) | $(SF \oplus OF) \lor ZF = 1$ |
| `jg` / `jnle` | أكبر (موقّع) | $(SF \oplus OF) \lor ZF = 0$ |
| `jbe` / `jna` | أقل أو يساوي (غير موقّع) | $CF \lor ZF = 1$ |
| `ja` / `jnbe` | أكبر (غير موقّع) | $CF \lor ZF = 0$ |
| `js` | سالب | $SF = 1$ |
| `jns` | غير سالب | $SF = 0$ |
| `jo` | فيضان | $OF = 1$ |
| `jno` | لا فيضان | $OF = 0$ |
| `jp` / `jpe` | زوجية | $PF = 1$ |
| `jnp` / `jpo` | فردية | $PF = 0$ |

### 6.3 CALL — الاستدعاء

$$\text{call}(addr, \sigma) = \sigma'''$$

حيث:
$$\sigma' = \text{push}(\text{rd}(rip, \sigma) + 5, \sigma)$$
$$\sigma'' = \text{wr}(rsp, \text{rd}(rsp, \sigma') -_{64} 8, \sigma')$$
$$\sigma''' = \text{wr}(rip, addr, \sigma'')$$

### 6.4 RET — العودة

$$\text{ret}(\sigma) = \sigma''$$

حيث:
$$\sigma' = \text{pop}(rip, \sigma)$$
$$\sigma'' = \sigma'$$

### 6.5 LOOP — التكرار

$$\text{loop}(addr, \sigma) = \begin{cases}
\text{wr}(rip, addr, \sigma') & \text{if } \text{rd}(rcx, \sigma') \neq 0 \\
\text{wr}(rip, \text{rd}(rip, \sigma') + 2, \sigma') & \text{otherwise}
\end{cases}$$

حيث:
$$\sigma' = \text{wr}(rcx, \text{rd}(rcx, \sigma) -_{64} 1, \sigma)$$

---

## الجزء السابع: تعليمات السلاسل

### 7.1 MOVS — نقل سلسلة

$$\text{movsb}(\sigma) = \text{wm}(\text{rd}(rdi, \sigma), \text{rm}(\text{rd}(rsi, \sigma), 1, \sigma), 1, \sigma')$$

حيث:
$$\sigma' = \text{wr}(rsi, \text{rd}(rsi, \sigma) + df, \text{wr}(rdi, \text{rd}(rdi, \sigma) + df, \sigma))$$

و $df = 1$ إذا كان $DF = 0$، وإلا $df = -1$.

### 7.2 STOS — تخزين سلسلة

$$\text{stosb}(\sigma) = \text{wm}(\text{rd}(rdi, \sigma), \text{rd}(al, \sigma), 1, \sigma')$$

### 7.3 LODS — تحميل سلسلة

$$\text{lodsb}(\sigma) = \text{wr}(al, \text{rm}(\text{rd}(rsi, \sigma), 1, \sigma), \sigma')$$

### 7.4 CMPS — مقارنة سلسلة

$$\text{cmpsb}(\sigma) = \text{set\_flags}(\text{rm}(rsi, 1, \sigma), \text{rm}(rdi, 1, \sigma), \ldots, \sigma')$$

### 7.5 SCAS — فحص سلسلة

$$\text{scasb}(\sigma) = \text{set\_flags}(\text{rd}(al, \sigma), \text{rm}(rdi, 1, \sigma), \ldots, \sigma')$$

### 7.6 REP — التكرار

$$\text{rep}(instr, \sigma) = \text{instr}^{\text{rd}(rcx, \sigma)}(\sigma)$$

حيث:
$$\text{instr}^0(\sigma) = \sigma$$
$$\text{instr}^{n+1}(\sigma) = \text{instr}(\text{instr}^n(\sigma))$$

---

## الجزء الثامن: تعليمات النظام

### 8.1 SYSCALL — استدعاء النظام

$$\text{syscall}(\sigma) = \text{dispatch}(\text{rd}(rax, \sigma), \sigma)$$

حيث:
$$\text{dispatch}(n, \sigma) = \begin{cases}
\text{sys\_read}(\sigma) & \text{if } n = 0 \\
\text{sys\_write}(\sigma) & \text{if } n = 1 \\
\text{sys\_open}(\sigma) & \text{if } n = 2 \\
\text{sys\_close}(\sigma) & \text{if } n = 3 \\
\text{sys\_mmap}(\sigma) & \text{if } n = 9 \\
\text{sys\_munmap}(\sigma) & \text{if } n = 11 \\
\text{sys\_fork}(\sigma) & \text{if } n = 57 \\
\text{sys\_execve}(\sigma) & \text{if } n = 59 \\
\text{sys\_exit}(\sigma) & \text{if } n = 60 \\
\text{sys\_wait4}(\sigma) & \text{if } n = 61 \\
\text{sys\_pipe2}(\sigma) & \text{if } n = 293 \\
\text{sys\_epoll\_create1}(\sigma) & \text{if } n = 291 \\
\text{sys\_epoll\_wait}(\sigma) & \text{if } n = 232 \\
\text{sys\_eventfd2}(\sigma) & \text{if } n = 290 \\
\vdots & \vdots
\end{cases}$$

### 8.2 INT — المقاطعة البرمجية

$$\text{int}(n, \sigma) = \text{interrupt\_handler}_n(\sigma)$$

### 8.3 IRET — العودة من المقاطعة

$$\text{iret}(\sigma) = \text{pop}(rip, \text{pop}(cs, \text{pop}(rflags, \sigma)))$$

### 8.4 HLT — الإيقاف

$$\text{hlt}(\sigma) = \text{HALT}$$

### 8.5 NOP — لا عملية

$$\text{nop}(\sigma) = \sigma$$

### 8.6 CLI / STI — التحكم في المقاطعات

$$\text{cli}(\sigma) = \text{set\_flag}(IF, 0, \sigma)$$

$$\text{sti}(\sigma) = \text{set\_flag}(IF, 1, \sigma)$$

---

## الجزء التاسع: الفاصلة العائمة

### 9.1 FLD / FST — التحميل والتخزين

$$\text{fld}(src, \sigma) = \text{push\_fp}(\text{val}(src, \sigma), \sigma)$$

$$\text{fst}(dst, \sigma) = \text{wr}(dst, \text{top\_fp}(\sigma), \sigma)$$

### 9.2 FADD / FSUB / FMUL / FDIV

$$\text{fadd}(src, \sigma) = \text{push\_fp}(\text{top\_fp}(\sigma) + \text{val}(src, \sigma), \text{pop\_fp}(\sigma))$$

$$\text{fsub}(src, \sigma) = \text{push\_fp}(\text{top\_fp}(\sigma) - \text{val}(src, \sigma), \text{pop\_fp}(\sigma))$$

$$\text{fmul}(src, \sigma) = \text{push\_fp}(\text{top\_fp}(\sigma) \cdot \text{val}(src, \sigma), \text{pop\_fp}(\sigma))$$

$$\text{fdiv}(src, \sigma) = \text{push\_fp}(\text{top\_fp}(\sigma) / \text{val}(src, \sigma), \text{pop\_fp}(\sigma))$$

---

## الجزء العاشر: SIMD

### 10.1 MOVDQU / MOVDQA — النقل المتجهي

$$\text{movdqu}(xmm, src, \sigma) = \text{wr\_vec}(xmm, \text{rm\_vec}(src, 16, \sigma), \sigma)$$

### 10.2 PADDB / PADDW / PADDD — الجمع المتجهي

$$\text{paddd}(xmm1, xmm2, \sigma) = \text{wr\_vec}(xmm1, \text{vec\_add}(\text{rd\_vec}(xmm1, \sigma), \text{rd\_vec}(xmm2, \sigma)), \sigma)$$

حيث:
$$\text{vec\_add}([a_0, a_1, a_2, a_3], [b_0, b_1, b_2, b_3]) = [a_0 + b_0, a_1 + b_1, a_2 + b_2, a_3 + b_3]$$

### 10.3 PMULLD — الضرب المتجهي

$$\text{pmulld}(xmm1, xmm2, \sigma) = \text{wr\_vec}(xmm1, \text{vec\_mul}(\text{rd\_vec}(xmm1, \sigma), \text{rd\_vec}(xmm2, \sigma)), \sigma)$$

---

## الجزء الحادي عشر: دوال الأعلام

### 11.1 حساب الأعلام

$$\text{set\_flags}(a, b, r, \sigma) = (mem, regs, flags')$$

حيث:
$$flags'(f) = \begin{cases}
1 & \text{if } f = CF \wedge r < a \\
1 & \text{if } f = ZF \wedge r = 0 \\
1 & \text{if } f = SF \wedge r \geq 2^{63} \\
1 & \text{if } f = OF \wedge \text{overflow}(a, b, r) \\
\text{parity}(r) & \text{if } f = PF \\
flags(f) & \text{otherwise}
\end{cases}$$

حيث:
$$\text{overflow}(a, b, r) = (\text{sign}(a) = \text{sign}(b)) \wedge (\text{sign}(r) \neq \text{sign}(a))$$

$$\text{parity}(v) = \left(\sum_{i=0}^{7} \lfloor (v \mod 256) / 2^i \rfloor \mod 2\right) \mod 2 = 0$$

---

## الجزء الثاني عشر: الصيغة المجمعة

### 12.1 جدول شامل

| الفئة | التعليمة | التعريف الرياضي |
|---|---|---|
| **نقل** | `mov` | $\text{wr}(dst, \text{val}(src, \sigma), \sigma)$ |
| | `xchg` | $\text{wr}(a, b, \text{wr}(b, a, \sigma))$ |
| | `lea` | $\text{wr}(r, \text{compute\_addr}(expr), \sigma)$ |
| **مكدس** | `push` | $\text{wm}(\text{dec}(rsp, 8), x, 8)$ |
| | `pop` | $\text{wr}(r, \text{rm}(rsp, 8)) \circ \text{inc}(rsp, 8)$ |
| **حساب** | `add` | $\text{wr}(dst, a +_{64} b) \circ \text{set\_flags}$ |
| | `sub` | $\text{wr}(dst, a -_{64} b) \circ \text{set\_flags}$ |
| | `mul` | $\text{wr}(rax, p \mod 2^{64}) \circ \text{wr}(rdx, \lfloor p / 2^{64} \rfloor)$ |
| | `div` | $\text{wr}(rax, q) \circ \text{wr}(rdx, r)$ |
| | `cmp` | $\text{set\_flags}(a, b, a -_{64} b)$ |
| **منطق** | `and` | $\text{wr}(dst, a \wedge b)$ |
| | `or` | $\text{wr}(dst, a \vee b)$ |
| | `xor` | $\text{wr}(dst, a \oplus b)$ |
| | `not` | $\text{wr}(dst, \neg a)$ |
| | `test` | $\text{set\_flags}(0, 0, a \wedge b)$ |
| | `shl` | $\text{wr}(dst, (a \cdot 2^n) \mod 2^{64})$ |
| | `shr` | $\text{wr}(dst, \lfloor a / 2^n \rfloor)$ |
| **تدفق** | `jmp` | $\text{wr}(rip, addr)$ |
| | `jcc` | $\text{wr}(rip, addr) \text{ if } cond(\sigma) = 1$ |
| | `call` | $\text{push}(rip + 5) \circ \text{wr}(rip, addr)$ |
| | `ret` | $\text{pop}(rip)$ |
| | `loop` | $\text{dec}(rcx) \circ \text{jump\_if}(rcx \neq 0, addr)$ |
| **سلاسل** | `movsb` | $\text{wm}(rdi, \text{rm}(rsi, 1), 1) \circ \text{inc}(rsi, rdi)$ |
| | `rep` | $\text{instr}^{\text{rd}(rcx)}$ |
| **نظام** | `syscall` | $\text{dispatch}(\text{rd}(rax))$ |
| | `int` | $\text{interrupt\_handler}_n$ |
| | `nop` | $\text{id}(\sigma) = \sigma$ |
| | `hlt` | $\text{HALT}$ |

### 12.2 البرنامج كتركيب دوال

ليكن البرنامج $P$ يتكون من $n$ تعليمة:
$$P = [I_1, I_2, \ldots, I_n]$$

التنفيذ هو تركيب:
$$\text{exec}(P, \sigma_0) = I_n \circ I_{n-1} \circ \cdots \circ I_1(\sigma_0) = \sigma_n$$

أو بشكل تكراري:
$$\sigma_{i+1} = I_{i+1}(\sigma_i), \quad i = 0, 1, \ldots, n-1$$

### 12.3 الدلالات التشغيلية الصغرى

$$\frac{\sigma \xrightarrow{I} \sigma'}{\langle I; P, \sigma \rangle \rightarrow \langle P, \sigma' \rangle}$$

$$\frac{}{\langle \text{nop}; P, \sigma \rangle \rightarrow \langle P, \sigma \rangle}$$

$$\frac{\sigma \xrightarrow{\text{call}(addr)} \sigma'}{\langle \text{call}(addr); P, \sigma \rangle \rightarrow \langle \text{func}(addr); \text{ret}; P, \sigma' \rangle}$$

---

## الجزء الثالث عشر: أمثلة تطبيقية

### 13.1 حلقة جمع

```asm
mov rcx, 5
xor rax, rax
.loop:
    add rax, rbx
    loop .loop
```

**الترجمة الرياضية:**

$$\sigma_0 \xrightarrow{\text{mov}(rcx, 5)} \sigma_1 \xrightarrow{\text{xor}(rax, rax)} \sigma_2$$

$$\sigma_3 = \text{add}(rax, rbx, \sigma_2)$$

$$\sigma_4 = \text{loop}(\text{addr}(\text{.loop}), \sigma_3)$$

إذا كان $rcx(\sigma_3) = 5$:
$$\sigma_4 = \text{wr}(rip, \text{addr}(\text{.loop}), \text{wr}(rcx, 4, \sigma_3))$$

وهكذا حتى $rcx = 0$.

### 13.2 طباعة نص

```asm
section .data
msg: db "Hello", 10
len: equ $ - msg

section .text
_start:
    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rsi, msg        ; buffer
    mov rdx, len        ; length
    syscall
    mov rax, 60         ; sys_exit
    xor rdi, rdi        ; exit code 0
    syscall
```

**الترجمة الرياضية:**

$$\sigma_0 \xrightarrow{\text{mov}(rax, 1)} \sigma_1 \xrightarrow{\text{mov}(rdi, 1)} \sigma_2 \xrightarrow{\text{mov}(rsi, msg)} \sigma_3 \xrightarrow{\text{mov}(rdx, len)} \sigma_4$$

$$\sigma_5 = \text{syscall}(\sigma_4) = \text{sys\_write}(\sigma_4)$$

حيث:
$$\text{sys\_write}(\sigma_4) = \text{write\_to\_fd}(1, \text{read\_mem}(msg, len, \sigma_4), len, \sigma_4)$$

---

## الجزء الرابع عشر: التطبيقات

هذا التمثيل الرياضي يسمح بـ:

1. **التحقق الرسمي** (Formal Verification)
   - إثبات صحة البرامج باستخدام Coq, Agda, Lean
   - التحقق من خصائص مثل: المكدس متوازن، لا تسرب ذاكرة

2. **بناء مترجمات صحيحة** (Correct-by-Construction Compilers)
   - إثبات أن المترجم يحافظ على الدلالات

3. **التحليل الساكن** (Static Analysis)
   - تحديد القيم الممكنة لكل مسجل
   - كشف الأخطاء المحتملة

4. **التحسين** (Optimization)
   - إثبات تكافؤ التحويلات
   - إزالة الكود الميت

5. **الأمن** (Security)
   - إثبات عدم وجود ثغرات
   - التحقق من سياسات الوصول

---

## الخلاصة

جميع تعليمات لغة التجميع يمكن تمثيلها كدوال من $\Sigma$ إلى $\Sigma$:

$$\boxed{\text{Instruction}: \Sigma \rightarrow \Sigma}$$

حيث:
$$\sigma' = \text{Instruction}(\sigma)$$

والبرنامح الكامل هو تركيب لهذه الدوال:
$$\text{Program} = I_n \circ I_{n-1} \circ \cdots \circ I_1$$

هذا التمثيل يوفر أساساً رياضياً متيناً لفهم وتحليل وإثبات خصائص البرامج المكتوبة بلغة التجميع.---
## Mathematical Semantics — Formal State Transition
### Axiomatic State Definition
Σ = (regs: R → ℤ₆₄, mem: Addr → Byte, ip: Addr, flags: F)
where:
- R = {rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp}
- F = {ZF, CF, SF, OF}
- ℤ₆₄ = ℤ mod 2⁶⁴ (64-bit integers)
### Instruction Semantics
∀ instruction I, state Σ:
| Instruction | Type Signature | Formal Definition |
|---|---|---|
| mov | R × Operand → Σ | Σ'.regs[r] ← eval(operand, Σ) |
| add | R × Operand → Σ | Σ'.regs[r] ← Σ.regs[r] + eval(op, Σ), flags ← compute |
| sub | R × Operand → Σ | Σ'.regs[r] ← Σ.regs[r] - eval(op, Σ), flags ← compute |
| push | Operand → Σ | Σ'.mem[Σ.regs[rsp]-8] ← eval(op), Σ'.regs[rsp] ← Σ.regs[rsp]-8 |
| pop | R → Σ | Σ'.regs[r] ← Σ.mem[Σ.regs[rsp]], Σ'.regs[rsp] ← Σ.regs[rsp]+8 |
| cmp | Op × Op → Σ | flags ← compare(eval(a), eval(b)) |
### Algebraic Properties
- **Commutativity**: add(a, b) ≡ add(b, a)
- **Associativity**: add(add(a, b), c) ≡ add(a, add(b, c))
- **Identity**: add(a, 0) ≡ a
- **Inverse**: sub(a, a) ≡ 0
- **Stack invariant**: ∀ n: pop(push(x)) ≡ x
### Memory Model
mem: Addr → Byte where Addr = ℤ₆₄, Byte = {0, 1, ..., 255}
∀ address a ∈ Addr:
- Read: read(a) = mem[a]
- Write: write(a, v) ⟹ mem[a] ← v
- Bounds: a < 2⁶⁴ (64-bit address space)
