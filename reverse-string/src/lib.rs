pub fn reverse(input: &str) -> String {
   // 方法一：将 字符串转为字符数组后， rev 变为Rev集合， 然后通过内部迭代，构造collect为string
   input.chars().rev().collect()

   // 方法二： 使用unicode_segmentation：1.6.0 库
   // input.graphemes(true).rev().collect()
}