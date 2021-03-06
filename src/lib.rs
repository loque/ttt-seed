#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use seed_icons::fa::solid::undo_alt;
use std::fmt::{Debug, Formatter, Result as FmtResult};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::new()
}
struct Model {
    state: State,
    turn: Player,
    winner: Option<Player>,
    board: Board,
}

impl Model {
    fn new() -> Self {
        Model {
            state: State::Idle,
            turn: Player::X,
            winner: None,
            board: [[None; 3]; 3],
        }
    }
}

enum State {
    Idle,
    Playing,
    Ended,
}

impl State {
    fn to_text(&self) -> &str {
        match self {
            State::Idle => "Idle",
            State::Playing => "Playing",
            State::Ended => "Ended",
        }
    }

    fn is(&self, test: &str) -> bool {
        self.to_text() == test
    }

    fn is_not(&self, test: &str) -> bool {
        self.to_text() != test
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_text())
    }
}

type Board = [BoardRow; 3];
type BoardRow = [Option<Player>; 3];

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn to_text(&self) -> &str {
        match self {
            Self::X => "X",
            Self::O => "O",
        }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_text())
    }
}

enum Msg {
    SelectPoint(Pos),
    Reset,
}

struct Pos {
    row_idx: usize,
    col_idx: usize,
}

fn did_player_win(board: &Board, player: &Player) -> bool {
    // check rows
    for row in board {
        let mut match_count = 0;
        for point in row {
            if let Some(point_player) = point {
                if point_player == player {
                    match_count = match_count + 1;
                }
            }
        }
        if match_count == 3 {
            return true;
        }
    }

    // check cols
    for col_idx in 0..2 {
        let mut match_count = 0;

        for row in board {
            let point = &row[col_idx];

            if let Some(point_player) = point {
                if point_player == player {
                    match_count = match_count + 1;
                }
            }
        }

        if match_count == 3 {
            return true;
        }
    }

    // check diagonal starting at (0,0)
    let mut match_count = 0;
    for i in 0..3 {
        if let Some(point_player) = &board[i][i] {
            if point_player == player {
                match_count = match_count + 1;
            }
        }
    }

    if match_count == 3 {
        return true;
    }

    // check diagonal starting at (0,2)
    let mut match_count = 0;
    for i in 0..3 {
        if let Some(point_player) = &board[i][2 - i] {
            if point_player == player {
                match_count = match_count + 1;
            }
        }
    }

    if match_count == 3 {
        return true;
    }

    false
}

fn did_game_end(board: &Board) -> bool {
    for row in board {
        for point in row {
            if point.is_none() {
                return false;
            }
        }
    }
    true
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SelectPoint(pos) => {
            // update the board
            model.board[pos.row_idx][pos.col_idx] = Some(model.turn);

            if did_player_win(&model.board, &model.turn) {
                model.winner = Some(model.turn);
                model.state = State::Ended;
            } else if did_game_end(&model.board) {
                model.state = State::Ended;
            } else {
                match model.turn {
                    Player::X => model.turn = Player::O,
                    Player::O => model.turn = Player::X,
                };

                model.state = State::Playing;
            }
        }
        Msg::Reset => {
            *model = Model::new();
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        header![h1![C!["text-shadow-black"], "Tic Tac Toe"]],
        section![
            C!["flex", "flex-col", "mb-8"],
            view_board(&model.board, &model.state)
        ],
        IF!( model.state.is_not("Ended") => view_turn(&model.turn)),
        IF!( model.state.is("Ended") => div![
            C!["flex", "flex-row", "items-center", "h-12"],
            vec![
                view_winner(&model.winner),
                view_reset()
            ]
        ]),
    ]
}

fn view_turn(turn: &Player) -> Node<Msg> {
    div![
        C!["flex", "flex-row", "items-center", "h-12", "gap-1.5"],
        span!["Player "],
        i![turn.to_text()],
        span![" turn"]
    ]
}

fn view_board(board: &Board, state: &State) -> Vec<Node<Msg>> {
    let mut board_content: Vec<Node<Msg>> = vec![];
    let state_is_not_ended = state.is_not("Ended");

    for (row_idx, row) in board.iter().enumerate() {
        let mut row_content: Vec<Node<Msg>> = vec![];

        for (col_idx, point) in row.iter().enumerate() {
            let point_content = match point {
                Some(player) => player.to_text(),
                None => "",
            };

            row_content.push(div![
                C![
                    "flex",
                    "justify-center",
                    "items-center",
                    "w-40",
                    "h-40",
                    "text-9xl",
                    "border",
                    "border-solid",
                    "border-black",
                ],
                point_content,
                ev(
                    Ev::Click,
                    move |_| IF!(state_is_not_ended => Msg::SelectPoint(Pos{ row_idx, col_idx }))
                )
            ])
        }

        board_content.push(div![C!["flex", "flex-row", "bg-white"], row_content])
    }

    board_content
}

fn view_winner(winner: &Option<Player>) -> Node<Msg> {
    let mut content = vec![];

    if winner.is_some() {
        content.push(span!["Player "]);
        content.push(i![winner.unwrap().to_text().to_owned()]);
        content.push(span![" won!"]);
    } else {
        content.push(span!["It's a tie!"]);
    }

    strong![C!["flex", "gap-1.5", "items-center"], content]
}

fn view_reset() -> Node<Msg> {
    button![
        C![
            "bg-green-500",
            "text-white",
            "shadow-green-500",
            "rounded-lg",
            "py-1",
            "px-3",
            "text-lg",
            "ml-8",
            "w-12",
            "h-12",
            // hover
            "hover:bg-green-400",
            "hover:shadow-green-400",
            // focus
            "focus:outline-none",
            "focus:border-transparent",
            // active
            "active:bg-green-600",
            "active:shadow-green-600",
            // animation
            "transition",
            "duration-200",
            "ease-in-out",
        ],
        undo_alt::i(),
        ev(Ev::Click, |_| Msg::Reset)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
