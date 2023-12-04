pub mod obj {
    pub fn parse(content: &str) -> (Vec<f32>, Vec<u16>) {
        let mut verts: Vec<Vec<f32>> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut normals: Vec<Vec<f32>> = Vec::new();
        let mut texcoords: Vec<Vec<f32>> = Vec::new();

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
                    verts.push(vec![x, y, z]);
                },
                "vn" => {
                    let i = iter.next().unwrap().parse::<f32>().unwrap();
                    let j = iter.next().unwrap().parse::<f32>().unwrap();
                    let k = iter.next().unwrap().parse::<f32>().unwrap();
                    normals.push(vec![i, j, k]);
                },
                "vt" => {
                    let u = iter.next().unwrap().parse::<f32>().unwrap();
                    let v = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    // let w = iter.next().unwrap_or("0.0").parse::<f32>().unwrap();
                    texcoords.push(vec![u, v]);
                },
                "f" => {
                    let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) else {
                        panic!("Invalid .obj face format");
                    };
                    // Handle 3 or 4 args
                    // Handle with vt and vn or without
                    // Handle with vt and without
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

        let res = verts.into_iter().flatten().collect();
        return (res, indices);
    }
}
