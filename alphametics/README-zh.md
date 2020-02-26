# Alphametics

写来解决字母算术谜题的功能。

[Alphametics](https://en.wikipedia.org/wiki/Alphametics) 是一个将单词中的字母将替换为数字的迷题。

例如 `SEND + MORE = MONEY`:

```text
  S E N D
  M O R E +
-----------
M O N E Y
```

将其替换为有效数字可得到：

```text
  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
```

这是正确的，因为每个字母都用不同的数字替换，并且单词，翻译成数字，然后得出有效的和。
每个字母必须代表一个不同的数字，并且最前面的字母所代表的数字不能为零。
编写函数来解决字母拼写难题。
