use fem_comp::vals::Obj;
extern crate nfd;
extern crate raylib;
use nfd::Response;
use raylib::{consts::KeyboardKey::*, prelude::*};
use std::ffi::CString;

fn static_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

#[derive(PartialEq)]
enum ConstrType {
    FixedC,
    FixedXaPoYaP,
    NonFixed,
}

// TODO
// 1. Zoom view  [X]
// 2. Rich text output []
// 3. Sliders []
// 4. Size coefficent []

impl ConstrType {
    fn determine(fl: [f32; 3]) -> Self {
        match fl {
            [x, y, p] if !(x == 0.0 && y == 0.0 && p == 0.0) => ConstrType::FixedC,
            [x, y, p] if !(x == 0.0  || y == 0.0 ) && p == 0.0 => {
                ConstrType::FixedXaPoYaP
            }
            _ => ConstrType::NonFixed,
        }
    }
    pub fn draw_determined(fl: &[f32; 3], x: f32, y: f32, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        match ConstrType::determine(*fl) {
            // v1
            // v2 v3
            ConstrType::FixedC => d.draw_triangle(
                Vector2 { x, y: y - 8.0 },
                Vector2 {
                    x: x - 8.0,
                    y: 8.0 + y,
                },
                Vector2 {
                    x: 8.0 + x,
                    y: 8.0 + y,
                },
                Color::BLACK,
            ),
            ConstrType::FixedXaPoYaP => d.draw_circle(x as i32, y as i32, 10.0, Color::BLACK),
            ConstrType::NonFixed => {
                d.draw_rectangle(x as i32, (y as i32) - 5, 10, 10, Color::BLACK)
            }
        }
    }
}

fn not_fun(d: &mut RaylibDrawHandle, layout_recs: [Rectangle; 8], test: &Obj, test_info: &str,x : i32) {
    if d.gui_button(
        Rectangle {
            x: layout_recs[0].x + 15.0,
            y: layout_recs[0].y + x as f32,
            width: 120.0,
            height: 24.0,
        },
        Some(static_cstr(test_info).as_c_str()),
    ) {
        if d.gui_window_box(layout_recs[1], Some(static_cstr(test_info).as_c_str())) {
            match test_info{
                "Nodes" => d.draw_text(
                    &format!("{:?}", test.nodes),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
                "Loads" => d.draw_text(
                    &format!("{:?}", test.loads),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
                "PhysGeos" => d.draw_text(
                    &format!("{:?}", test.physgeos),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
                "Constraints" => d.draw_text(
                    &format!("{:?}", test.constraints),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
                "S vec" => d.draw_text(
                    &format!("{:?}", test.s),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
                _ => d.draw_text(
                    &format!("Something went wrong",),
                    (layout_recs[1].x as i32) + 10,
                    (layout_recs[1].y as i32) + 15,
                    10,
                    Color::BLACK,
                ),
            }
        }
    }
}

fn main() {
    let mut output: String = "".to_string();
    let (mut sc_h, mut sc_w) = (1000, 1200);
    let mut layout_recs = [
        Rectangle {
            // Object info box
            x: 0.0,
            y: 100.0,
            width: 300.0,
            height: (sc_h - 105) as f32,
        },
        Rectangle {
            // Visual output
            x: 320.0,
            y: 100.0,
            width: (sc_w - 325) as f32,
            height: (sc_h - 105) as f32,
        },
        Rectangle {
            // Status bar
            x: 0.0,
            y: 0.0,
            width: sc_w as f32,
            height: 65.0,
        },
        Rectangle {
            // Input File button
            x: 20.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Quit button
            x: 160.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Clear button
            x: 300.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Save button
            x: 440.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // View additional info
            x: 300.0,
            y: 100.0,
            width: 300.0,
            height: 200.0,
        },
    ];
    let mut test: Obj = Obj::create_empty_obj();

    let (mut rl, thread) = raylib::init()
        .size(sc_w, sc_h)
        .title("FEA solver, batteries lost in transit")
        .resizable()
        .build();

    rl.set_window_min_size(800, 1000);
    rl.set_target_fps(120);
    let mut obj_open = false;

    let mut camra = Camera2D {
        offset: Vector2::new(320.0, 100.0).into(),
        target: Vector2::new(320.0, 100.0).into(),
        rotation: 0.0,
        zoom: 1.0,
    };
    while !rl.window_should_close() {
        camra.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;
        camra.zoom = camra.zoom.max(0.1).min(2.0);

        if rl.is_key_pressed(KEY_R) {
            camra.zoom = 1.0;
        }

        layout_recs[0].height = (sc_h - 105) as f32;
        layout_recs[1].width = (sc_w - 325) as f32;
        layout_recs[1].height = (sc_h - 105) as f32;
        layout_recs[2].width = sc_w as f32;

        let mut d = rl.begin_drawing(&thread);
        (sc_h, sc_w) = (d.get_screen_height(), d.get_screen_width());
        d.clear_background(Color::WHITE);
        d.draw_rectangle_rec(layout_recs[1], Color::LIGHTGRAY);
        // d.gui_grid(layout_recs[1], 1.0, 1);
        if !test.is_empty() {
           
            {
                let mut d = d.begin_mode2D(camra);

                for element in test.elements.iter() {
                    let (start, end) = (element.b_id, element.e_id);
                    let (start_x, start_y, end_x, end_y) = (
                        test.nodes[start].0 + layout_recs[1].x + 10.0,
                        test.nodes[start].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y
                            - 10.0,
                        test.nodes[end].0 + layout_recs[1].x + 10.0,
                        test.nodes[end].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y - 10.0,
                    );
                    
                    
                    d.draw_line(
                        (start_x) as i32,
                        (start_y) as i32,
                        (end_x) as i32,
                        (end_y) as i32,
                        Color::BLACK,
                    );

                    for ele in test.constraints.iter() {
                        // Make this a pattern
                        if ele.node_id == start {
                            ConstrType::draw_determined(
                                &ele.stiffness,
                                start_x,
                                start_y,
                                &mut d,
                            );
                            continue;
                        } else if ele.node_id == end {
                            ConstrType::draw_determined(
                                &ele.stiffness,
                                end_x,
                                end_y,
                                &mut d,
                            );
                            continue;
                        } else if test.constraints.iter().any(|f| f.node_id == start ) {
                            // ??
                            ConstrType::draw_determined(
                                &[0.0, 0.0, 0.0],
                                end_x,
                                end_y,
                                &mut d,
                            );
                        }
                    }
                  
                }
            }
            [("Nodes",15),("Loads",45),("PhysGeos",75),("Constraints",105),("S vec",135)].into_iter().for_each(|x| {
                not_fun(&mut d, layout_recs, &test, x.0,x.1);
            }); 
        } else {
            d.draw_text(
                &format!("No object"),
                (layout_recs[0].x as i32) + 10,
                (layout_recs[0].y as i32) + 15,
                11,
                Color::BLACK,
            );
        }

        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));

        // Bunch of buttons
        if d.gui_button(layout_recs[3], Some(static_cstr("Input File").as_c_str())) {
            output = match nfd::open_file_dialog(Some("xlsx"), Some(".")).expect("oh no") {
                Response::Okay(file_path) => file_path,
                _ => continue,
            };
            test = fem_comp::create_obj_from_xlsx(output.as_str()).expect("Create object error");
            test.c_s();
            obj_open = true;
        }

        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }

        if obj_open {
            if d.gui_button(layout_recs[5], Some(static_cstr("Clear Input").as_c_str())) {
                test = Obj::create_empty_obj();
                obj_open = false;
            }
            if d.gui_button(layout_recs[6], Some(static_cstr("Save object").as_c_str())) {
                test.write_data(output.as_str()).unwrap();
            }
        }

        d.gui_group_box(
            layout_recs[0],
            Some(static_cstr(&format!("Object : {}", output)).as_c_str()),
        );
    }
}