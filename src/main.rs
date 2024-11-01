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
    fn update_pos(&mut self, current_position: Cords) -> Cords;

    fn ship_tick(&mut self, current_position: Cords) -> Cords {
        self.update_pos(current_position)
    }
}

// --- FlyShip Struct ---
struct FlyShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    id: ID,
}

impl Ship for FlyShip {
    fn update_pos(&mut self, current_position: Cords) -> Cords {
        if self.time_stationary < 10{
            self.time_stationary += 1;
            //return pos without moving
            current_position
        }else{
        //reset time_stationary
        self.time_stationary = 0;
        // Movement logic for FlyShip
        (current_position.0, current_position.1.saturating_add(1)) 
        }
    }
}

// --- BeeShip Struct ---
struct BeeShip {
    time_stationary: u32,
    past_positions: Vec<Cords>,
    id: ID,
}

impl Ship for BeeShip {
    fn update_pos(&mut self, current_position: Cords) -> Cords {
        // Movement logic for BeeShip
        (current_position.0, current_position.1 + 1)
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
        for y in 0..3 {
            for x in 0..3 {
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
        // Step 1: Collect updates for old and new positions
        let updates: Vec<(ID, Cords, Cords)> = self
            .ships
            .iter_mut()
            .filter_map(|(&id, ship)| {
                // Get the current position from `positions` directly
                let current_position = self.positions.iter().find_map(|(&cords, &ship_id)| {
                    if ship_id == id {
                        Some(cords)
                    } else {
                        None
                    }
                })?;                
                // Calculate the updated position
                let updated_position = ship.ship_tick(current_position);
                Some((id, current_position, updated_position))
            })
            .collect();        
            // Step 2: Apply the updates by removing old positions and setting new ones
        for (id, old_position, updated_position) in updates {
            // Remove the old position
            self.positions.remove(&old_position);
            // Set the new position
            self.set_cords(id, updated_position);
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



// // Method of Game
// // 1. move the player ship
// //     1. Find the current player ship position,
// //     2. Set the current player position to *
// //     3. Set the new player position to p

// pub enum SpaceObject{
//     PlayerShip,
//     Bee,
//     Fly,
// }

// pub type Cords = (u8, u8);

// Vec<Vec<SpaceObject>>

// BTreeMap<Cords, SpaceObject>

// Vec<(Cords, SpaceObject)> //Sorted top to bottom left to right

// 2,1

// let player = (2,0);
// let flys = [(0,1), (1,1)];
// let bees = [(1,2)];


// // 1. turn the mapping into the vector
// // 2. sort the vector by cords such that objects are left to right top to bottom
// // 3. loop through every cord in the game and check if an obj exist print it or else empty space

// // width = 3;
// // height = 3;


// // for arr in game {
// //     for obj in arr {
// //         print(obj);
// //     }
// // }

// // for x in width {
// //     for y in height {
// //         if let Some(obj) = object.get((x, y)) {
// //             print(obj);
// //         } else {
// //             print("*");
// //         }
// //     }
// // }

// // ***
// // ***
// // ***


// let vec: Vec<u8> = vec![1,2,3];
// let tup: (u8, String, bool) = (1, "Hi", true);

// p = game[0,0] = 0,0

// f = game[2,1] = 2,1

// trait Ship {
//     //Required Method
//     pub fn update_pos(&mut self, current_position: Cords) -> Cords;

//     //Provided Method
//     pub fn ship_tick(&mut self, current_position: Cords) -> Cords {
//         self.update_pos(current_position)
//     }
// }

// struct FlyShip {
//     time_stationary: u32,
//     past_positions: Vec<Cords>,
//     id: ID,
// }

// impl Ship for FlyShip {
//     pub fn update_pos(&mut self, current_position: Cords) -> Cords {
//         return current_position+2;
//     }
// }

// struct BeeShip {
//     time_stationary: u32,
//     past_positions: Vec<Cords>,
//     id: ID,
// }

// impl Ship for BeeShip {
//     pub fn update_pos(&mut self, current_position: Cords) -> Cords {
//         return current_position+1;
//     }
// }

// impl BeeShip {
//     pub fn ship_tick(&mut self, current_position: Cords) -> Cords {
//       //1. update time_stationary
//       //2. update current_position and any meta data related to position
//         let updated_position = self.update_pos(current_position);
//         return updated_position;
//     }
// }

// struct GameState {
//     ships: BTreeMap<ShipId, Ship>,
//     state: (BTreeMap<Cords, ShipId>, BTreeMap<Ship, Cords>)
// }

// impl GameState {
//     pub fn new() -> Self;
//     pub fn get_cords(&mut self, ship_id: ID) -> Cords;
//     pub fn set_cords(&mut self, ship_id: ID, cords: Cords);
//     pub fn remove_ship(&mut self, ship_id: ID);
//     pub fn print_frame(&self);
//     pub fn ships(&self) -> Vec<&Ships>;

//     pub fn game_tick(&mut self) {
//         for ship in self.ships() {
//             let updated_position = ship.ship_tick(self.get_cords(ship.id));
//             self.set_cords(ship.id, updated_position);
//         }
//     }
// }

// let gamestate = GameState::new();
// loop {
//     //Player ship if stdin == ">" {set_cords
//     gamestate.game_tick();
//     gamestate.print_frame();
//     sleep(tickrate);
// }