pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    /**
     * ⚠️注意：
     * 1. usize不要使用减法
     * 2. 通过使用enumerate方法构造usize迭代器
     * 3. 合理的通过unwrap_or规避if条件判断
     **/

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = Vec::new();
        res.push(vec![1_u32]);
        for (i, _) in (0..self.rows-1).enumerate() {
            let mut t: Vec<u32> = Vec::new();
            t.push(1);
            let pre = res.get(i).unwrap();
            for (j, _) in (0..pre.len()).enumerate() {
                let x = pre.get(j).unwrap_or(&0);
                let y = pre.get(j+1).unwrap_or(&0);
                t.push(x + y);
            }
            res.push(t);
        }
        res
    }
}
