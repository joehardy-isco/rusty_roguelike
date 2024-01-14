mod map;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {
    map: Map,
}
impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}
//a trait defines shared functionality for objects. the trait below is 'GameState'. traits have to be implemented to have their functionality be available.
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        //ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}
fn main() -> BError {
    //per the comment below, you can take advantage of the question mark operator by making the main function return an error
    //handle errors in `Result`s with ?. ? passes the error to the parent function
    //(as opposed to matching to Ok or Err results,
    //or unwrapping the result [which immediately throws and error and halts code execution])

    let context = BTermBuilder::simple80x50()
        .with_title("Rusty RogueLike")
        .with_fps_cap(30.0)
        .build()?; //? works because main returns a BError (and so could build)
                   //context.print_centered(9,"Hello, Bracket World");
    main_loop(context, State::new()) //main_loop returns a BError, so no semicolon is needed (main_loop will cause main to return an error)
}
