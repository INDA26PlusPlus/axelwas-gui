use chess_box::pieces::ChessPiece;
use nannou::prelude::{*};

struct Textures {
    queen_w: Handle<Image>,
    queen_b: Handle<Image>,
    rook_w: Handle<Image>,
    rook_b: Handle<Image>,
    pawn_w: Handle<Image>,
    pawn_b: Handle<Image>,
    knight_w: Handle<Image>,
    knight_b: Handle<Image>,
    king_w: Handle<Image>,
    king_b: Handle<Image>,
    bishop_w: Handle<Image>,
    bishop_b: Handle<Image>,
}

impl Textures {
    fn load(app: &App) -> Self {
        Self {
            queen_w: app.asset_server().load("Queen_W.png"),
            queen_b: app.asset_server().load("Queen_B.png"),
            rook_b: app.asset_server().load("Rook_B.png"),
            rook_w: app.asset_server().load("Rook_W.png"),
            pawn_w: app.asset_server().load("Pawn_W.png"),
            pawn_b: app.asset_server().load("Pawn_B.png"),
            knight_w: app.asset_server().load("Knight_W.png"),
            knight_b: app.asset_server().load("Knight_B.png"),
            king_w: app.asset_server().load("King_W.png"),
            king_b: app.asset_server().load("King_B.png"),
            bishop_w: app.asset_server().load("Bishop_W.png"),
            bishop_b: app.asset_server().load("Bishop_B.png"),
        }
    }
    fn image_from_type<'a>(&'a self, t: &chess_box::pieces::PieceType, is_white: bool) ->  &'a Handle<Image> {
        let p = match (*t, is_white) {
            (chess_box::pieces::PieceType::Pawn, true) => &self.pawn_w,
            (chess_box::pieces::PieceType::Pawn, false) => &self.pawn_b,
            (chess_box::pieces::PieceType::Knight, true) => &self.knight_w,
            (chess_box::pieces::PieceType::Knight, false) => &self.knight_b,
            (chess_box::pieces::PieceType::Bishop, true) => &self.bishop_w,
            (chess_box::pieces::PieceType::Bishop, false) => &self.bishop_b,
            (chess_box::pieces::PieceType::Rook, true) => &self.rook_w,
            (chess_box::pieces::PieceType::Rook, false) => &self.rook_b,
            (chess_box::pieces::PieceType::Queen, true) => &self.queen_w,
            (chess_box::pieces::PieceType::Queen, false) => &self.queen_b,
            (chess_box::pieces::PieceType::King, true) => &self.king_w,
            (chess_box::pieces::PieceType::King, false) => &self.king_b,
        };
        return p;
    }
}

struct Model {
    textures: Textures,
    game: chess_box::game::ChessGame
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).view(view).build();

    Model { textures: Textures::load(app), game: chess_box::game::ChessGame::new_standard_game() }
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
            let piece = model.game.board().get_piece_file_rank(c, r);
            let piece: ChessPiece = match piece {
                None => continue,
                Some(p) => p
            };

            draw.rect().texture(model.textures.image_from_type(&piece.piece_type(), piece.is_white()))
                .x(win.left() + c as f32 * (width / 8.0) + width / 16.0)
                .y(win.bottom() + r as f32 * (height / 8.0) + height / 16.0)
                .w(width / 8.0)
                .h(height / 8.0);
        }
    }
}

fn mouse_logic(app: &App, model: &Model) {
    if !app.mouse_buttons().any_just_pressed([MouseButton::Left]) {
        return;
    }
    let pos = app.mouse();

    let diff_x = (app.window_rect().left() - pos.x).abs();
    let diff_y = (app.window_rect().top() - pos.y).abs();

    let column = (diff_x / (app.window_rect().w() / 8.0)).floor() as usize;
    let row = (diff_y / (app.window_rect().h() / 8.0)).floor() as usize;

    println!("{:?}", (column, row));
}

fn view(app: &App, model: &Model) {

    draw_background(app);

    draw_pieces(app, model);

    mouse_logic(app, model);
}