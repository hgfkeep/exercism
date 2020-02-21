#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(Vec::from(scores))
    }

    pub fn scores(&self) -> &[u32] {
        self.0.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        let x = self.0.last();
        x.map(|x| * x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut x = self.0.clone();
        x.sort();
        x.last().map(|x| *x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut x = self.0.clone();
        x.sort();
        x.reverse();
        x.iter().take(3).map(|x| *x).collect()
    }
}
