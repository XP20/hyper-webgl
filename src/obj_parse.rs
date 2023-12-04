pub mod obj {
    pub fn parse(content: &str) -> (Vec<f32>, Vec<u16>) {
        let mut verts: Vec<f32> = Vec::new();

        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut texcoords: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let lines = content.lines();
        for line in lines {
            let mut iter = line.split_whitespace();
            let Some(identifier) = iter.next() else {
                continue;
            };
            match identifier {
                "v" => {
                    let x = iter.next().unwrap().parse::<f32>().unwrap();
                    let y = iter.next().unwrap().parse::<f32>().unwrap();
                    let z = iter.next().unwrap().parse::<f32>().unwrap();
                    // let w = iter.next().unwrap_or("1.0").parse::<f32>().unwrap();
                    positions.extend_from_slice(&[x, y, z]);
                },
                "vn" => {
                    let i = iter.next().unwrap().parse::<f32>().unwrap();
                    let j = iter.next().unwrap().parse::<f32>().unwrap();
                    let k = iter.next().unwrap().parse::<f32>().unwrap();
                    normals.extend_from_slice(&[i, j, k]);
                },
                "vt" => {
                    let u = iter.next().unwrap().parse::<f32>().unwrap();
                    let v = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    // let w = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    texcoords.extend_from_slice(&[u, v]);
                },
                "f" => {
                    let x = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap() - 1;
                    let y = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap() - 1;
                    let z = iter.next().unwrap().split('/').next().unwrap().parse::<u16>().unwrap() - 1;
                    if let Some(item) = iter.next() {
                        let w = item.split('/').next().unwrap().parse::<u16>().unwrap() - 1;
                        indices.extend_from_slice(&[x, y, z, x, z, w]);
                    } else {
                        indices.extend_from_slice(&[x, y, z]);
                    }
                },
                _ => continue,
            }
        }

        verts = positions.clone();        

        return (verts, indices);
    }
}
