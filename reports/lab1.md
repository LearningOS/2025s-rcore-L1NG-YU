# 荣誉准则


在完成本次实验的过程（含此前学习的过程）中，我曾未与其他人做过交流
        

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    - 微信群聊中助教们对其他人就ch3方面相关问题的解释
    - 知乎上 @香草美人 的rcore相关专栏

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计

# 1.实现的功能
开一个常量MAX_SYSCALL_NUM来表示syscall的最大值

在TaskControlBlock（./src/task/task.rs）里加一个syscall_times的数组来记录所有syscall的调用次数,同时在TaskManager（./src/task/mod.rs）里添加两个函数：
- increase_syscall_times(syscall_id) 用于实现调用次数的自增
- get_single_syscall_time(syscall_id) 用于查询某个syscall的调用次数

在 ./src/syscall 中：
-  ./mod.rs syscall函数中实现调用syscall时的调用次数的自增
-  ./process.rs 对不同trace_request按要求进行处理


---

# 2.简答作业
## 2.1 第一部分

Rustsbi 版本为: 0.2.2

出现报错:
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```
原因：
ch2b_bad_address.rs 由于除0错误触发异常退出
ch2b_bad_instructions.rs 在用户态非法使用指令sretch2b_bad_register.rs 在用户态非法使用指令csrr

## 2.2第二部分
### 2.2.1
sp代表的值：内核栈的栈顶

__restore 的使用情景:

- 从系统调用和异常返回时, 恢复要返回的用户态的上下文信息
- 任务切换时, 恢复要切换的任务的上下文信息

### 2.2.2
```asm
ld t0, 32*8(sp) # 内核栈 32*8(sp) 处存储了原 sstatus 寄存器的值, 将其读取到 t0
ld t1, 33*8(sp) # 内核栈 32*8(sp) 处存储了原 sepc 寄存器的值, 将其读取到 t1
ld t2, 2*8(sp) # 内核栈 32*8(sp) 处存储了原 sscratch 寄存器的值, 将其读取到 t2
csrw sstatus, t0 # 将 t0中原 sstatus 寄存器的值读取到 sstatus
csrw sepc, t1 # 将 t0中原 sepc 寄存器的值读取到 sepc
csrw sscratch, t2 # 将 t0中原 sscratch 寄存器的值读取到 sscratch
```
### 2.2.3

- 跳过x2是因为x2对应的用户栈指针保存到了sscratch
存器, 不需要从内核栈中进行恢复

- 跳过x4是因为并没有使用过x4

### 2.2.4
sp指向用户栈, sscratch指向内核栈

### 2.2.5
`sret`后发生了状态切换, 执行该指令后, PC设置为 `sepc`寄存器的值。`sepc`存储着产生中断或异常前的指令地址，因此返回原始代码。

### 2.2.6
`sp`, `sscratch`寄存器的内容被交换, sp保存了原sscratch中的内核栈指针, sscratch保存了原sp中的用户栈栈指针

### 2.2.7
`ecall`指令



