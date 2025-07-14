use raylib::prelude::*;

fn main() {
    let (image_width, image_height) = (500, 500);
    let (mut rl, thread) = raylib::init()
        .size(image_width, image_height)
        .title("Polígonos con raylib")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        poligono1(&mut d);
        poligono2(&mut d);
        poligono3(&mut d);
        poligono4(&mut d);
        poligono5(&mut d);
    }
}
fn poligono1(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];
    
    d.draw_triangle_fan(&polygon, Color::YELLOW);
    
    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}
fn poligono2(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];
    
    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}
fn poligono3(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];
    
    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}
fn poligono4(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];

    // Relleno (triangulación simple tipo abanico)
    for i in 1..(polygon.len() - 1) {
        d.draw_triangle(polygon[0], polygon[i], polygon[i + 1], Color::GREEN);
    }

    // Borde
    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}
fn poligono5(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];
    
    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}