# Triangle

确定三角形是等边的，等腰的还是不等边的。

等边三角形的三条边都相等
等腰三角形的至少两个边的长度相同。（有时指定为正好两边长度相同，在本练习中等腰三角形至少两个边相等。等边三角形是特殊的等腰三角形）

不等边三角形具有不同长度的所有侧面。

## Note

三角形必须满足，所有边的长度必须大于0，并且任何两侧的长度之和必须大于或等于第三边的长度。请参阅[三角不等式](https://en.wikipedia.org/wiki/Triangle_inequality)。

## 深入挖掘

两侧长度之和等于第三个称为退化三角形-面积为零，变成一条线。随意添加自己的代码/测试来检查退化的三角形。

## Rust中的三角形

- [Result](https://doc.rust-lang.org/std/result/index.html)

三角形实现可以采用多种形式。根据您采用的方法，以下一些主题可能会对您有所帮助。

- [Enums](https://doc.rust-lang.org/book/2018-edition/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

或者，也许你会想出一个不使用它们的办法！

## Non-integer lengths
基础练习测试边均为整数的三角形的标识。但是，某些三角形不能用纯整数表示。一个简单的例子是直角三角形（等边三角形，其等边相隔90度），其等边均长为1。其斜边为2的平方根，这是一个无理数：没有简单的乘法可以表示此数字作为整数。

重写分析函数以处理整数和浮点情况将很繁琐，尤其是对于所有潜在的整数和浮点类型都这样做很麻烦：给定带符号宽度和带符号宽度的无符号变体8、16、32、64和128，即使考虑浮点数，也将是10个基本上相同代码的重新实现！

有更好的方法：[泛型](https://doc.rust-lang.org/stable/book/2018-edition/ch10-00-generics.html)。通过将Triangle重写为`Triangle<T>`，可以编写一次代码，并将生成所有这些特化的工作移交给编译器。请注意，为了使用数学运算，您需要将通用类型限制为使用特征支持这些运算的类型。

您可以运行一些额外测试，这些测试可以对浮点数进行测试。要启用它们，请使用`--feature`运行测试，如下所示：

```bash
cargo test --features generic
```

## 开发代码

使用如下命令执行测试:

```bash
$ cargo test
```

除了第一个测试用例，其余测试用例均默认忽略。当通过了第一个测试用例，可以打开位于tests文件夹中的测试用例源代码文件，移除`#[ignore]`标签，再次测试代码，通过测试用例。反复操作直到通过所有测试用例。

如果想不编辑测试源代码文件，直接运行所有的测试用例，可以使用如下命令：

```bash
$ cargo test -- --ignored
```

运行某个测试用例，例如`some_test`可以使用命令:

```bash
$ cargo test some_test
```

运行某个ignore的测试用例，可以使用命令:

```bash
$ cargo test some_test -- --ignored
```
