《荣誉准则》

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

# Q1


rustsbi 0.3.0-alpha.2


* `ch2b_bad_address` 0x0 地址的读写权限属于 pmp01，为 M 权级，而测试代码运行在 U 权级，
只允许读写 0x88000000..0x00000000 区域的内存，因此，在 0x0 地址写入会触发 PageFault。

```text
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
```

* `ch2b_bad_instructions` 试图调用 `sret` 指令，该指令至少需要 S 权级，因此在 U 模式下调用
会触发 IllegalInstruction。

* `ch2b_bad_register` 试图读取 sstatus CSR，该寄存器至少需要 S 权级，因此在 U 模式下访问
会触发 IllegalInstruction。

# Q2

## Q2.1

* 刚进入 __restore 时，a0 代表了指向 `TrapContext` 的指针
* __restore 的两种使用情景：
  * 由于汇编脚本 trap.S 中，__restore 是紧接 __alltraps 的，所以，在调用 trap_handler 
    处理完陷入之后，就会调用 __restore，通过 TrapContext 恢复原任务的用户态的上下文和栈，
    并从内核态回到用户态；
  * __restore 在 Rust 代码中，也作为 TaskContext 的 ra 字段的值，并且在 __switch 调用中，
    切换完两个任务的上下文之后，ra 作为 ret 的目标地址，会调用 __restore，切换到新任务的上下文，
    从内核栈切到新的用户栈，回到用户态。

## Q2.2

特殊处理了 sstatus、sepc 和 sscratch 寄存器，它们的值是从 TrapContext 中恢复的。

对进入用户态的意义:
* 保持 sstatus 的 SPP 字段为 0 来表示陷入前处于 U 模式
* sepc 用于控制 sret 后，应用程序接下来执行的指令
* sscratch 用于获取用户栈指针，接下来准备交换内核栈和用户栈的栈指针

## Q2.3

跳过 x2 (sp) 是因为此时 sp 指向内核栈，还需要释放内核栈帧才能交换回用户栈指针。

跳过 x4 (tp) 是因为目前为单线程，它不会改变，暂时不需要保存和恢复。

## Q2.4


L60 `csrrw sp, sscratch, sp` 用于交换内核栈和用户栈的栈指针，该指令之后：
* `sp` 指向用户栈的栈顶
* `sscratch` 指向内核栈的栈顶

## Q2.5

__restore：中发生状态切换在 `sret` 指令，它会进入用户态是因为这条指令所做的事情，有


* `pc <- CSRs[spec]` 把 sret 之后的执行权转交给 spec CSR 中的地址，接下来的程序流应该从应用程序开始
* 将机器权级切换成 sstatus.SPP 这个位表示的权级，此时它为 0，那么权级切换到了 U

## Q2.6


L13 `csrrw sp, sscratch, sp` 用于交换内核栈和用户栈的栈指针，该指令之后：
* `sp` 指向内核栈的栈顶
* `sscratch` 指向用户栈的栈顶

## Q2.7

从 U 态进入 S 态是 `ecall` 指令主动触发的。（当然导致异常和中断的指令也会触发权级切换）
