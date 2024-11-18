use std::collections::BTreeMap;
use std::thread::sleep;
use std::time::Duration;


// --- SpaceObject Enum ---
enum SpaceObject {
    PlayerShip,
    Bee,
    Fly,
}

// --- Type Alias for Coordinates ---
type Cords = (u8, u8);
type ID = u32;

// Ship Trait
trait Ship {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords>;

    fn ship_tick(&mut self, current_position: Cords) -> Option<Cords> {
        self.update_pos(current_position)
    }
}

// Define grid boundaries (width and height)
const GRID_WIDTH: u8 = 5;
const GRID_HEIGHT: u8 = 5;

// --- FlyShip Struct ---
struct FlyShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    id: ID,
}

impl Ship for FlyShip {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
        self.time_stationary = self.time_stationary + 1 % 10;
        if self.time_stationary == 9 {
            let new_y = (current_position.1 + 1) % GRID_HEIGHT;
            Some((current_position.0, new_y))
        } else {None}
    }
}

// --- BeeShip Struct ---
struct BeeShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    id: ID,
}

impl Ship for BeeShip {
    fn update_pos(&mut self, current_position: Cords) -> Option<Cords> {
        Some((current_position.0, current_position.1 + 1))
    }
}

// --- GameState Struct ---
struct GameState {
    ships: BTreeMap<ID, Box<dyn Ship>>,
    positions: BTreeMap<Cords, ID>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            ships: BTreeMap::new(),
            positions: BTreeMap::new(),
        }
    }

    fn get_cords(&self, ship_id: ID) -> Option<Cords> {
        self.positions.iter().find_map(|(&cords, &id)| if id == ship_id { Some(cords) } else { None })
    }

    fn set_cords(&mut self, ship_id: ID, cords: Cords) {
        self.positions.insert(cords, ship_id);
    }

    fn print_frame(&self) {
        //clear the screen
        print!("\x1B[2J\x1B[H");
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cords = (x, y);
                if let Some(id) = self.positions.get(&cords) {
                    print!("S");
                // Simplified symbol for any ship
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    fn game_tick(&mut self) {
        // Step 1: Collect IDs and current positions into a temporary vector
        let ship_positions: Vec<(ID, Cords)> = self
            .positions
            .iter()
            .map(|(&cords, &id)| (id, cords))
            .collect();   
        // Step 2: Collect updates based on the ship positions
        let updates: Vec<(ID, Cords, Option<Cords>)> = ship_positions
            .into_iter()
            .filter_map(|(id, current_position)| {
                let updated_position = self.ships.get_mut(&id)?.ship_tick(current_position);
                Some((id, current_position, updated_position))
            })
            .collect();    
        // Step 3: Apply the updates
        for (id, old_position, updated_position) in updates {
            if let Some(new_position) = updated_position {
                // Remove the old position
                self.positions.remove(&old_position); 
                // Set the new position
                self.set_cords(id, new_position);     
            }
        }
    }

}

fn main() {
    let mut gamestate = GameState::new();
     // Step 1: Create a FlyShip instance with a unique ID (e.g., 1)
     let flyship = FlyShip {
        time_stationary: 0,
        past_positions: Vec::new(),
        id: 1,
    };
    // Step 2: Insert the FlyShip into the `ships` map
    gamestate.ships.insert(flyship.id, Box::new(flyship));
    // Step 3: Set its initial coordinates to (0, 0)
    gamestate.set_cords(1, (0, 0));
    loop {
        gamestate.game_tick();
        gamestate.print_frame();
        // Tick rate
        sleep(Duration::from_millis(500));
    }
}