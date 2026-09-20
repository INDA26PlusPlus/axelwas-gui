use nannou::prelude::*;

struct Textures {
    queen_w: Handle<Image>
}

impl Textures {
    fn load(app: &App) -> Self {
        Self {
            queen_w: app.asset_server().load("QueenW.png")
        }
    }
}

struct Model {
    textures: Textures
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).view(view).build();

    Model { textures: Textures::load(app) }
}

fn draw_background(app: &App) {
    let draw = app.draw();

    let win = app.window_rect();
    let width = win.right() - win.left();
    let height = win.top() - win.bottom();

    for c in 0..8 {
        for r in 0..8 {
            let light = Color::srgb(240.0 / 256.0, 217.0 / 256.0, 181.0 / 256.0);
            let dark = Color::srgb(181.0 / 256.0, 136.0 / 256.0, 99.0 / 256.0);
            let to_use = if (c + r) % 2 == 1 {
                light
            } else {
                dark
            };

            draw.rect().color(to_use)
                .x(win.left() + c as f32 * (width / 8.0) + width / 16.0)
                .y(win.bottom() + r as f32 * (height / 8.0) + height / 16.0)
                .w(width / 8.0)
                .h(height / 8.0);
        }
    }
}

fn draw_pieces(app: &App, model: &Model) {
    let draw = app.draw();

    let win = app.window_rect();
    let width = win.right() - win.left();
    let height = win.top() - win.bottom();


    for c in 0..8 {
        for r in 0..8 {
            draw.rect().texture(&model.textures.queen_w)
                .x(win.left() + c as f32 * (width / 8.0) + width / 16.0)
                .y(win.bottom() + r as f32 * (height / 8.0) + height / 16.0)
                .w(width / 8.0)
                .h(height / 8.0);
        }
    }
}

fn view(app: &App, model: &Model) {
    let draw = app.draw();

    draw_background(app);

    draw_pieces(app, model);

    

}