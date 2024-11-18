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
        self.time_stationary = (self.time_stationary + 1) % 10;
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
        //iterate over the positions map, storing the current coordinates of each ship according to it's ID
        let ship_positions: Vec<(ID, Cords)> = self
            .positions
            .iter()
            //transform each key value pair into a tuple by deferencing the keys and values
            .map(|(&cords, &id)| (id, cords))
            //convert the iterator into a vector of ship IDs and their current coords
            .collect();   
        //use the collected positions to calcuate updates
        let updates: Vec<(ID, Cords, Option<Cords>)> = ship_positions
            //consume the ship position vector, creating an iterator that produces each ID, Coords tuple
            .into_iter()
            //process each ship ID & current position to determine the updated position
            .filter_map(|(id, current_position)| {
                //retrieve a mutable reference to the ship object associated with the given ID, allowing the ship's state to be modified
                let updated_position = self.ships.get_mut(&id)?
                //call ship_tick method, passing in the current position, which will calculate the next position (if any), return an Option<Coords> or None if ship doesn't move this tick
                .ship_tick(current_position);
                //wrap the ship ID, current position, and result of the updated position into a Option<tuple>
                Some((id, current_position, updated_position))
                //if updated_position was None return None
            })
            //convert the iterator of tuples into a vector
            .collect();    
        //iterate through the the calculated updates vector to remove old positions and set new positions
        for (id, old_position, updated_position) in updates {
            //check if the updated position is Some, if it was None then assume the ship didn't move (implicitly skipped)
            if let Some(new_position) = updated_position {
                // Remove the old position from the positions map
                self.positions.remove(&old_position); 
                // Set the new position of the corresponding ship ID in the positions map
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