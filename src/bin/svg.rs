fn main() {
    let mut lz = lorenz::Lorenz {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        .. Default::default()
    };

    println!("<svg viewBox=\"-20 -20 40 40\" xmlns=\"http://www.w3.org/2000/svg\">");
    print!("  <polyline stroke=\"black\" fill=\"none\" stroke-width=\"0.1\" points=\"");
    for i in 0..4000 {
        lz.update(0.01);
        if i > 0 {
            print!(" ");
        }
        print!("{:.5},{:.5}", lz.x, lz.y);
    }
    println!("\"/>");
    println!("</svg>");
}
