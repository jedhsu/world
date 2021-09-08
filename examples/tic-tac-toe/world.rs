pub struct Position {
    state: PositionState,
}

pub enum PositionState {
    White,
    Black,
    Empty,
}

impl PositionState {
    fn initialize(&self) -> &Self {
        return 
    }
}

pub struct TicTacToe {
    board_size: u16,
    players: u16,
}

impl World for TicTacToe {
    fn initialize(&self);
}

pub struct Test:
    //! In Tic-Tac-Toe, a block quantum is not allowed to move. It can only switch states
    //! conditional on the player selecting the action.

    pub struct TicTacToeBlockActions(
        QuantumAction,
    ):
        fn actions(
            &self,
            action_space: type,
        ) -> set<QuantumAction> {
            actions = set()

            # <TODO> explore changing this to type level
            actions.add(
                QuantumAction(
                    &self.position,
                    &self.action.Circle,
                    condition=Action.place_circle(position),
                )
            )
            return actions }
        }
}
pub enum Position {

}

pub trait Symmetry {
    fn rotate(&self, x: i32, y: i32) -> (i32, i32);
    fn vflip(&self, x: i32, y: i32) -> (i32, i32);
    fn ap(&self, eval: Fn) -> Fn;
    fn sym(&self, eval: Fn) -> Fn;
    // fn dihedral_symmetries(&self, eval: Fn) -> Fn;
}

//! can be a lot cleaner...
impl Symmetry for TicTacToe {
    const N = &Self.BOARD_SIDE;

    fn rotate(
        x: i32,
        y: i32,
    ) -> (i32, i32) {
        (y, N - x + 1)
    }

    fn vflip(
        x: i32,
        y: i32,
    ) -> (i32, i32) {
        ( x, cls.N - y + 1,)
    }

    fn ap(&self, eval: Fn) -> Fn {
        pos_of_xy(fn(xy_of_pos(pos)))
    }

    fn sym(&self, eval: Fn) -> Fn { }

    // fn dihedral_symmetries():
    //     pass
}

pub struct Test:
    pub struct TicTacToe_Space(
        Space,
    ):
        fn construct(&self):
            return super().construct(
                product(
                    <1, 2, 3>,
                    <1, 2, 3>,
                )
            )
