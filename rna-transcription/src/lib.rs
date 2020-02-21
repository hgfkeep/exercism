#[derive(Debug, PartialEq)]
pub struct DNA {
    data: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    data: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        //判断dna字符串中包含非法字符
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'G' | 'C' | 'T' => (),
                _ => return Err(i),
            }
        }
        Ok(DNA {
            data: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let s: String = self
            .data
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                _ => 'A',
            })
            .collect();
        RNA::new(s.as_str()).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'U' => (),
                _ => return Err(i),
            }
        }
        Ok(RNA {
            data: rna.to_string(),
        })
    }
}
