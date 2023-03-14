#![allow(non_snake_case)]
use nannou::{prelude::*, image::{ImageBuffer, DynamicImage}};
use nannou_egui::{self, egui::{self}, Egui};
use nannou::image::Rgb;
#[derive(Debug)]
struct Drummer{
    freq: f32,
    phase: f32,
    name: String,
}
struct Settings{
    dt: f32,
    koeff: f32,
    target_freq: f32,
    nijika_freq: f32,
    andrew_freq: f32,
    nijika_focus: bool,
    disable_cool_animations: bool
}
struct UhTextures{
    fletcher: [wgpu::Texture; 2],
    andrew: [wgpu::Texture; 2],
    nijika: [wgpu::Texture; 2],
}
#[derive(Clone)]
enum RushingOrDragging{
    RUSHING,
    DRAGGING,
    MYTEMPO
}
struct Model{
    fletcher: Drummer,
    andrew: Drummer,
    nijika: Drummer,
    settings: Settings,
    egui: Egui,
    textures: UhTextures,
    rushing_or_dragging: [RushingOrDragging; 2]
}
impl Model {
    fn update(&mut self, dt: f32) {
        let koeff = self.settings.koeff;
        // dPhase of Nijika is half the influence of Andrew and one of Fletcher's  
        // dPhase of Andrew is the influence of Fletcher  
        let dPhase_Andrew = 
            self.andrew.freq + koeff*(self.fletcher.phase - self.andrew.phase).sin()/3.;
        
        let dPhase_Nijika = 
            if self.settings.nijika_focus{
                    self.nijika.freq + koeff*((self.fletcher.phase - self.nijika.phase).sin())/3.
            }
            else{
                self.nijika.freq + koeff*(0.5*(self.andrew.phase - self.nijika.phase).sin()
                    + (self.fletcher.phase - self.nijika.phase).sin())/3.
            };
        let rushing_or_dragging: [RushingOrDragging; 2] = [self.nijika.freq, self.andrew.freq].map(|x|
            {
            // println!("{x} \\ {}", (self.fletcher.freq - x).signum() as i8);
            match (x - self.fletcher.freq).signum() as i8 {
                1 => {RushingOrDragging::RUSHING},
                -1 => {RushingOrDragging::DRAGGING},
                _ => {RushingOrDragging::MYTEMPO}
            }
            }
        );
        self.nijika.phase = (self.nijika.phase + dPhase_Nijika*dt)% (2.*PI); 
        self.andrew.phase = (self.andrew.phase + dPhase_Andrew*dt) % (2.*PI);
        self.fletcher.phase = (self.fletcher.phase + self.fletcher.freq*dt) % (2.*PI);
        self.rushing_or_dragging = rushing_or_dragging;
    }
}

fn egui_majjikks(model: &mut Model, update: Update){
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("global").anchor(egui::Align2::CENTER_TOP, [0., 20.]).show(&ctx, |ui| {
        ui.label("Δt:");
        ui.add(egui::Slider::new(&mut settings.dt, 0.0..=4.));
        ui.label("coupling coefficient:");
        ui.add(egui::Slider::new(&mut settings.koeff, 0.0..=0.5));
        ui.label("Fletcher's frequency (⌚):");
        ui.add(egui::Slider::new(&mut settings.target_freq, 0.05..=0.2));
        let clicked = ui.button("give em hell").clicked();
        if clicked {
            model.nijika.phase = random();
            model.andrew.phase = random();
        }
        ui.checkbox(&mut settings.disable_cool_animations, "disable animations (");
    });
    egui::Window::new(model.nijika.name.as_str()).anchor(egui::Align2::LEFT_TOP, [40., 20.]).show(&ctx, |ui| {
        ui.label("Nijika's frequency:");
        ui.add(egui::Slider::new(&mut settings.nijika_freq, 0.05..=0.2));
        let clicked = ui.button("throw a cymbal at her").clicked();
        if clicked {
            model.nijika.phase = random();
        }
        ui.checkbox(&mut settings.nijika_focus, "focus");
    });
    egui::Window::new(model.andrew.name.as_str()).anchor(egui::Align2::RIGHT_TOP, [-40., 20.]).show(&ctx, |ui| {
        ui.label("Andrew's frequency:");
        ui.add(egui::Slider::new(&mut settings.andrew_freq, 0.05..=0.2));
        let clicked = ui.button("throw a cymbal at him").clicked();
        if clicked {
            model.andrew.phase = random();
        }
    });
}

fn update(app: &App, model: &mut Model, update: Update) {
    egui_majjikks(model, update);
    model.fletcher.freq = model.settings.target_freq;
    model.nijika.freq = model.settings.nijika_freq;
    model.andrew.freq = model.settings.andrew_freq;
    if (app.elapsed_frames() % 2) == 0 {
        model.update(model.settings.dt);
    }
}

fn the_code_is_too_messy_i_need_another_function_for_drawing_sprites(model: &Model, draw: &Draw, frq: f32){
    let NIJIKA = rgb(1., 0.9, 0.07);
    let ANDREW = rgb(0.9, 0.17, 0.8);
    let INABAKUMORI = rgb(0.7, 0.7, 0.7);
    let mut nijika_vec: Vec2 = Vec2::new(-312., 0.);
    let mut andrew_vec: Vec2 = Vec2::new(312., 0.);
    let fletcher_vec: Vec2 = Vec2::new(0., 256.);
    let threshold: f32 = 0.25;
    let mut draw_index_nijika: usize = 0;
    let mut draw_index_andrew: usize = 0;
    let mut draw_index_fletcher: usize = 0;
    let prepared_rushing_or_dragging = model.rushing_or_dragging.clone().map(|x| 
        match x {
            RushingOrDragging::RUSHING => {"RUSHING!"},
            RushingOrDragging::DRAGGING => {"DRAGGING!"},
            RushingOrDragging::MYTEMPO => {"not good enough"},
        }
    );
    // println!("[{}] \\ [{}]", (PI/frq)-threshold*2., model.nijika.phase % frq); // DEBUG!
    if (model.nijika.phase % frq) < threshold || (model.nijika.phase % frq) > (PI/frq)-threshold*2.{
        draw_index_nijika = 1;
        let random_vec: Vec2 = Vec2::new(random_range(-20., 20.), random_range(-20., 20.));
        nijika_vec += random_vec;
    }
    draw.texture(&model.textures.nijika[draw_index_nijika]).xy(nijika_vec);

    if (model.andrew.phase % frq) < threshold {
        draw_index_andrew = 1;
        let random_vec: Vec2 = Vec2::new(random_range(-20., 20.), random_range(-20., 20.));
        andrew_vec += random_vec;
    }
    draw.texture(&model.textures.andrew[draw_index_andrew]).xy(andrew_vec);

    if (model.fletcher.phase % frq) < threshold {
        draw_index_fletcher = 1;
    }
    if (model.fletcher.phase % frq) < threshold*2. {
        draw.text(prepared_rushing_or_dragging[0]).color(INABAKUMORI).x_y(-380., 284.).font_size(42).w_h(42.*10., 42.);
        draw.text(prepared_rushing_or_dragging[0]).color(NIJIKA).x_y(-382., 286.).font_size(42).w_h(42.*10., 42.);
        draw.text(prepared_rushing_or_dragging[1]).color(INABAKUMORI).x_y(384., 284.).font_size(42).w_h(42.*10., 42.);
        draw.text(prepared_rushing_or_dragging[1]).color(ANDREW).x_y(382., 286.).font_size(42).w_h(42.*10., 42.);
        
    }
    draw.texture(&model.textures.fletcher[draw_index_fletcher]).xy(fletcher_vec).w_h(216., 216.);
}

fn draw_beats(model: &Model, draw: &Draw, frq: f32, damping: f32){
    {
        let decayed_col = rgba(1., 0.9, 0.07, 0.8 - (model.nijika.phase % frq).abs() * damping);
        draw.ellipse().x_y(-312., 0.).color(decayed_col).radius(256.);
    }
    {
        let decayed_col = rgba(0.9, 0.17, 0.8, 0.8 - (model.andrew.phase % frq).abs() * damping);
        draw.ellipse().x_y(312., 0.).color(decayed_col).radius(256.);
    }
    {
        let decayed_col = rgba(0.8627, 0.0784, 0.2353, 0.8 - (model.fletcher.phase % frq).abs() * damping);
        draw.ellipse().x_y(0., 256.).color(decayed_col).radius(256.);
    }
}

fn draw_runny_circles(model: &Model, draw: &Draw){
    let NIJIKA = rgb(1., 0.9, 0.07);
    let ANDREW = rgb(0.9, 0.17, 0.8);
    let INABAKUMORI = rgb(0.7, 0.7, 0.7);

    draw.ellipse()
        .x_y(0., 0.)
        .color(INABAKUMORI) 
        .radius(100.);
    draw.ellipse()
        .x_y(-80. * (model.nijika.phase).cos(), 80.*(model.nijika.phase).sin())
        .color(NIJIKA)
        .radius(10.);
    draw.ellipse()
        .x_y(-80. * (model.andrew.phase).cos(), 80.*(model.andrew.phase).sin()) 
        .color(ANDREW)
        .radius(10.);
    draw.ellipse()
        .x_y(-80. * (model.fletcher.phase).cos(), 80.*(model.fletcher.phase).sin())
        .color(CRIMSON)
        .radius(10.);
}
fn view(app: &App, model: &Model, frame: Frame) {
    
    let draw = app.draw();
    
    draw.background()
        .color(WHITE);
 
    let frq = 2.*PI/4.;
    let damping = 0.3;

    draw_beats(model, &draw, frq, damping);

    if !model.settings.disable_cool_animations{
        the_code_is_too_messy_i_need_another_function_for_drawing_sprites(model, &draw, frq);
    }

    draw_runny_circles(model, &draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let target_freq: f32 = 1./(2.*PI*2.);
    let Fletcher = Drummer{
        freq: target_freq,
        phase: random_f32() * 2.*PI,
        name: "Fletcher".to_string()
    };                                          // conducting
    let Andrew = Drummer{
        freq: target_freq*(1.-0.08), 
        phase: random_f32() * 2.*PI,
        name: "Andrew".to_string()
    };                                          // dragging
    let Nijika = Drummer{
        freq: target_freq*(1.+0.08),
        phase: random_f32() * 2.*PI,
        name: "Nijika".to_string()
    };                                          // rushing
    let window_id = app
        .new_window()
        .size_pixels(1600, 950)
        .title("uniteSynchronisation")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    // and here we do some asset loading
    let assets = app.assets_path().unwrap_or(std::path::PathBuf::from(""));
    let mut fallback_image: DynamicImage = DynamicImage::ImageRgb8(ImageBuffer::new(512, 512));
    for (x, y, pixel) in fallback_image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let color = if (x / 32 + y / 32) % 2 == 0 {
            Rgb([255, 0, 255])
        } else {
            Rgb([0, 0, 0])
        };
        *pixel = color;
    }
    
    let fletcher_textures = [
        wgpu::Texture::from_path(app, assets.join("fletcher_ready.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image)),
        wgpu::Texture::from_path(app, assets.join("fletcher_beat.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image))
    ];
    let andrew_textures = [
        wgpu::Texture::from_path(app, assets.join("andrew_ready.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image)),
        wgpu::Texture::from_path(app, assets.join("andrew_beat.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image))
    ];
    let nijika_textures = [
        wgpu::Texture::from_path(app, assets.join("nijika_ready.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image)),
        wgpu::Texture::from_path(app, assets.join("nijika_beat.png")).unwrap_or_else(|_| wgpu::Texture::from_image(app, &fallback_image))
    ];        
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model{
        settings: Settings {
            dt: 1.,
            koeff: 0.25,
            target_freq,
            andrew_freq: Andrew.freq,
            nijika_freq: Nijika.freq,
            nijika_focus: false,
            disable_cool_animations: false
        }, 
        egui,
        fletcher: Fletcher,
        andrew: Andrew,
        nijika: Nijika,
        textures: UhTextures { fletcher: fletcher_textures, andrew: andrew_textures, nijika: nijika_textures },
        rushing_or_dragging: [RushingOrDragging::RUSHING, RushingOrDragging::DRAGGING]
    }
}
fn main() {
    println!("sanity: checked!");
    nannou::app(model)
        .view(view)
        .update(update)
        .run();    
}

