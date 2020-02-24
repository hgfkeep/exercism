#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
/**
 * 思路： 计算当前局的得分和额外加分次数。
 */
#[derive(Debug)]
pub struct BowlingGame {
    /**
     * 每局击球得分情况, 不算额外加分。（当局得分，额外加分次数open(0)/strike(2)/spare(1)，需要补球），
     * 需要第三个值，主要是防止某个frame， 第一次+补球的积分<10， 无法判断下次给的分数到底是当前frame的还是下个frame的。
     * 步骤：
     *  1. 设置当前局分数：（当局得分，额外记分次数, 需要补球）
     *  2. 计算前面最多两次的分数，通过额外几分次数判断前次是否需要加上本次的得分；
     */
    frame: Vec<(u16, u8, bool)>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let l = self.frame.len();
        //记录了完整的10局，游戏结束了, 最好情况下，第10局后补2次球
        if l >= 10 && self.frame[9].1 == 0 && !self.frame[9].2 {
            return Err(Error::GameComplete);
        }
        //先计算本次得分情况
        match pins {
            10 => {
                if let Some(f) = self.frame.last() {
                    if f.2 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                self.frame.push((10, 2, false));
            }
            _x if pins < 10 => {
                if let Some((score, bonus_time, need_spare)) = self.frame.pop() {
                    if need_spare {
                        // 补球, 弹出并合并
                        let total = score + pins;
                        if total == 10 {
                            self.frame.push((10, 1, false));
                        } else if total < 10 {
                            self.frame.push((total, 0, false));
                        } else {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                    } else {
                        //当局首次击球，需要将pop的frame 重新加入，它们是两局
                        self.frame.push((score, bonus_time, need_spare));
                        self.frame.push((pins, 0, true));
                    }
                    println!("spare: {:?}", self.frame);
                } else {
                    // 如果是第一击, 还需要
                    self.frame.push((pins, 0, true));
                }
            }
            _ => {
                return Err(Error::NotEnoughPinsLeft);
            }
        }
        //上述操作可能会导致len变化，需要重新覆盖和计算长度
        let l = self.frame.len();

        //当前roll的前两个roll， 计算n-2得分
        if l > 2 && self.frame[l - 3].1 > 0 {
            self.frame[l - 3] = (
                self.frame[l - 3].0 + pins,
                self.frame[l - 3].1 - 1,
                self.frame[l - 3].2,
            );
        }

        //当前roll的前一个roll， 计算n-1得分
        if l > 1 && self.frame[l - 2].1 > 0 {
            self.frame[l - 2] = (
                self.frame[l - 2].0 + pins,
                self.frame[l - 2].1 - 1,
                self.frame[l - 2].2,
            );
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let l = self.frame.len();

        //结束了, 以frame[9] "完整了吗?"为判断依据。
        if l >= 10 && self.frame[9].1 == 0 && !self.frame[9].2 {
            Some(
                (0..10)
                    .map(|x| self.frame[x])
                    .fold(0_u16, |sum, (score, _, _)| sum + score),
            )
        } else {
            None
        }
    }
}
