pub mod obj {
    pub fn parse(content: &str) -> Vec<f32> {
        let mut res: Vec<f32> = Vec::new();

        let lines = content.lines();
        for line in lines {
            let mut iter = line.split_whitespace();
            let Some(identifier) = iter.next() else {
                continue;
            };
            match identifier {
                "v" => continue,
                _ => continue,
            }
        }

        return res;
    }
}
