#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
اللغة الرياضية العربية - الملف المتكامل النهائي
المُجمّع + الاختبارات + الآلة الحاسبة
جميع الحلول وفق الدستور الرياضي
"""
import sys, subprocess

# ═══════════════════════════════════════════════════════════
# Lexer
# ═══════════════════════════════════════════════════════════
أرقام_القيم = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9}
رموز_العمليات = {
    "≔":"≔","≡":"≡","+":"+","-":"-","·":"·","*":"·","×":"·","⊕":"⊕","⎕":"⎕",
    "(":"(",")":")",",":",","،":",",":":":",".":".",
    "⟨":"⟨","⟩":"⟩","<":"<",">":">","=":"=","≠":"≠","؟":"؟",
    "∀":"∀","∈":"∈","μ":"μ",
    "﴿":"﴿","﴾":"﴾","⋄":"⋄","⊸":"⊸","⊙":"⊙"
}
رموز_يونانية = {"λ":"λ","μ":"μ"}
أسماء_بديلة = {
    "دالة":"λ","λ":"λ","اطبع":"⎕","⎕":"⎕",
    "طالما":"μ","μ":"μ","لكل":"∀","∀":"∀","في":"∈","∈":"∈",
    "انقل":"⊸","⊸":"⊸","اقرأ":"⊙","⊙":"⊙"
}
عمليات_الجمع = {"+":"+","-":"-","⊕":"⊕"}
عمليات_الضرب = {"·":"·"}
عمليات_المقارنة = {"<":"<",">":">","=":"=","≠":"≠"}

def حلل_رموز(نص):
    رموز=[]; i=0; n=len(نص)
    while i<n:
        ح=نص[i]
        if ح.isspace(): i+=1; continue
        ر=أرقام_القيم.get(ح)
        if ر is not None:
            ق=0
            while i<n:
                د=أرقام_القيم.get(نص[i])
                if د is None: break
                ق=ق*10+د; i+=1
            رموز.append(("عدد",ق)); continue
        if ح=='"':
            i+=1; أ=[]
            while i<n and نص[i]!='"': أ.append(نص[i]); i+=1
            if i>=n: raise Exception("نص غير مغلق")
            i+=1; رموز.append(("نص","".join(أ))); continue
        ي=رموز_يونانية.get(ح)
        if ي is not None: رموز.append(("عملية",ي)); i+=1; continue
        if ح.isalpha() or ح=="_":
            ب=i; i+=1
            while i<n and (نص[i].isalpha() or نص[i].isdigit() or نص[i]=="_"): i+=1
            ك=نص[ب:i]; بد=أسماء_بديلة.get(ك)
            if بد is not None: رموز.append(("عملية",بد))
            else: رموز.append(("معرف",ك))
            continue
        ع=رموز_العمليات.get(ح)
        if ع is not None: رموز.append(("عملية",ع)); i+=1; continue
        raise Exception("رمز غير معروف: "+ح)
    return رموز

# ═══════════════════════════════════════════════════════════
# Parser
# ═══════════════════════════════════════════════════════════
def حلل_برنامج(رموز):
    ب=[]; i=0
    while i<len(رموز):
        بيان,i=حلل_بيان(رموز,i); ب.append(بيان)
    return ب

def حلل_بيان(رموز,i):
    if i>=len(رموز): raise Exception("بيان مفقود")
    ن,ق=رموز[i]
    if ن=="عملية" and ق=="⎕":
        i+=1; ت,i=حلل_تعبير(رموز,i); return ("اطبع",ت),i
    if ن=="عملية" and ق=="﴿":
        i+=1; بيانات=[]
        while True:
            if i>=len(رموز): raise Exception("﴾ مطلوبة")
            if رموز[i][1]=="﴾": i+=1; break
            بيان,i=حلل_بيان(رموز,i); بيانات.append(بيان)
            if i>=len(رموز): raise Exception("﴾ مطلوبة")
            if رموز[i][1]=="⋄": i+=1; continue
            if رموز[i][1]=="﴾": i+=1; break
            raise Exception("⋄ أو ﴾ مطلوبة")
        return ("كتلة",بيانات),i
    if ن=="معرف":
        اسم=ق
        if i+1<len(رموز) and رموز[i+1][0]=="عملية" and رموز[i+1][1]=="⊸":
            i+=2
            if i>=len(رموز) or رموز[i][0]!="معرف":
                raise Exception("اسم المصدر مطلوب بعد ⊸")
            مصدر=رموز[i][1]; i+=1
            return ("نقل",اسم,مصدر),i
        if i+1<len(رموز) and رموز[i+1][0]=="عملية" and (رموز[i+1][1]=="≔" or رموز[i+1][1]=="≡"):
            رمز=رموز[i+1][1]; i+=2; ت,i=حلل_تعبير(رموز,i)
            return ("عرف" if رمز=="≡" else "أسند",اسم,ت),i
    if ن=="عملية" and ق=="μ":
        i+=1; شرط,i=حلل_تعبير(رموز,i)
        if i>=len(رموز) or رموز[i][1]!=":": raise Exception(": مطلوبة")
        i+=1; جسم,i=حلل_بيان(رموز,i)
        return ("طالما",شرط,جسم),i
    if ن=="عملية" and ق=="∀":
        i+=1
        if i>=len(رموز) or رموز[i][0]!="معرف": raise Exception("اسم مطلوب بعد ∀")
        اسم=رموز[i][1]; i+=1
        if i>=len(رموز) or رموز[i][1]!="∈": raise Exception("∈ مطلوبة")
        i+=1; قائمة,i=حلل_تعبير(رموز,i)
        if i>=len(رموز) or رموز[i][1]!=":": raise Exception(": مطلوبة")
        i+=1; جسم,i=حلل_بيان(رموز,i)
        return ("لكل",اسم,قائمة,جسم),i
    raise Exception("بيان غير معروف")

def حلل_تعبير(رموز,i): return حلل_شرطي(رموز,i)

def حلل_شرطي(رموز,i):
    شرط,i=حلل_مقارنة(رموز,i)
    if i<len(رموز) and رموز[i][0]=="عملية" and رموز[i][1]=="؟":
        i+=1; صح,i=حلل_شرطي(رموز,i)
        if i>=len(رموز) or رموز[i][1]!=":": raise Exception(": مطلوبة")
        i+=1; خطأ,i=حلل_شرطي(رموز,i)
        return ("شرطي",شرط,صح,خطأ),i
    return شرط,i

def حلل_مقارنة(رموز,i):
    ي,i=حلل_جمع(رموز,i)
    while i<len(رموز) and رموز[i][0]=="عملية" and رموز[i][1] in عمليات_المقارنة:
        ع=رموز[i][1]; i+=1; م,i=حلل_جمع(رموز,i); ي=("مقارنة",ع,ي,م)
    return ي,i

def حلل_جمع(رموز,i):
    ي,i=حلل_ضرب(رموز,i)
    while i<len(رموز) and رموز[i][0]=="عملية" and رموز[i][1] in عمليات_الجمع:
        ع=رموز[i][1]; i+=1; م,i=حلل_ضرب(رموز,i); ي=("ثنائية",ع,ي,م)
    return ي,i

def حلل_ضرب(رموز,i):
    ي,i=حلل_عامل(رموز,i)
    while i<len(رموز) and رموز[i][0]=="عملية" and رموز[i][1] in عمليات_الضرب:
        i+=1; م,i=حلل_عامل(رموز,i); ي=("ثنائية","·",ي,م)
    return ي,i

def حلل_عامل(رموز,i):
    if i>=len(رموز): raise Exception("عامل مفقود")
    ن,ق=رموز[i]
    if ن=="عدد": return ("عدد",ق),i+1
    if ن=="نص": return ("نص",ق),i+1
    if ن=="عملية" and ق=="⊙":
        i+=1
        return ("اقرأ",),i
    if ن=="عملية" and ق=="λ":
        i+=1; معلمون=[]
        if i<len(رموز) and رموز[i][1]=="(":
            i+=1
            while True:
                if i>=len(رموز) or رموز[i][0]!="معرف":
                    raise Exception("اسم معامل مطلوب")
                معلمون.append(رموز[i][1]); i+=1
                if i>=len(رموز): raise Exception(") مطلوبة")
                if رموز[i][1]==",": i+=1; continue
                if رموز[i][1]==")": i+=1; break
        else:
            if i>=len(رموز) or رموز[i][0]!="معرف":
                raise Exception("معامل مطلوب بعد λ")
            معلمون.append(رموز[i][1]); i+=1
        if i>=len(رموز) or رموز[i][1]!=".": raise Exception(". مطلوبة")
        i+=1; جسم,i=حلل_تعبير(رموز,i)
        return ("دالة",معلمون,جسم),i
    if ن=="عملية" and ق=="⟨":
        i+=1; ع=[]
        if i<len(رموز) and رموز[i][1]=="⟩": return ("قائمة",ع),i+1
        while True:
            عنصر,i=حلل_تعبير(رموز,i); ع.append(عنصر)
            if i>=len(رموز): raise Exception("⟩ مطلوبة")
            if رموز[i][1]==",": i+=1; continue
            if رموز[i][1]=="⟩": i+=1; break
            raise Exception("فاصلة أو ⟩ مطلوبة")
        return ("قائمة",ع),i
    if ن=="عملية" and ق=="(":
        i+=1; ت,i=حلل_تعبير(رموز,i)
        if i>=len(رموز) or رموز[i][1]!=")": raise Exception(") مطلوبة")
        return ت,i+1
    if ن=="معرف":
        اسم=ق; i+=1
        if i<len(رموز) and رموز[i][1]=="(":
            i+=1; وس=[]
            if i<len(رموز) and رموز[i][1]==")": return ("استدعاء",اسم,وس),i+1
            while True:
                و,i=حلل_تعبير(رموز,i); وس.append(و)
                if i>=len(رموز): raise Exception(") مطلوبة")
                if رموز[i][1]==",": i+=1; continue
                if رموز[i][1]==")": i+=1; break
            return ("استدعاء",اسم,وس),i
        return ("متغير",اسم),i
    raise Exception("عامل غير متوقع")

# ═══════════════════════════════════════════════════════════
# Ownership Checker (Linear Logic ⊸)
# ═══════════════════════════════════════════════════════════
def get_used_vars(expr):
    ن=expr[0]
    if ن=="متغير": return {expr[1]}
    if ن in ["ثنائية","مقارنة"]:
        return get_used_vars(expr[2])|get_used_vars(expr[3])
    if ن=="شرطي":
        return get_used_vars(expr[1])|get_used_vars(expr[2])|get_used_vars(expr[3])
    if ن=="استدعاء":
        res=set()
        for arg in expr[2]: res|=get_used_vars(arg)
        return res
    if ن=="قائمة":
        res=set()
        for e in expr[1]: res|=get_used_vars(e)
        return res
    if ن=="دالة": return get_used_vars(expr[2])-set(expr[1])
    if ن=="اقرأ": return set()
    return set()

def get_stmt_used_vars(stmt):
    ن=stmt[0]
    if ن in ["أسند","عرف"]: return get_used_vars(stmt[2])
    if ن=="اطبع": return get_used_vars(stmt[1])
    if ن=="نقل": return {stmt[2]}
    if ن=="طالما": return get_used_vars(stmt[1])|get_stmt_used_vars(stmt[2])
    if ن=="لكل": return get_used_vars(stmt[2])|get_stmt_used_vars(stmt[3])
    if ن=="كتلة":
        res=set()
        for s in stmt[1]: res|=get_stmt_used_vars(s)
        return res
    return set()

def check_ownership(برنامج):
    consumed=set()
    def check_stmt(stmt):
        ن=stmt[0]
        if ن=="كتلة":
            for s in stmt[1]: check_stmt(s)
            return
        if ن=="طالما":
            used=get_used_vars(stmt[1])
            for v in used:
                if v in consumed:
                    raise Exception(f"خطأ ملكية ⊸: '{v}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه")
            check_stmt(stmt[2])
            return
        if ن=="لكل":
            used=get_used_vars(stmt[2])
            for v in used:
                if v in consumed:
                    raise Exception(f"خطأ ملكية ⊸: '{v}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه")
            check_stmt(stmt[3])
            return
        if ن=="نقل":
            هدف=stmt[1]; مصدر=stmt[2]
            if مصدر in consumed:
                raise Exception(f"خطأ ملكية ⊸: '{مصدر}' مستهلك بالفعل — لا يمكن نقله مرتين")
            consumed.add(مصدر)
            return
        if ن in ["أسند","عرف"]:
            اسم=stmt[1]
            consumed.discard(اسم)
            used=get_used_vars(stmt[2])
            for v in used:
                if v in consumed:
                    raise Exception(f"خطأ ملكية ⊸: '{v}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه")
            return
        if ن=="اطبع":
            used=get_used_vars(stmt[1])
            for v in used:
                if v in consumed:
                    raise Exception(f"خطأ ملكية ⊸: '{v}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه")
            return
        used=get_stmt_used_vars(stmt)
        for v in used:
            if v in consumed:
                raise Exception(f"خطأ ملكية ⊸: '{v}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه")
    for بيان in برنامج:
        check_stmt(بيان)
    return True

# ═══════════════════════════════════════════════════════════
# Helpers
# ═══════════════════════════════════════════════════════════
def get_free_vars(expr, bound):
    ن=expr[0]
    if ن=="متغير": return {expr[1]} if expr[1] not in bound else set()
    if ن in ["ثنائية","مقارنة"]:
        return get_free_vars(expr[2],bound)|get_free_vars(expr[3],bound)
    if ن=="دالة": return get_free_vars(expr[2], bound|set(expr[1]))
    if ن=="استدعاء":
        res=set()
        for arg in expr[2]: res|=get_free_vars(arg,bound)
        return res
    if ن=="شرطي":
        return get_free_vars(expr[1],bound)|get_free_vars(expr[2],bound)|get_free_vars(expr[3],bound)
    if ن=="قائمة":
        res=set()
        for e in expr[1]: res|=get_free_vars(e,bound)
        return res
    return set()

def استنتاج_نوع(expr, type_env):
    ن=expr[0]
    if ن=="نص": return "نص"
    if ن=="عدد": return "عدد"
    if ن=="قائمة": return "قائمة"
    if ن=="اقرأ": return "نص"
    if ن=="ثنائية":
        if expr[1]=="⊕": return "نص"
        if expr[1] in ["+","-","·"]: return "عدد"
        return "مجهول"
    if ن=="مقارنة": return "منطقي"
    if ن=="شرطي":
        t1=استنتاج_نوع(expr[2], type_env)
        if t1!="مجهول": return t1
        return استنتاج_نوع(expr[3], type_env)
    if ن=="متغير": return type_env.get(expr[1], "مجهول")
    if ن=="استدعاء":
        اسم=expr[1]
        if اسم in ["طول","حجم"]: return "عدد"
        if اسم=="ذيل": return "قائمة"
        if اسم=="رمز": return "عدد"
        if اسم=="نص": return "نص"
        if اسم=="عدد": return "عدد"
        if اسم in type_env: return type_env[اسم]
        return "مجهول"
    if ن=="دالة": return استنتاج_نوع(expr[2], type_env)
    return "مجهول"

ARG_REGS = ["rdi","rsi","rdx","rcx","r8","r9"]
_counters = {"cond":0,"empty":0,"copy":0,"loop":0}

# ═══════════════════════════════════════════════════════════
# Code Generator
# ═══════════════════════════════════════════════════════════
def compile_expr(expr, env, funcs, env_layout=None):
    ن=expr[0]
    if ن=="عدد": return [f"    mov rax, {expr[1]}"]
    if ن=="نص":
        byts=expr[1].encode('utf-8'); ln=len(byts)
        code=[f"    mov rdi, {8+ln}", "    call arena_alloc"]
        code.append(f"    mov qword [rax], {ln}")
        for k, b in enumerate(byts):
            code.append(f"    mov byte [rax + {8+k}], {b}")
        return code
    if ن=="اقرأ":
        _counters["copy"]+=1; k=_counters["copy"]
        return [
            "    xor rcx, rcx",
            f".rd_loop_{k}:",
            f"    lea rsi, [read_buf + rcx]",
            "    xor rdi, rdi",
            "    mov rdx, 1",
            "    xor rax, rax",
            "    push rcx",
            "    syscall",
            "    pop rcx",
            "    test rax, rax",
            f"    jz .rd_end_{k}",
            f"    mov al, [read_buf + rcx]",
            "    cmp al, 10",
            f"    je .rd_end_{k}",
            "    inc rcx",
            f"    jmp .rd_loop_{k}",
            f".rd_end_{k}:",
            "    mov rax, rcx",
            "    add rax, 8",
            "    mov rdi, rax",
            "    call arena_alloc",
            "    mov [rax], rcx",
            "    push rax",
            "    push rcx",
            "    lea rsi, [read_buf]",
            "    lea rdi, [rax + 8]",
            f".rd_copy_{k}:",
            "    test rcx, rcx",
            f"    jz .rd_cdone_{k}",
            "    mov al, [rsi]",
            "    mov [rdi], al",
            "    inc rsi",
            "    inc rdi",
            "    dec rcx",
            f"    jmp .rd_copy_{k}",
            f".rd_cdone_{k}:",
            "    pop rcx",
            "    pop rax",
        ]
    if ن=="متغير":
        if env_layout and expr[1] in env_layout:
            return [f"    mov rax, [r15 + {env_layout[expr[1]]}]"]
        if expr[1] in env["locals"]:
            return [f"    mov rax, [rbp - {env['locals'][expr[1]]}]"]
        if expr[1] in env["globals"]:
            return [f"    mov rax, [vars + {env['globals'][expr[1]]*8}]"]
        raise Exception(f"متغير غير معرف: {expr[1]}")
    if ن=="قائمة":
        elems=expr[1]; ln=len(elems)
        code=[f"    mov rdi, {8+ln*8}", "    call arena_alloc"]
        code.append("    push rax")
        code.append(f"    mov qword [rax], {ln}")
        for idx,el in enumerate(elems):
            code.extend(compile_expr(el,env,funcs,env_layout))
            code.append("    mov rcx, rax")
            code.append("    mov rbx, [rsp]")
            code.append(f"    mov [rbx + {8+idx*8}], rcx")
        code.append("    pop rax")
        return code
    if ن=="ثنائية":
        op=expr[1]
        left=compile_expr(expr[2],env,funcs,env_layout)
        right=compile_expr(expr[3],env,funcs,env_layout)
        if op=="⊕":
            _counters["copy"]+=1; k=_counters["copy"]
            code=left+["    push rax"]+right+["    mov r11, rax","    pop r10"]
            code += [
                "    push r10","    push r11",
                "    mov rax, [r10]","    add rax, [r11]","    add rax, 8",
                "    mov rdi, rax","    call arena_alloc","    mov r12, rax",
                "    mov rax, [r10]","    add rax, [r11]","    mov [r12], rax",
                "    mov rax, [r10]","    lea rsi, [r10 + 8]","    lea rdi, [r12 + 8]",
                f".cpy1_{k}:","    test rax, rax",f"    jz .c1d_{k}",
                "    mov cl, [rsi]","    mov [rdi], cl",
                "    inc rsi","    inc rdi","    dec rax",
                f"    jmp .cpy1_{k}",f".c1d_{k}:",
                "    mov rax, [r11]","    lea rsi, [r11 + 8]",
                f".cpy2_{k}:","    test rax, rax",f"    jz .c2d_{k}",
                "    mov cl, [rsi]","    mov [rdi], cl",
                "    inc rsi","    inc rdi","    dec rax",
                f"    jmp .cpy2_{k}",f".c2d_{k}:",
                "    pop r11","    pop r10","    mov rax, r12"]
            return code
        code=left+["    push rax"]+right+["    pop rbx"]
        if op=="+": code.append("    add rax, rbx")
        elif op=="-": code.append("    sub rbx, rax"); code.append("    mov rax, rbx")
        elif op=="·": code.append("    imul rax, rbx")
        return code
    if ن=="مقارنة":
        op=expr[1]
        left=compile_expr(expr[2],env,funcs,env_layout)
        right=compile_expr(expr[3],env,funcs,env_layout)
        code=left+["    push rax"]+right+["    pop rbx"]
        code.append("    cmp rbx, rax"); code.append("    mov rax, 0")
        if op=="<": code.append("    setl al")
        elif op==">": code.append("    setg al")
        elif op=="=": code.append("    sete al")
        elif op=="≠": code.append("    setne al")
        return code
    if ن=="شرطي":
        _counters["cond"]+=1; k=_counters["cond"]
        code=compile_expr(expr[1],env,funcs,env_layout)
        code.append("    cmp rax, 0"); code.append(f"    je .else_{k}")
        code.extend(compile_expr(expr[2],env,funcs,env_layout))
        code.append(f"    jmp .end_{k}"); code.append(f".else_{k}:")
        code.extend(compile_expr(expr[3],env,funcs,env_layout))
        code.append(f".end_{k}:")
        return code
    if ن=="دالة":
        params=expr[1]; body=expr[2]
        bound=set(env["globals"].keys())|set(params)
        free_vars=list(get_free_vars(body,bound))
        inner_label=f"func_{funcs['idx']}"; funcs['idx']+=1
        inner_fc=[f"{inner_label}:","    push rbp","    mov rbp, rsp"]
        stack_size=len(params)*8
        if stack_size%16!=0: stack_size+=8
        if stack_size==0: stack_size=16
        inner_fc.append(f"    sub rsp, {stack_size}")
        inner_env={"globals":env["globals"],"locals":{}}
        inner_env_layout={}
        for j,p in enumerate(params):
            inner_env["locals"][p]=(j+1)*8
            if j<len(ARG_REGS):
                inner_fc.append(f"    mov [rbp - {(j+1)*8}], {ARG_REGS[j]}")
        for ix,v in enumerate(free_vars): inner_env_layout[v]=8+ix*8
        inner_fc.extend(compile_expr(body,inner_env,funcs,inner_env_layout))
        inner_fc+=["    leave","    ret"]
        funcs['bodies'].extend(inner_fc)
        code=[]
        env_size=8+len(free_vars)*8
        code.append(f"    mov rdi, {env_size}")
        code.append("    call arena_alloc"); code.append("    push rax")
        code.append(f"    mov rcx, {inner_label}"); code.append("    mov [rax], rcx")
        for ix,v in enumerate(free_vars):
            offset=8+ix*8
            if env_layout and v in env_layout:
                code.append(f"    mov rcx, [r15 + {env_layout[v]}]")
            elif v in env["locals"]:
                code.append(f"    mov rcx, [rbp - {env['locals'][v]}]")
            elif v in env["globals"]:
                code.append(f"    mov rcx, [vars + {env['globals'][v]*8}]")
            code.append(f"    mov [rax + {offset}], rcx")
        code.append("    pop rax")
        return code
    if ن=="استدعاء":
        اسم=expr[1]; args=expr[2]
        if اسم=="طول":
            if len(args)!=1: raise Exception("طول تأخذ وسيطاً واحداً")
            code=compile_expr(args[0],env,funcs,env_layout)
            code.append("    mov rax, [rax]")
            return code
        if اسم=="رأس":
            if len(args)!=1: raise Exception("رأس تأخذ وسيطاً واحداً")
            _counters["empty"]+=1; k=_counters["empty"]
            code=compile_expr(args[0],env,funcs,env_layout)
            code += ["    mov rbx, [rax]","    test rbx, rbx",f"    jz .hemp_{k}",
                     "    mov rax, [rax + 8]",f"    jmp .hdne_{k}",
                     f".hemp_{k}:","    mov rax, 60","    mov rdi, 1","    syscall",
                     f".hdne_{k}:"]
            return code
        if اسم=="ذيل":
            if len(args)!=1: raise Exception("ذيل تأخذ وسيطاً واحداً")
            _counters["copy"]+=1; k=_counters["copy"]
            code=compile_expr(args[0],env,funcs,env_layout)
            code += ["    mov rcx, [rax]","    test rcx, rcx",f"    jz .taihemp_{k}",
                "    push rax","    dec rcx","    mov rdi, rcx",
                "    shl rdi, 3","    add rdi, 8",
                "    call arena_alloc","    mov r12, rax","    mov [rax], rcx",
                "    pop rsi","    add rsi, 16","    lea rdi, [r12 + 8]",
                f".tcopy_{k}:","    test rcx, rcx",f"    jz .tcd_{k}",
                "    mov rdx, [rsi]","    mov [rdi], rdx",
                "    add rsi, 8","    add rdi, 8","    dec rcx",
                f"    jmp .tcopy_{k}",f".tcd_{k}:",
                "    mov rax, r12",f"    jmp .taine_{k}",
                f".taihemp_{k}:","    mov rax, 60","    mov rdi, 1","    syscall",
                f".taine_{k}:"]
            return code
        if اسم=="حجم":
            if len(args)!=1: raise Exception("حجم تأخذ وسيطاً واحداً")
            code=compile_expr(args[0],env,funcs,env_layout)
            code.append("    mov rax, [rax]")
            return code
        if اسم=="ألحق":
            if len(args)!=2: raise Exception("ألحق تأخذ وسيطين")
            _counters["copy"]+=1; k=_counters["copy"]
            code=compile_expr(args[0],env,funcs,env_layout)
            code.append("    push rax")
            code.extend(compile_expr(args[1],env,funcs,env_layout))
            code += ["    mov r11, rax","    pop r10","    push r10","    push r11",
                "    mov rax, [r10]","    add rax, 1",
                "    mov rdi, rax","    shl rdi, 3","    add rdi, 8",
                "    call arena_alloc","    mov r12, rax",
                "    mov rax, [r10]","    add rax, 1","    mov [r12], rax",
                "    mov rax, [r10]","    lea rsi, [r10 + 8]","    lea rdi, [r12 + 8]",
                f".lcpy_{k}:","    test rax, rax",f"    jz .lcd_{k}",
                "    mov rcx, [rsi]","    mov [rdi], rcx",
                "    add rsi, 8","    add rdi, 8","    dec rax",
                f"    jmp .lcpy_{k}",f".lcd_{k}:","    mov [rdi], r11",
                "    pop r11","    pop r10","    mov rax, r12"]
            return code
        if اسم=="رمز":
            if len(args)!=2: raise Exception("رمز تأخذ وسيطين")
            code=compile_expr(args[0],env,funcs,env_layout)
            code.append("    push rax")
            code.extend(compile_expr(args[1],env,funcs,env_layout))
            code += ["    pop rbx","    mov rcx, rax",
                     "    mov rax, [rbx]","    cmp rcx, rax","    jge .ch_err",
                     "    movzx rax, byte [rbx + rcx + 8]","    jmp .ch_ok",
                     ".ch_err:","    mov rax, 60","    mov rdi, 1","    syscall",
                     ".ch_ok:"]
            return code
        if اسم=="نص":
            if len(args)!=1: raise Exception("نص تأخذ وسيطاً واحداً")
            _counters["copy"]+=1; k=_counters["copy"]
            code=compile_expr(args[0],env,funcs,env_layout)
            code += [
                "    mov rbx, 10","    mov rcx, 0",
                "    lea rdi, [num_buf + 31]",
                f".nts_{k}:",
                "    xor rdx, rdx","    div rbx",
                "    add dl, '0'","    dec rdi","    mov [rdi], dl",
                "    inc rcx","    test rax, rax",
                f"    jnz .nts_{k}",
                "    push rdi","    push rcx",
                "    mov rax, rcx","    add rax, 8",
                "    mov rdi, rax","    call arena_alloc",
                "    pop rcx","    mov [rax], rcx",
                "    pop rsi","    push rax",
                "    lea rdi, [rax + 8]",
                f".ntc_{k}:",
                "    test rcx, rcx",f"    jz .ntd_{k}",
                "    mov al, [rsi]","    mov [rdi], al",
                "    inc rsi","    inc rdi","    dec rcx",
                f"    jmp .ntc_{k}",f".ntd_{k}:",
                "    pop rax",
            ]
            return code
        if اسم=="عدد":
            if len(args)!=1: raise Exception("عدد تأخذ وسيطاً واحداً")
            _counters["copy"]+=1; k=_counters["copy"]
            code=compile_expr(args[0],env,funcs,env_layout)
            code += [
                "    mov rcx, [rax]","    lea rsi, [rax + 8]",
                "    mov rax, 0","    mov rbx, 10",
                f".std_{k}:",
                "    test rcx, rcx",f"    jz .sdd_{k}",
                "    movzx rdx, byte [rsi]",
                "    cmp dl, '0'",f"    jl .std_skip_{k}",
                "    cmp dl, '9'",f"    jg .std_skip_{k}",
                "    sub dl, '0'",
                "    imul rax, rbx","    add rax, rdx",
                f".std_skip_{k}:",
                "    inc rsi","    dec rcx",
                f"    jmp .std_{k}",f".sdd_{k}:",
            ]
            return code
        code=[]
        for arg in args:
            code.extend(compile_expr(arg,env,funcs,env_layout))
            code.append("    push rax")
        for j in range(len(args)-1,-1,-1):
            if j<len(ARG_REGS): code.append(f"    pop {ARG_REGS[j]}")
        if اسم in env["locals"]:
            code.append(f"    mov r10, [rbp - {env['locals'][اسم]}]")
        elif اسم in env["globals"]:
            code.append(f"    mov r10, [vars + {env['globals'][اسم]*8}]")
        else: raise Exception(f"دالة غير معرفة: {اسم}")
        code.append("    push r15")
        code.append("    mov r15, r10")
        code.append("    mov r10, [r10]")
        code.append("    call r10")
        code.append("    pop r15")
        return code
    raise Exception(f"تعبير غير مدعوم: {ن}")

# ═══════════════════════════════════════════════════════════
# Statement Compiler
# ═══════════════════════════════════════════════════════════
def compile_stmt(stmt, env, funcs, type_env):
    ن=stmt[0]
    if ن in ["أسند","عرف"]:
        اسم=stmt[1]
        if اسم not in env["globals"]: env["globals"][اسم]=len(env["globals"])
        code=compile_expr(stmt[2],env,funcs)
        code.append(f"    mov [vars + {env['globals'][اسم]*8}], rax")
        return code
    if ن=="نقل":
        هدف=stmt[1]; مصدر=stmt[2]
        if هدف not in env["globals"]: env["globals"][هدف]=len(env["globals"])
        if مصدر not in env["globals"]: raise Exception(f"متغير غير معرف: {مصدر}")
        code=[]
        code.append(f"    mov rax, [vars + {env['globals'][مصدر]*8}]")
        code.append(f"    mov [vars + {env['globals'][هدف]*8}], rax")
        code.append(f"    mov qword [vars + {env['globals'][مصدر]*8}], 0")
        return code
    if ن=="اطبع":
        e=stmt[1]
        code=compile_expr(e,env,funcs)
        نوع=استنتاج_نوع(e, type_env)
        code.append("    call print_str" if نوع=="نص" else "    call print_int")
        return code
    if ن=="كتلة":
        code=[]
        for s in stmt[1]:
            code.extend(compile_stmt(s,env,funcs,type_env))
        return code
    if ن=="طالما":
        _counters["loop"]+=1; k=_counters["loop"]
        code=[f".while_{k}:"]
        code.extend(compile_expr(stmt[1],env,funcs))
        code.append("    cmp rax, 0")
        code.append(f"    je .wend_{k}")
        code.extend(compile_stmt(stmt[2],env,funcs,type_env))
        code.append(f"    jmp .while_{k}")
        code.append(f".wend_{k}:")
        return code
    if ن=="لكل":
        _counters["loop"]+=1; k=_counters["loop"]
        متغير=stmt[1]; قائمة=stmt[2]; جسم=stmt[3]
        if متغير not in env["globals"]: env["globals"][متغير]=len(env["globals"])
        code=[]
        code.extend(compile_expr(قائمة,env,funcs))
        code += ["    mov r14, [rax]","    lea rbx, [rax + 8]",
                 f".fe_{k}:","    test r14, r14",f"    jz .feend_{k}",
                 "    mov rax, [rbx]",
                 f"    mov [vars + {env['globals'][متغير]*8}], rax",
                 "    push rbx","    push r14"]
        code.extend(compile_stmt(جسم,env,funcs,type_env))
        code += ["    pop r14","    pop rbx","    add rbx, 8","    dec r14",
                 f"    jmp .fe_{k}",f".feend_{k}:"]
        return code
    raise Exception(f"بيان غير مدعوم: {ن}")

# ═══════════════════════════════════════════════════════════
# Program Compiler
# ═══════════════════════════════════════════════════════════
def compile_program(برنامج):
    check_ownership(برنامج)
    asm=["global _start","section .bss",
         "    vars resq 256","    num_buf resb 32","    read_buf resb 256",
         "    arena_ptr resq 1","    arena_mem resb 262144","",
         "section .text"]
    asm += ["arena_alloc:","    mov rax, [arena_ptr]",
            "    add rdi, 15","    and rdi, -16",
            "    add [arena_ptr], rdi","    ret",""]
    asm += ["print_int:",
        "    push rax","    push rbx","    push rcx",
        "    push rdx","    push rsi","    push rdi",
        "    mov rbx, 10","    mov rcx, 0",
        "    lea rdi, [num_buf + 31]",
        ".piloop:",
        "    xor rdx, rdx","    div rbx","    add dl, '0'",
        "    dec rdi","    mov [rdi], dl",
        "    inc rcx","    test rax, rax","    jnz .piloop",
        "    mov rsi, rdi","    mov byte [rsi + rcx], 10","    inc rcx",
        "    mov rdi, 1","    mov rax, 1","    mov rdx, rcx","    syscall",
        "    pop rdi","    pop rsi","    pop rdx",
        "    pop rcx","    pop rbx","    pop rax","    ret",""]
    asm += ["print_str:",
        "    push rax","    push rdx","    push rsi","    push rdi",
        "    mov rsi, rax","    add rsi, 8","    mov rdx, [rax]",
        "    mov rdi, 1","    mov rax, 1","    syscall",
        "    mov rsi, nl_ptr","    mov rdx, 1",
        "    mov rdi, 1","    mov rax, 1","    syscall",
        "    pop rdi","    pop rsi","    pop rdx","    pop rax","    ret",""]
    asm += ["section .data","nl_ptr: db 10","section .text",""]
    asm += ["_start:","    lea rax, [arena_mem]","    mov [arena_ptr], rax",""]
    type_env={}
    for بيان in برنامج:
        if بيان[0] in ["عرف","أسند"]:
            type_env[بيان[1]]=استنتاج_نوع(بيان[2], type_env)
        elif بيان[0]=="نقل":
            type_env[بيان[1]] = type_env.get(بيان[2], "مجهول")
    var_map={}; funcs={"idx":0,"bodies":[]}; global_code=[]
    env={"globals":var_map,"locals":{}}
    for بيان in برنامج:
        global_code.extend(compile_stmt(بيان,env,funcs,type_env))
    asm += global_code+["","    mov rax, 60","    xor rdi, rdi","    syscall",""]
    asm += funcs["bodies"]
    return "\n".join(asm)

# ═══════════════════════════════════════════════════════════
# Test Helpers
# ═══════════════════════════════════════════════════════════
def build_and_run(name, source, expected, stdin_input=None):
    try:
        ر=حلل_رموز(source); ب=حلل_برنامج(ر); asm=compile_program(ب)
        with open(f"{name}.asm","w") as f: f.write(asm)
        subprocess.run(["nasm","-f","elf64",f"{name}.asm","-o",f"{name}.o"],
                       check=True,capture_output=True)
        subprocess.run(["ld",f"{name}.o","-o",name],check=True,capture_output=True)
        result=subprocess.run([f"./{name}"],capture_output=True,text=True,
                              timeout=5,input=stdin_input)
        out=result.stdout.strip()
        if out==expected: print(f"✅ ({name}): {out}")
        else:
            print(f"❌ ({name}): Expected '{expected}', Got '{out}'")
            sys.exit(1)
    except Exception as e:
        print(f"❌ ({name}): Error: {e}"); sys.exit(1)

def build_and_expect_error(name, source, expected_err_contains):
    try:
        ر=حلل_رموز(source); ب=حلل_برنامج(ر); asm=compile_program(ب)
        print(f"❌ ({name}): Expected compile error but succeeded!")
        sys.exit(1)
    except Exception as e:
        if expected_err_contains in str(e):
            print(f"✅ ({name}): رفض صحيح — {e}")
        else:
            print(f"❌ ({name}): Wrong error: {e}")
            sys.exit(1)

# ═══════════════════════════════════════════════════════════
# Phase 2: Calculator
# ═══════════════════════════════════════════════════════════
def run_calculator():
    برنامج_حاسبة = '⎕ "أدخل عدداً:"\nأ ≔ عدد(⊙)\n⎕ "أدخل عدداً ثانياً:"\nب ≔ عدد(⊙)\n⎕ "المجموع:"\n⎕ نص(أ + ب)'

    print("\n🔧 تجميع الآلة الحاسبة...")
    ر = حلل_رموز(برنامج_حاسبة)
    ب = حلل_برنامج(ر)
    asm = compile_program(ب)
    with open('calc.asm', 'w') as f:
        f.write(asm)
    subprocess.run(['nasm', '-f', 'elf64', 'calc.asm', '-o', 'calc.o'], check=True)
    subprocess.run(['ld', 'calc.o', '-o', 'calc'], check=True)
    print("✅ تم التجميع")

    print("\n🔢 التشغيل (5 + 3):")
    result = subprocess.run(['./calc'], capture_output=True, text=True, input="5\n3\n")
    print(result.stdout)

    expected = "أدخل عدداً:\nأدخل عدداً ثانياً:\nالمجموع:\n8"
    if result.stdout.strip() == expected:
        print("✅ الآلة الحاسبة تعمل!")
    else:
        print(f"❌ متوقع: {expected}")
        print(f"   فعلي: {result.stdout.strip()}")
        sys.exit(1)

# ═══════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════
if __name__ == '__main__':
    print("=" * 50)
    print("المرحلة 1: اختبارات المُجمّع")
    print("=" * 50)

    build_and_run("t_fact",
        "مضروب ≡ λن. (ن = 1) ؟ 1 : ن · مضروب(ن - 1)\n⎕ مضروب(5)", "120")
    build_and_run("t_foreach",
        "م ≔ 0\n∀ س ∈ ⟨1,2,3,4⟩ : م ≔ م + س\n⎕ م", "10")
    build_and_run("t_block",
        "ع ≔ 0\nن ≔ 1\nμ ع < 5 : ﴿ ن ≔ ن · 2 ⋄ ع ≔ ع + 1 ﴾\n⎕ ن", "32")
    build_and_run("t_move_valid",
        "أ ≔ ⟨1,2,3⟩\nب ⊸ أ\n⎕ طول(ب)", "3")
    build_and_run("t_move_str",
        'ن ≔ "مرحبا"\nم ⊸ ن\n⎕ م', "مرحبا")
    build_and_run("t_reassign_after_move",
        "أ ≔ ⟨1,2,3⟩\nب ⊸ أ\nأ ≔ ⟨4,5⟩\n⎕ طول(أ)", "2")
    build_and_expect_error("t_use_after_move",
        "أ ≔ ⟨1,2,3⟩\nب ⊸ أ\n⎕ طول(أ)", "مستهلك")
    build_and_expect_error("t_double_move",
        "أ ≔ ⟨1,2,3⟩\nب ⊸ أ\nج ⊸ أ", "مستهلك")
    build_and_expect_error("t_move_in_block",
        "أ ≔ ⟨1,2,3⟩\n﴿ ب ⊸ أ ⋄ ⎕ طول(أ) ﴾", "مستهلك")
    build_and_run("t_read_echo",
        "س ≔ ⊙\n⎕ س", "مرحبا", stdin_input="مرحبا")
    build_and_run("t_read_size",
        "س ≔ ⊙\n⎕ حجم(س)", "10", stdin_input="مرحبا")
    build_and_run("t_char",
        'س ≔ "abc"\n⎕ رمز(س, 0)', "97")
    build_and_run("t_char2",
        'س ≔ "مرحبا"\n⎕ رمز(س, 0)', "217")
    build_and_run("t_num2str",
        "⎕ نص(42)", "42")
    build_and_run("t_str2num",
        '⎕ عدد("123")', "123")
    build_and_run("t_roundtrip",
        "⎕ عدد(نص(99))", "99")

    print("\n🎉 المرحلة 1: جميع الاختبارات نجحت!")

    print("\n" + "=" * 50)
    print("المرحلة 2: الآلة الحاسبة التفاعلية")
    print("=" * 50)
    run_calculator()

    print("\n" + "=" * 50)
    print("🏆 جميع المراحل نجحت!")
    print("=" * 50)