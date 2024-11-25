use std::collections::BTreeMap;
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};


// Define grid boundaries (width and height)
const GRID_WIDTH: u8 = 5;
const GRID_HEIGHT: u8 = 5;

// --- Type Alias for Coordinates ---
type Cords = (u8, u8);

// Ship Trait
trait Ship {
    // fn update_pos(&mut self, current_position: Cords) -> Option<Cords>;

    // fn ship_tick(&mut self, current_position: Cords) -> Option<Cords> {
    //     self.update_pos(current_position)
    // }

    fn get_ai(&mut self) -> Option<&mut ShipAI>;

    fn symbol(&self) -> char;
}

enum ShipAction{
    Move(i8, i8),
    None,
}

struct ShipAI{
    current_tick: u64,
    tick_interval: u64,
    actions: Vec<ShipAction>,
};


impl ShipAI{
    fn new(tick_interval: u64) -> Self {
        ShipAI{
            current_tick: 0,
            tick_interval: tick_interval,
            actions: vec![ShipAction::Move(1,0)],
        }
    }

    fn tick(&mut self) -> ShipAction{
        self.current_tick = (self.current_tick + 1) % self.tick_interval;
        if self.current_tick == 0 {
            self.actions[0]
        } else {
            ShipAction::None
        }
    }
}

// --- PlayerShip Struct ---
struct PlayerShip;

impl Ship for PlayerShip{
    //detect player inputs
    // fn update_pos(&mut self, current_position: Cords) -> Option<Cords>{
    //     Some(current_position)
    // }

    fn get_ai(&mut self) -> Option<&mut ShipAI>{
        None
    }

    fn symbol(&self) -> char{
        'P'
    }
}

// --- FlyShip Struct ---
struct FlyShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    ship_ai: ShipAI,
}

impl Ship for FlyShip {
    // fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
    //     self.time_stationary = (self.time_stationary + 1) % 10;
    //     if self.time_stationary == 9 {
    //         let new_y = (current_position.1 + 1) % GRID_HEIGHT;
    //         Some((current_position.0, new_y))
    //     } else {None}
    // }

    fn get_ai(&mut self) -> Option<&mut ShipAI>{
        Some(self.ship_ai)
    }

    fn symbol(&self) -> char{
        'F'
    }
}

// --- BeeShip Struct ---
struct BeeShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    ship_ai: ShipAI,
}

impl Ship for BeeShip {
    // fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
    //     Some((current_position.0, current_position.1 + 1))
    // }

    fn get_ai(&mut self) -> Option<&mut ShipAI>{
        Some(self.ship_ai)
    }

    fn symbol(&self) -> char{
        'B'
    }
}


// --- Explosion Struct ---
struct Explosion {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    ship_ai: ShipAI,
}

impl Ship for Explosion{

    // fn update_pos(&mut self, current_position: Cords) -> Option<Cords>{
    //     Some((current_position.0, current_position.1))
    // }
    fn get_ai(&mut self) -> Option<&mut ShipAI>{
        Some(self.ship_ai)
    }

    fn symbol(&self) -> char{
        '#'
    }
}

enum GameAction {
    Move(Cords, Cords),
    Add(Cords, Box<dyn Ship>),
    Remove(Cords),
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

    // fn game_tick(&mut self) {
    //     //collect updates for new and old positions
    //     let updates: Vec<(Cords, Cords)> = self.game_board
    //     .iter_mut()
    //     .filter_map(|(&current_position, ship)| {
    //         let new_position = ship.ship_tick(current_position)?;
    //         Some((current_position, new_position))
    //     })
    //     .collect();
    //     //apply updates
    //     for (old_position, new_position) in updates {
    //         self.move_ship(old_position, new_position);
    //     }
    // }

    fn game_tick(&mut self){
        let gb: &mut BTreeMap<Cords, Box<dyn Ship>> = self.game_board;
        let iter: IterMut<(&Cords, &mut Box<dyn Ship>)> = gb.iter_mut();
        let new_iter: IterMut<ShipAction> = iter.map(|(cords, ship): (&Cords, &mut Box<dyn Ship>)| {
            let opt_ai: Option<&mut ShipAI> = ship.get_ai();
            let a: Option<ShipAction> = opt_ai.map(|ai: &mut ShipAI|{
                ai.tick()
            }); 
            (a.unwrap_or(ShipAction::None), cords)
        });
        new_iter.map(|sa: ShipAction|{
            match sa {
                ShipAction::Move(x, y) => GameAction::Move((), ())
            }
        });
    }
}

fn main() {
    let mut gamestate = GameState::new();
    //Create a PlayerShip instance
    let playership = PlayerShip;
    gamestate.add_ship((2, GRID_HEIGHT - 1), Box::new(playership));
    //Create a FlyShip instance
    let flyship = FlyShip {
        time_stationary: 0,
        past_positions: Vec::new(),
        ship_ai: ShipAI::new(10),
    };
    // Insert the FlyShip into the `ships` map
    gamestate.add_ship((0,0), Box::new(flyship));
    // Set its initial coordinates to (0, 0)
    loop {
        gamestate.game_tick();
        gamestate.print_frame();
        // Tick rate
        sleep(Duration::from_millis(100));
    }
}


// TODO:

    //USE STYLE DOC

    //RUN CARGO CLIPPY OFTEN