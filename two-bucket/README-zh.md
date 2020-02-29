# Two Bucket

给定两个不同尺寸的桶，演示了如何通过所述桶之间策略性转移的流体，精确测量得到流体的体积升。

由于此数学问题针对对于单个例子比较好解决，所以期望使用一个通用的解决方案来通过测试用例。

为了帮助您，测试提供了首先要填充的存储桶。
这意味着，当从较大的存储桶开始装满时，不允许在任何时候使较小的桶装满而较大的存储桶为空（也就是相反的起点）；那将无法比较两种方法的目的！

To help, the tests provide you with which bucket to fill first. That means, when starting with the larger bucket full, you are NOT allowed at any point to have the smaller bucket full and the larger bucket empty (aka, the opposite starting point); that would defeat the purpose of comparing both approaches!

程序的输入：

- 桶1的大小
- 桶2的大小
- 达到所需的升数
- 哪个桶先满

程序需要确定：

- 达到所需升数（包括第一次填充）所需的“移动”总步数
- 哪个水桶包含目标测量值的水（假设这是水桶A）--- 桶1还是桶2
- 另一个桶（桶B）还剩多少公升

注意：任何时候，对一两个桶中任何一个进行任何更改，都视为一（1）步操作。

例如：

第一桶最多可容纳7升，第二桶最多可容纳11升。
假设在给定的步骤中，第一个桶盛有7升，而第二个桶盛有8升（7,8）。
如果清空第一个存储桶，而对第二个存储桶没有任何改变，则分别剩下0升和8升（0,8），则算作1步“移动”。
相反，如果您从第一个存储桶中倒入第二个存储桶中，直到第二个存储桶装满，则在第一个存储桶中有4升，在第二个存储桶中有11升（4,11），那么这也算是一步“移动”。

总而言之，唯一有效的举动是：

- 从一个桶倒到另一个桶
- 清空一个水桶，另一个桶不变
- 装满一个水桶，另一个桶不变

## 题解

此题需要使用欧几里德定理，即 am + bn = x 有解，当前仅x可以被a和b的最大公约数 `gcd(a,b)` 整除， 即 `x % gcd(a,b) = 0`。