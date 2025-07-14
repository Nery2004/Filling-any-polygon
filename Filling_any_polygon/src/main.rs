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