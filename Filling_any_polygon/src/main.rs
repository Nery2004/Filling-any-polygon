use raylib::prelude::*;
struct Edge {
    y_min: f32,
    y_max: f32,
    x: f32,
    slope: f32,
}

fn scanline_fill(d: &mut RaylibDrawHandle, polygon: &[Vector2], color: Color) {
    if polygon.len() < 3 { return; }

    let y_min = polygon.iter().map(|p| p.y).reduce(f32::min).unwrap().floor() as i32;
    let y_max = polygon.iter().map(|p| p.y).reduce(f32::max).unwrap().ceil() as i32;

    let mut edges = Vec::new();
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        if p1.y != p2.y { 
            let (y_min, y_max, x, slope) = if p1.y < p2.y {
                (p1.y, p2.y, p1.x, (p2.x - p1.x) / (p2.y - p1.y))
            } else {
                (p2.y, p1.y, p2.x, (p1.x - p2.x) / (p1.y - p2.y))
            };

            edges.push(Edge { y_min, y_max, x, slope });
        }
    }

    edges.sort_by(|a, b| a.y_min.partial_cmp(&b.y_min).unwrap());

    let mut active_edges = Vec::new();
    let mut y = y_min;

    while y <= y_max {
        while let Some(edge) = edges.first() {
            if edge.y_min as i32 == y {
                active_edges.push(edges.remove(0));
            } else {
                break;
            }
        }


        active_edges.retain(|e| (e.y_max as i32) > y);

        active_edges.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        for i in (0..active_edges.len()).step_by(2) {
            if i + 1 >= active_edges.len() { break; }

            let x_start = active_edges[i].x.floor() as i32;
            let x_end = active_edges[i + 1].x.ceil() as i32;

            if x_start < x_end {
                d.draw_line(x_start, y, x_end, y, color);
            }
        }

        for edge in active_edges.iter_mut() {
            edge.x += edge.slope;
        }

        y += 1;
    }
}

fn main() {
    let mut saved = false;
    let image_width = 800;
    let image_height = 800;

    let (mut rl, thread) = raylib::init()
        .size(image_width, image_height)
        .title("Polígonos")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            poligono1(&mut d);
            poligono2(&mut d);
            poligono3(&mut d);
            poligono4(&mut d);
        } 

        if !saved {
            rl.take_screenshot(&thread, "out.png");
            saved = true;
        }
    }
}

fn poligono1(d: &mut RaylibDrawHandle) {
    let p = vec![
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


    scanline_fill(d, &p, Color::YELLOW);


    for i in 0..p.len() {
        let next = (i + 1) % p.len();
        d.draw_line_ex(p[i], p[next], 2.0, Color::WHITE);
    }
}

fn poligono2(d: &mut RaylibDrawHandle) {
    let polygon = vec![
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];
    
    scanline_fill(d, &polygon, Color::BLUE);
    
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
    
    scanline_fill(d, &polygon, Color::RED);

    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();
        d.draw_line_ex(polygon[i], polygon[next], 2.0, Color::WHITE);
    }
}

fn poligono4(d: &mut RaylibDrawHandle) {
    let exterior = vec![
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

    let agujero = vec![
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];
    
    scanline_fill(d, &exterior, Color::GREEN);
    
    scanline_fill(d, &agujero, Color::BLACK);
    
    // Contornos
    for i in 0..exterior.len() {
        let next = (i + 1) % exterior.len();
        d.draw_line_ex(exterior[i], exterior[next], 2.0, Color::WHITE);
    }

    for i in 0..agujero.len() {
        let next = (i + 1) % agujero.len();
        d.draw_line_ex(agujero[i], agujero[next], 2.0, Color::WHITE);
    }
}