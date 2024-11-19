use std::collections::BTreeMap;
use std::thread::sleep;
use std::time::Duration;


// Define grid boundaries (width and height)
const GRID_WIDTH: u8 = 5;
const GRID_HEIGHT: u8 = 5;

// --- Type Alias for Coordinates ---
type Cords = (u8, u8);

// Ship Trait
trait Ship {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords>;

    fn ship_tick(&mut self, current_position: Cords) -> Option<Cords> {
        self.update_pos(current_position)
    }

    fn symbol(&self) -> char;
}

// --- FlyShip Struct ---
struct FlyShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
}

impl Ship for FlyShip {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
        self.time_stationary = (self.time_stationary + 1) % 10;
        if self.time_stationary == 9 {
            let new_y = (current_position.1 + 1) % GRID_HEIGHT;
            Some((current_position.0, new_y))
        } else {None}
    }

    fn symbol(&self) -> char{
        'F'
    }
}

// --- BeeShip Struct ---
struct BeeShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
}

impl Ship for BeeShip {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
        Some((current_position.0, current_position.1 + 1))
    }

    fn symbol(&self) -> char{
        'B'
    }
}

// --- GameState Struct ---
struct GameState {
    game_board: BTreeMap<Cords, Box<dyn Ship>>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            game_board: BTreeMap::new(),
        }
    }
    //add a ship to the game board
    fn add_ship(&mut self, cords: Cords, ship: Box<dyn Ship>) {
        self.game_board.insert(cords, ship);
    }
    //remove a ship from the game board
    fn remove_ship(&mut self, cords: Cords) -> Option<Box<dyn Ship>>{
        self.game_board.remove(&cords)
    }
    //move an existing ship on the game board
    fn move_ship(&mut self, old_cords: Cords, new_cords: Cords){
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship);
        }
    }

    fn collision() {

    }

    fn print_frame(&self) {
        //clear the screen
        print!("\x1B[2J\x1B[H");
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cords = (x, y);
                if let Some(ship) = self.game_board.get(&cords) {
                    print!("{}", ship.symbol());
                // Simplified symbol for any ship
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    fn game_tick(&mut self) {
        //collect updates for new and old positions
        let updates: Vec<(Cords, Cords)> = self.game_board
        .iter_mut()
        .filter_map(|(&current_position, ship)| {
            let new_position = ship.ship_tick(current_position)?;
            Some((current_position, new_position))
        })
        .collect();
        //apply updates
        for (old_position, new_position) in updates {
            self.move_ship(old_position, new_position);
        }
    }
}

fn main() {
    let mut gamestate = GameState::new();
     // Step 1: Create a FlyShip instance
     let flyship = FlyShip {
        time_stationary: 0,
        past_positions: Vec::new(),
    };
    // Step 2: Insert the FlyShip into the `ships` map
    gamestate.add_ship((0,0), Box::new(flyship));
    // Step 3: Set its initial coordinates to (0, 0)
    loop {
        gamestate.game_tick();
        gamestate.print_frame();
        // Tick rate
        sleep(Duration::from_millis(500));
    }
}


// TODO:

    //USE STYLE DOC

    //RUN CARGO CLIPPY OFTEN