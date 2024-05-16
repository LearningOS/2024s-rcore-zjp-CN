《荣誉准则》

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

---

# Q1

> 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 
> - 需要回收的资源有哪些？

需要回收改进程内所有线程以下资源：
* 用户态的资源：
  * tid （由 RecycleAllocator 回收）
  * trap_cx、ustack、program code/data section 等内核和用户地址空间中被映射的内存资源（由 MemorySet 回收）
  * MemorySet 内部自身的资源（主要是一些连续页表的映射 MapArea）
  * mutex、semaphore、condvar 等用于并发机制的资源
* file descriptors （由实现了 File trait 的具体类型的 Drop impl 回收）
* 子进程的上述资源


> - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

* 一些存放 `Arc<ProcessControlBlock>` 的地方：这些地方需要直接（主动）回收
  * PID2PCB
  * 主线程 upgrade 的进程
  * TaskManager 的 ready_queue
  * Processor 的 current
  * TimerCondVar 的 task 
* ProcessControlBlock 内部引用 TaskControlBlock 的字段：这些地方不需要直接回收，因为当 `Arc<ProcessControlBlock>` 计数为 0 被回收时，
  会自动回收它们
  * Condvar 的 wait_queue
  * Mutex 的 wait_queue
  * Semaphore 的 wait_queue

# Q2

> 对比以下两种 Mutex.unlock 的实现，二者有什么区别？这些区别可能会导致什么问题？

它们的区别在于唤醒等待队列中的下个线程时，是否让锁不被持有：
* `Mutex2` 在唤醒时，保持锁被持有的状态，达到了互斥访问的目的；也就是释放锁的线程将锁直接移交给这次唤醒的线程 
* `Mutex1` 在唤醒时，把锁住的状态改成了解除了，那么如果此时有线程尝试去获取锁，那么直接获得锁，最终可能造成唤醒的线程和
  这个直接获得锁的线程同时进入临界区，造成竞态条件。

对于 `ch8b_phil_din_mutex` 测例，结果为：

```text
Mutex2: good 始终最多只有 2 个线程 EATING
'-' -> THINKING; 'x' -> EATING; ' ' -> WAITING
#0:  --------                      xxxxxxxx -----------                   xxxxx ------xxxxxxx --xxxxx
#1:  --- xxxxxxx ---       xxxxxxx ---------- x ---               xxxxxxx
#2:  -----             xxx ----------xxx -----             xxxxxx --------------xxxxx
#3:  ------ xxxxxxxxxxx-------xxxxxx ---------     xxxxxxx --     xxxxxxxxxx
#4:  -------           xx ------            xxxxxx  --     xxxxx  ------        xxx

Mutex1: bad 因为有 3 个线程同时 EATING
'-' -> THINKING; 'x' -> EATING; ' ' -> WAITING
#0: ----------                xxxxxxxx  -----------                     xxxxx ----- xxxxxx --- xxxx
#1: ---- xxxxxxx  -- xxxxxxxx -----------    xx ---            xxxxxxxx
#2:  -----                xxxx---------- xxx ----       xxxxxxx-------------  xxxx
#3:  -------   xxxxxxxxxx ------- xxxxx -------- xxxxxx --- xxxxxxxxxx
#4:  ------               x   ------    xxxxxx  --      xxxxx  --------xxx
```
