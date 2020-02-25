# 赛况

记录小型足球比赛的结果。基于输入文件，其中包含哪个队与哪个队对战
创建一个类似下表的文件：

```text
Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
```

> Team 列占31个字符位

其中：

- MP: 比赛场数
- W: 胜利场数
- D: 打平场数
- L: 输球场数
- P: 最终得分

一场胜利可为球队赢得3分。平局赢得1。亏损赢得0。
输出结果按得分降序排列。如果出现平局，则按字母顺序对球队进行排序。

## 输入

输入类似：

```text
Allegoric Alaskans;Blithering Badgers;win
Devastating Donkeys;Courageous Californians;draw
Devastating Donkeys;Allegoric Alaskans;win
Courageous Californians;Blithering Badgers;loss
Blithering Badgers;Devastating Donkeys;loss
Allegoric Alaskans;Courageous Californians;win
```

解释：

如第一对的比赛结果：

```text
Allegoric Alaskans;Blithering Badgers;win
```

意识是Allegoric Alaskans 打赢了Blithering Badgers。

下面的输入：

```text
Courageous Californians;Blithering Badgers;loss
```

意思是Blithering Badgers 打赢了Courageous Californians。

下面的输入：

```text
Devastating Donkeys;Courageous Californians;draw
```

意思是Devastating Donkeys和Courageous Californians平局。
