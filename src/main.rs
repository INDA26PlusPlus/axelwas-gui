use std::net::TcpListener;
use std::{io::Read, net::TcpStream, time::Duration};
use std::io::Write;

use chess_box::moves::Move;
use chess_box::{board::Square, game::MoveError, pieces::{ChessPiece, PieceType}};
use nannou::prelude::*;

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

#[derive(Debug, Clone)]
enum ConnectionState {
    OurTurn,
    WaitOK(((usize, usize), (usize, usize), Option<PieceType>)),
    TheirTurn,
    Disconnect(String)
}


enum Connection {
    Open(TcpStream, ConnectionState),
    Listening(TcpListener),
    Dialog(String),
    EndScreen
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MoveState {
    None,
    Highlighted((usize, usize)),
    PendingPromotion(((usize, usize), (usize, usize))),
    ToMove(((usize, usize), (usize, usize), Option<PieceType>)),
    Reject
}

struct Model {
    textures: Textures,
    game: chess_box::game::ChessGame,
    chess_move: MoveState,
    connection: Connection,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).view(view).build();

    Model { 
        textures: Textures::load(app), 
        game: chess_box::game::ChessGame::new_standard_game(), 
        chess_move: MoveState::None, 
        connection: Connection::Dialog(String::new())
    }
}

fn draw_background(app: &App, model: &Model) {
    let draw = app.draw();

    let win = app.window_rect();
    let width = win.right() - win.left();
    let height = win.top() - win.bottom();

    for c in 0..8 {
        for r in 0..8 {
            let light = Color::srgb(240.0 / 256.0, 217.0 / 256.0, 181.0 / 256.0);
            let dark = Color::srgb(181.0 / 256.0, 136.0 / 256.0, 99.0 / 256.0);
            let highlighted_color = Color::srgb(1.0, 0.0, 0.0);
            
            let mut to_use = if (c + r) % 2 == 1 {
                light
            } else {
                dark
            };

            if MoveState::Highlighted((c, r)) == model.chess_move {
                to_use = highlighted_color;
            }

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

            
            if piece.is_white() == model.game.is_white_turn() && model.game.in_check() && piece.piece_type() == PieceType::King {
                draw.ellipse().color(Color::srgba(1.0, 0.0, 0.0, 0.5))
                    .w(width / 9.0)
                    .h(height / 9.0)
                    .x(win.left() + c as f32 * (width / 8.0) + width / 16.0)
                    .y(win.bottom() + r as f32 * (height / 8.0) + height / 16.0);
            }
            draw.rect().texture(model.textures.image_from_type(&piece.piece_type(), piece.is_white()))
                .x(win.left() + c as f32 * (width / 8.0) + width / 16.0)
                .y(win.bottom() + r as f32 * (height / 8.0) + height / 16.0)
                .w(width / 8.0)
                .h(height / 8.0);
        }
    }
}

fn draw_dialog(app: &App, model: &Model) {
    match model.chess_move { 
        MoveState::PendingPromotion(_) => (),
        _ => return,
    };

    let draw = app.draw();
    draw.rect().color(Color::srgb(0.0, 0.0, 0.0)).x(0.0).y(0.0).w(300.0).h(300.0);
    draw.text("♕ Queen\n♖ Rook\n♘ Knight\n♗ Bishop").font_size(40).x(0.0).y(0.0); 

}

fn dialog_logic(app: &App, model: &mut Model) {
    let (from, to) = match model.chess_move {
        MoveState::PendingPromotion(s) => s,
        _ => return,
    };
    let o = (app.mouse().y / 40.0).floor();

    // model.game.make_promotion_move(start, stop, promotion_choice)

    let promotion_choice = match o {
        1.0 => PieceType::Queen,
        0.0 => PieceType::Rook,
        -1.0 => PieceType::Knight,
        -2.0 => PieceType::Bishop,
        _ => return 
    };

    // model.game.make_promotion_move(from, to, promotion_choice).unwrap();
    model.chess_move = MoveState::ToMove((from, to, Some(promotion_choice)));
}

fn draw_connection_dialog(app: &App, model: &Model) {
    let draw = app.draw();

    if let Connection::Dialog(text) = &model.connection {
        draw.rect().color(Color::BLACK).x(0.0).y(0.0).w(400.0).h(400.0);
        draw.text(format!("Please connect to a server.\naddress: {}", text).as_str()).color(Color::WHITE).font_size(30);
        return;
    }

    match model.connection {
        Connection::Listening(_) => {
            draw.rect().color(Color::BLACK).x(0.0).y(0.0).w(400.0).h(200.0);
            draw.text("Listening on port 6767.\nYou will be white on game start").color(Color::WHITE).font_size(25);
        },
        _ => return,
    };
}

fn draw_end(app: &App, model: &Model) {
    if model.game.in_checkmate() {
        app.draw().rect().color(Color::BLACK).x(0.0).y(0.0).w(400.0).h(140.0);
        let color_win = match model.game.is_white_turn() {
            true => "BLACK",
            false => "WHITE"
        };
        
        app.draw().text(format!("CHECKMATE!!\n{} WINS!!", color_win).as_str()).font_size(50).x(0.0).y(0.0);
    }
    if model.game.in_stalemate() {
        app.draw().rect().color(Color::BLACK).x(0.0).y(0.0).w(400.0).h(70.0);
        app.draw().text("STALEMATE D:").font_size(50).x(0.0).y(0.0);
    }
}

fn mouse_logic(app: &App, model: &mut Model) {
    match model.connection {
        Connection::Dialog(_) => return,
        _ => ()
    };

    if !app.mouse_buttons().any_just_pressed([MouseButton::Left]) {
        return;
    }
    match model.chess_move {
        MoveState::Reject => {
            model.chess_move = MoveState::None;
            return;
        },
        _ => ()
    };

    match model.chess_move {
        MoveState::PendingPromotion(_) => { dialog_logic(app, model); return; }
        _ => ()
    };


    let pos = app.mouse();

    let diff_x = (app.window_rect().left() - pos.x).abs();
    let diff_y = (app.window_rect().bottom() - pos.y).abs();

    let column = (diff_x / (app.window_rect().w() / 8.0)).floor() as usize;
    let row = (diff_y / (app.window_rect().h() / 8.0)).floor() as usize;

    if model.chess_move == MoveState::Highlighted((column, row)) {
        model.chess_move = MoveState::None;
        return;
    }     

    let high = match model.chess_move {
        MoveState::None => {model.chess_move = MoveState::Highlighted((column, row)); return;},
        MoveState::Highlighted(h) => h, 
        MoveState::ToMove((_, _, _)) => return,
        MoveState::Reject => return,
        MoveState::PendingPromotion(_) => return,
    };

    let from = Square::new_square_from_index(high.0 as i8, high.1 as i8).unwrap();
    let to = Square::new_square_from_index(column as i8, row as i8).unwrap();

    let mut g_clone = model.game.clone();

    let result = g_clone.make_move(Some(from), Some(to));

    match result {
        Err(MoveError::InvalidPromotion) => model.chess_move = MoveState::PendingPromotion((high, (column, row))),
        Err(_) => {
            model.chess_move = MoveState::None;
            return;
        },
        Ok(_) => model.chess_move = MoveState::ToMove((high, (column, row), None))
    };

}

fn actualize_connection(model: &mut Model) {
    let ip = match &model.connection {
        Connection::Dialog(s) => s,
        _ => return
    };

    if ip.len() == 0 {
        model.connection = Connection::Listening(TcpListener::bind("0.0.0.0:6767").unwrap());
        return;
    }

    match TcpStream::connect(format!("{}:6767", ip)) {
        Ok(mut o) => {
            let mut buf = [2;2];
            match o.read(&mut buf) {
                Ok(_) => (),
                Err(_) => {model.connection = Connection::Dialog("Protocol error.".to_string()); return},
            };
            let start_state = if buf == *b"W\n" {
                ConnectionState::OurTurn
            } else if buf == *b"B\n" {
                ConnectionState::TheirTurn
            } else {
                model.connection = Connection::Dialog("Protocol error.".to_string());
                return;
            };

            o.set_read_timeout(Some(Duration::from_millis(1))).unwrap();

            model.connection = Connection::Open(o, start_state);
        },
        Err(e) => model.connection = Connection::Dialog(format!("error while connecting.\n{}", e))
    }
}

fn listening_logic(model: &mut Model) {
    let listen = match &mut model.connection {
        Connection::Listening(l) => l,
        _ => return,
    };

    listen.set_nonblocking(true).unwrap();

    let (mut stream, _) = match listen.accept() {
        Err(_) => return,
        Ok(s) => s
    };

    stream.write(b"B\n").unwrap();

    stream.set_read_timeout(Some(Duration::from_millis(1))).unwrap();

    model.connection = Connection::Open(stream, ConnectionState::OurTurn);
}

fn connection_dialog_logic(app: &App, model: &mut Model) {
    
    let text = match &mut model.connection {
        Connection::Dialog(s) => s,
        _ => return,
    };

    macro_rules! a {
        ($text_to_add:expr) => {
            *text = format!("{}{}", text, $text_to_add)
        }
    }

    match app.keys().get_just_pressed().next() {
        Some(KeyCode::Digit0) => a!("0"),
        Some(KeyCode::Digit1) => a!("1"),
        Some(KeyCode::Digit2) => a!("2"),
        Some(KeyCode::Digit3) => a!("3"),
        Some(KeyCode::Digit4) => a!("4"),
        Some(KeyCode::Digit5) => a!("5"),
        Some(KeyCode::Digit6) => a!("6"),
        Some(KeyCode::Digit7) => a!("7"),
        Some(KeyCode::Digit8) => a!("8"),
        Some(KeyCode::Digit9) => a!("9"),
        Some(KeyCode::Period) => a!("."),
        Some(KeyCode::Backspace) => drop(text.pop()),
        Some(KeyCode::Enter) => actualize_connection(model),
        _ => ()
    };

}

fn their_turn(line: String, model: &mut Model) -> ConnectionState {
    

    let from = &line[0..2];
    let to = &line[2..4];
    let promotion = line.chars().nth(4).unwrap();
    let board = &line[5..(5 + 64)];
    
    let from = match Square::try_from(from) {
        Ok(s) => s,
        Err(_) => return ConnectionState::Disconnect("Invalid from square notation".to_string()),
    };

    let to = match Square::try_from(to) {
        Ok(s) => s,
        Err(_) => return ConnectionState::Disconnect("invalid to square notation".to_string()),
    };

    let promotion = PieceType::try_from(promotion).ok();

    let res;
    if let Some(promotion) = promotion {
        res = model.game.make_promotion_move(Some(from), Some(to), promotion);
    } else {
        res = model.game.make_move(Some(from), Some(to));
    }

    let stream = match &mut model.connection {
        Connection::Open(st, _) => st,
        _ => return ConnectionState::Disconnect("Dissagreeing internal state".to_string())
    };



    if res == Ok(()) {
        let s: String = (*model.game.board()).into();
        if board.eq(s.as_str()) {
            match stream.write(b"OK\n") {
                Err(_) => return ConnectionState::Disconnect("Unable to write to stream".to_string()),
                Ok(_) => ()
            };

            return ConnectionState::OurTurn;
        }
    }

    match stream.write(b"REJECT\n") {
        Err(_) => return ConnectionState::Disconnect("Unable to write to stream".to_string()),
        Ok(_) => ()
    };

    ConnectionState::TheirTurn
}

fn our_turn(model: &mut Model) -> ConnectionState {
    let (stream, state) = match &mut model.connection {
        Connection::Open(st, s) => (st, s),
        _ => return ConnectionState::Disconnect("Invalid internal state".to_string())
    };

    let (to, from, promotion) = match model.chess_move {
        MoveState::ToMove((t, f, p)) => (t, f, p),
        _ => return state.clone(),
    };

    let to_str: String = Square::new_square_from_index(to.0 as i8, to.1 as i8).unwrap().into();
    let from_str: String = Square::new_square_from_index(from.0 as i8, from.1 as i8).unwrap().into();
    let promotion_ch = promotion.map(|p| p.into()).unwrap_or('-');
    
    let from_sq = Some(Square::new_square_from_index(from.0 as i8, from.1 as i8).unwrap());
    let to_sq = Some(Square::new_square_from_index(to.0 as i8, to.1 as i8).unwrap());

    let mut game_clone = model.game.clone();

    match promotion {
        None => game_clone.make_move(to_sq, from_sq).unwrap(),
        Some(p) => game_clone.make_promotion_move(to_sq, from_sq, p).unwrap(),
    }
    
    
    let board_str: String = (*game_clone.board()).into();
    



    let to_send = format!("{}{}{}{}\n", to_str, from_str, promotion_ch, board_str);

    match stream.write(to_send.as_bytes()) {
        Ok(_) => (),
        Err(e) => return ConnectionState::Disconnect(format!("Cannot write to strean: {}", e))
    };

    ConnectionState::WaitOK((to, from, promotion))
}

fn wait_ok(model: &mut Model) -> ConnectionState {
    let (stream, state) = match &mut model.connection {
        Connection::Open(st, s) => (st, s),
        _ => return ConnectionState::Disconnect("Invalid internal state".to_string())
    };

    let (from, to, promotion) = match state {
        ConnectionState::WaitOK((f, t, p)) => (f, t, p),
        _ => return ConnectionState::Disconnect("Invalid internal state".to_string())
    };

    let mut msg = String::new();

    let mut buf = [0; 1];
    while buf != [10] {
        match stream.read(&mut buf) {
            Ok(_) => (),
            Err(_) => return state.clone(),
        };
        msg.push(char::from(buf[0]));
    }



    if msg.starts_with("OK\n") {
        model.chess_move = MoveState::None; 

        let from = Some(Square::new_square_from_index(from.0 as i8, from.1 as i8).unwrap());
        let to = Some(Square::new_square_from_index(to.0 as i8, to.1 as i8).unwrap());

        match promotion {
            None => model.game.make_move(from, to).unwrap(),
            Some(p) => model.game.make_promotion_move(from, to, *p).unwrap(),
        }

        return ConnectionState::TheirTurn;
    } else if msg.starts_with("REJECT\n") {
        model.chess_move = MoveState::Reject;

        return ConnectionState::OurTurn;



    } else if msg.starts_with("CHECKMATE\n") {
        model.chess_move = MoveState::None; 

        let from = Some(Square::new_square_from_index(from.0 as i8, from.1 as i8).unwrap());
        let to = Some(Square::new_square_from_index(to.0 as i8, to.1 as i8).unwrap());

        match promotion {
            None => model.game.make_move(from, to).unwrap(),
            Some(p) => model.game.make_promotion_move(from, to, *p).unwrap(),
        }

        return ConnectionState::Disconnect("CHECKMATE".to_string());
    } else if msg.starts_with("STALEMATE\n") {
        model.chess_move = MoveState::None; 

        let from = Some(Square::new_square_from_index(from.0 as i8, from.1 as i8).unwrap());
        let to = Some(Square::new_square_from_index(to.0 as i8, to.1 as i8).unwrap());

        match promotion {
            None => model.game.make_move(from, to).unwrap(),
            Some(p) => model.game.make_promotion_move(from, to, *p).unwrap(),
        }

        return ConnectionState::Disconnect("STALEMATE".to_string());
    } else {
        return ConnectionState::Disconnect("Protocol error must answer OK or REJECT".to_string());
    }
}

fn connection_connection_logic(model: &mut Model) {
    
    let state = match &model.connection {
        Connection::Open(_, st) => st,
        _ => return,
    };

    dbg!(state);

    let to_state = match state {
        ConnectionState::Disconnect(msg) => {
            println!("dissconnecting because of: {}", msg);

            if model.game.in_checkmate() || model.game.in_stalemate() {
                model.connection = Connection::EndScreen
            } else {
                model.connection = Connection::Dialog(String::new());
            }

            return;
        },
        ConnectionState::OurTurn => {
            let to_state = our_turn(model);
            to_state
        },
        ConnectionState::WaitOK(_) => {
            let to_state = wait_ok(model);
            to_state
        }
        ConnectionState::TheirTurn => {
            model.chess_move = MoveState::None;

            let stream = match &mut model.connection {
                Connection::Open(st, _) => st,
                _ => return,
            };

            let mut buf = [0; 1700];

            let read = stream.read(&mut buf);
            match read {
                Err(_) => return,
                Ok(0) => return,
                Ok(i) => dbg!(i),
            };

            let taken_line = String::from_utf8_lossy(&buf).to_string();            
            println!("{}", taken_line);
            their_turn(taken_line, model)
        }
    };

    model.connection = match &model.connection {
        Connection::Open(s, _st) => Connection::Open(s.try_clone().unwrap(), to_state),
        _ => return,
    };

}

fn connection_logic(app: &App, model: &mut Model) {
    connection_dialog_logic(app, model);
    connection_connection_logic(model);
    listening_logic(model);
} 

fn draw_reject(app: &App, model: &Model) {
    match model.chess_move {
        MoveState::Reject => (),
        _ => return,
    }

    let draw = app.draw();
    draw.rect().w(300.0).h(70.0).color(Color::BLACK).x(0.0).y(0.0);
    draw.text("REJECTED").font_size(30).x(0.0).y(0.0);
}

fn view(app: &App, model: &Model) {

    draw_background(app, model);

    draw_pieces(app, model);

    draw_dialog(app, model);

    draw_end(app, model);

    draw_connection_dialog(app, model);

    draw_reject(app, model)
}

fn update(app: &App, model: &mut Model) {
    mouse_logic(app, model);

    connection_logic(app, model);
}