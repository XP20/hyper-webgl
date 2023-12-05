pub mod obj {
    pub fn parse(content: &str) -> (Vec<f32>, Vec<u16>) {
        let mut verts: Vec<Vec<f32>> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut positions: Vec<Vec<f32>> = Vec::new();
        let mut normals: Vec<Vec<f32>> = Vec::new();
        let mut texcoords: Vec<Vec<f32>> = Vec::new();

        let mut cunt = 0;

        let lines = content.lines();
        for line in lines {
            let mut iter = line.trim().split_whitespace();
            let Some(identifier) = iter.next() else {
                continue;
            };
            match identifier {
                "v" => {
                    let x = iter.next().unwrap().parse::<f32>().unwrap();
                    let y = iter.next().unwrap().parse::<f32>().unwrap();
                    let z = iter.next().unwrap().parse::<f32>().unwrap();
                    // let w = iter.next().unwrap_or("1.0").parse::<f32>().unwrap();
                    positions.push(vec![x, y, z]);
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
                    let specs = iter.collect::<Vec<&str>>().clone();
                    if specs.len() >= 3 {
                        let mut i = 0;
                        let mut v0 = 0;
                        while i < specs.len() {
                            let spec = specs[i];
                            let [v, vt, vn] = spec.split("/")
                                .map(|x| x.parse::<u16>().unwrap_or(0))
                                .collect::<Vec<u16>>()[..]
                            else {
                                panic!("obj_loader unexpected error");
                            };

                            let pos = positions[(v - 1) as usize].clone();
                            let tex = if vt == 0 {
                                vec![0.0, 0.0]
                            } else {
                                texcoords[(vt - 1) as usize].clone()
                            };
                            let norm = if vn == 0 {
                                vec![0.0, 0.0, 0.0]
                            } else {
                                normals[(vn - 1) as usize].clone()
                            };

                            verts.push([pos, tex, norm].concat());
                            cunt += 1;
                            if i == 0 {
                                v0 = verts.len() - 1;
                            }
                            if i >= 2 {
                                indices.extend_from_slice(&[
                                    v0 as u16, 
                                    (verts.len() - 2) as u16, 
                                    (verts.len() - 1) as u16
                                ]);
                            }

                            i += 1;
                        }
                    } else {
                        panic!("Invalid face, face contains less than 3 specs");
                    }
                },
                _ => continue,
            }
        }

        println!("{}, {}", cunt, verts.len());
        let res = verts.into_iter().flatten().collect();
        return (res, indices);
    }
}
