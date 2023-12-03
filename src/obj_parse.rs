pub mod obj {
    pub fn parse(content: &str) -> (Vec<f32>, Vec<u16>) {
        let mut verts: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let lines = content.lines();
        for line in lines {
            let mut iter = line.split_whitespace();
            let Some(identifier) = iter.next() else {
                continue;
            };
            match identifier {
                "v" => {
                    let x = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    let y = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    let z = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    let _w = iter.next().unwrap_or("1.0").parse::<f32>().unwrap();
                    verts.extend_from_slice(&[x, y, z]);
                },
                "f" => {
                    let x = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap();
                    let y = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap();
                    let z = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap();
                    if let Some(item) = iter.next() {
                        let w = item.split('/').next().unwrap().parse::<u16>().unwrap();
                        indices.extend_from_slice(&[x, y, z, x, z, w]);
                    } else {
                        indices.extend_from_slice(&[x, y, z]);
                    }
                },
                _ => continue,
            }
        }

        return (verts, indices);
    }
}
