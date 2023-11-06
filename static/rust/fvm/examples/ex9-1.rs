use jm_math::linear_algebra::Vector3D;

#[allow(non_snake_case)]
fn main() {
    let cC = Vector3D::from(13.0, 11.0, 0.0);
    let phiC = 167.0;
    let volC = 76.0;

    let cF = [
        Vector3D::from( 4.5,  9.5, 0.0),
        Vector3D::from( 8.0,  3.0, 0.0),
        Vector3D::from(17.0,  3.5, 0.0),
        Vector3D::from(22.0, 10.0, 0.0),
        Vector3D::from(16.0, 20.0, 0.0),
        Vector3D::from( 7.0, 18.0, 0.0),
    ];
    let phiF = [56.75, 35.0, 80.0, 252.0, 356.0, 151.0];
    let nf = [
        Vector3D::from( 9.0, 14.0, 0.0),
        Vector3D::from( 8.0,  8.0, 0.0),
        Vector3D::from(12.0,  5.0, 0.0),
        Vector3D::from(17.0,  9.0, 0.0),
        Vector3D::from(17.5, 14.0, 0.0),
        Vector3D::from(12.0, 17.0, 0.0),
    ];
    let gradF = [
        Vector3D::from(10.5,  5.5, 0.0),
        Vector3D::from( 4.0,  9.0, 0.0),
        Vector3D::from( 4.5, 18.0, 0.0),
        Vector3D::from(11.0, 23.0, 0.0),
        Vector3D::from(21.0, 17.0, 0.0),
        Vector3D::from(19.0,  8.0, 0.0),
    ];
    
    // next node of a n[i]
    let mut nn = nf.iter().cycle(); nn.next();
    let sf = nf.map(|n|{
        let nn = nn.next().unwrap();
        let dx = nn.get_x() - n.get_x();
        let dy = nn.get_y() - n.get_y();

        // surface vector
        Vector3D::from(dy, -dx, 0.0)
    });

    // integration point of each face
    nn = nf.iter().cycle(); nn.next();
    let ip = nf.map(|n| {
        let nn = nn.next().unwrap();
        Vector3D::from(
            0.5 * (nn.get_x() + n.get_x()),
            0.5 * (nn.get_y() + n.get_y()),
            0.0
        )
    });

    // weighting factor
    let gc = cF.iter().zip(ip.iter()).map(|(cf, ip)| {
        let dCf = (&cC - ip).length();
        let dFf = (cf - ip).length();
        
        dFf / (dCf + dFf)
    }).collect::<Vec<f64>>();

    let mut grad = Vector3D::new();
    for (i, sf) in sf.iter().enumerate() {
        let phif = gc[i] * phiC + (1.0 - gc[i]) * phiF[i];
        
        grad += &(phif * sf);
        // println!("phif: {phif}");
    }
    grad /= volC;

    println!("{:.3}", grad);
}