#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

use glium::Surface;
extern crate rust_lifegame;

use rust_lifegame::support;
use rust_lifegame::game;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;


widget_ids!(
struct Ids { 
    canvas, grid
});


struct PlayerState{
    camera_pos: (f64,f64),
    zoom: f64,
    is_move_btn_pressed: (bool,bool,bool,bool),
    current_mouse_pos: (f64,f64)
}
impl PlayerState{
    fn new()->Self{
        Self {
            camera_pos: (0.,0.),
            zoom: 1.,
            is_move_btn_pressed: (false,false,false,false),
            current_mouse_pos: (0.,0.)
        }
    }
}

fn main() {

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("はろーわーるど")
        .with_dimensions((WIDTH,HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    let mut ui = conrod_core::UiBuilder::new([HEIGHT as f64, WIDTH as f64]).build();
    
    
    // let ids = Ids::new(ui.widget_id_generator());
    // let assets = find_folder::Search::KidsThenParents(3,5).for_folder("assets").unwrap();
    // let font_path = assets.join("fonts/azuki/azuki.ttf");
    // ui.fonts.insert_from_file(font_path).unwrap();

    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    
    // let images = vec!["akane_normal_0.png", "akane_normal_1.png", "akane_normal_2.png", "akane_damage_0.png", "akane_damage_1.png", "akane_damage_2.png"];
    // let images = images.iter().map(|a|load_image(&display.0, a))
    //                             .map(|a|image_map.insert(a)).collect();
    println!("aaa");
    let mut events = Vec::new();

    let mut game = game::Game::new(100, ui.widget_id_generator());
    let mut player = PlayerState::new();

    let mut last_update = std::time::Instant::now();
    let frame_interval_ms = std::time::Duration::from_millis(1000/60 as u64);
    'render: loop{
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < frame_interval_ms{
            std::thread::sleep(frame_interval_ms - duration_since_last_update);
        }
        last_update = std::time::Instant::now();


        if game.is_time_pass{
            game.update();
        }
        {
            let  widgets = &mut ui.set_widgets();
            game.draw(widgets, player.camera_pos, player.zoom);
        }
        let movement = 10.0 / player.zoom;
        if player.is_move_btn_pressed.0 {
            player.camera_pos.1 -= movement;
        }
        if player.is_move_btn_pressed.1 {
            player.camera_pos.0 -= movement;
        }
        if player.is_move_btn_pressed.2 {
            player.camera_pos.1 += movement;
        }
        if player.is_move_btn_pressed.3 {
            player.camera_pos.0 += movement;
        }
        player.camera_pos.0 = player.camera_pos.0.max(0.);
        player.camera_pos.1 = player.camera_pos.1.max(0.);

        
        events.clear();
        events_loop.poll_events(|event| {events.push(event);});


        for event in events.drain(..){
            match event.clone(){
                glium::glutin::Event::WindowEvent {event,..} =>{
                    match event{
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        }=>break 'render,
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                            ..
                        }=>{
                            handle_keyboard(keycode, state, &mut game, &mut player);
                        },
                        glium::glutin::WindowEvent::MouseInput{
                            state: glium::glutin::ElementState::Released, 
                            button: glium::glutin::MouseButton::Left, ..}=>{
                            cell_clicked(&mut game, &mut player);
                        },
                        glium::glutin::WindowEvent::MouseWheel{
                            delta,
                            ..
                        }=>{
                            zoom_changed(delta, &mut player, (ui.win_w, ui.win_h));
                        },
                        glium::glutin::WindowEvent::CursorMoved{position: pos, ..}=>{
                            player.current_mouse_pos = pos.into();
                        },
                        _ => (),
                    }
                }
                _=>(),
            };
            let input = match support::convert_event(event, &display) {
                None => continue,
                Some(input)=> input,
            };
            ui.handle_event(input);
            
            
            
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0,0.0,0.0,1.0);
            renderer.draw(&display.0, &mut target, &image_map);
            target.finish().unwrap();
        }
    }


}

fn load_image(display: &glium::Display, path: &'static str) -> glium::texture::Texture2d {
    let assets = find_folder::Search::ParentsThenKids(1, 1).for_folder("assets").unwrap();
    let path = assets.join(format!("{}{}", "images/", path));
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    println!("{}",path.to_str().unwrap());
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    println!("ss");
    texture
}

fn handle_keyboard(keycode: glium::glutin::VirtualKeyCode, press: glium::glutin::ElementState, game: &mut game::Game, player: &mut PlayerState){
    let press = if press == glium::glutin::ElementState::Pressed {true} else {false};

    match keycode{
        glium::glutin::VirtualKeyCode::W=>{
            player.is_move_btn_pressed.0 = press;
        },
        glium::glutin::VirtualKeyCode::A=>{
            player.is_move_btn_pressed.1 = press;
        },
        glium::glutin::VirtualKeyCode::S=>{
            player.is_move_btn_pressed.2 = press;
        },
        glium::glutin::VirtualKeyCode::D=>{
            player.is_move_btn_pressed.3 = press;
        },
        glium::glutin::VirtualKeyCode::Space=>{
            if !press{
                game.is_time_pass = !game.is_time_pass;
            }
        },
        _=>()
    }
}
fn cell_clicked(game: &mut game::Game, player: &mut PlayerState){
    let (x,y) = player.current_mouse_pos;
    let (x,y) = (player.camera_pos.0 + x / player.zoom, player.camera_pos.1 + y / player.zoom);
    let (tilex, tiley) = (f64::floor( x / game::CELL_LENGTH) as usize, f64::floor(y/game::CELL_LENGTH) as usize);

    if  tilex < game.map_length as usize && tiley < game.map_length as usize{
        match game.cells[tilex][tiley].state{
            game::State::ALIVE=>{
                game.cells[tilex][tiley].state = game::State::DEAD(0);
            },
            game::State::DEAD(_)=>{
                game.cells[tilex][tiley].state = game::State::ALIVE;
                game.cells[tilex][tiley].image_type %= 3;
            }
        }
    }
}

fn zoom_changed(delta: glium::glutin::MouseScrollDelta, player: &mut PlayerState, dimension: (f64,f64)){
    let scrolly;
    match delta{
        glium::glutin::MouseScrollDelta::LineDelta(_,y)=>{
            scrolly = y as f64;
        },
        glium::glutin::MouseScrollDelta::PixelDelta(l)=>{
            scrolly = l.y;
        }
    }
    let original_zoom = player.zoom;
    if scrolly > 0.{
        player.zoom += 0.1;
    }else if scrolly < 0.{
        player.zoom -= 0.1;
    }
    if player.zoom <= 0.{
        player.zoom = 0.1;
    }
    let k = original_zoom / player.zoom;
    player.camera_pos.0 += dimension.0 / original_zoom * (1. - k) / 2.;
    player.camera_pos.1 += dimension.1 / original_zoom * (1. - k) / 2.;
}