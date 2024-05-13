《荣誉准则》

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

---

# Q1

>stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。
>
> 实际情况是轮到 p1 执行吗？为什么？

取决于如何处理溢出。

如果采用 wrap-around add，那么 p2.stride = 4，p2 执行。

如果不继续 add，并采用 FIFO 策略，则 p1 执行。

# Q2

>我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
>
>为什么？尝试简单说明（不要求严格证明）。

这里的 `STRIDE_{MIN, MAX}` 似乎指单步 pass，如果不是，那么题目有问题。

已知：
* `STRIDE_MAX = BigStride / p1`
* `STRIDE_MIN = BigStride / p2`
* `p1 >= p2 >= 2`

那么：
* `1/p1 <= 1/p2 <= 1<2`，即 `1/p2 - 1/p2 <= 1/2 - 1/p1`
* 记 `1/p2 - 1/p1` 为 p，那么 `p < 1/2`
* `STRIDE_MAX - STRIDE_MIN = BigStride * (1/p2 - 1/p1) = BigStride * p < BigStride/2`

# Q3

>已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 `BinaryHeap<Stride>` 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。


```rust
fn main() {
    assert!(!(Stride(125) < Stride(255)));
    assert!(Stride(129) < Stride(255));
    assert!(Stride(10) < Stride(100));
    assert!(!(Stride(100) < Stride(10)));
}

use core::cmp::Ordering;
const BIG_STRIDE: u8 = 255;
struct Stride(u8);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let [a, b] = [self.0, other.0];
        if a == 255 || b == 255 {
            let diff = if a < b { b - a } else { a - b };
            Some(diff.cmp(&(BIG_STRIDE / 2)))
        } else {
            Some(a.cmp(&b))
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
```
