Proton Mail needs your permission to
.
Proton Mail
Navigation

    Inbox
    38
    Drafts
    Sent
    Starred
    More

Folders
Manage your folders
Labels

    Manage your labels

Your current storage: 10.95 MB / 500.00 MB
10.95 MB / 500.00 MB

(No Subject)
From
Stored with zero-access encryption
John Colish<johncolish668@gmail.com>
Inbox
4:04 PMMonday, October 28th, 2024 at 4:04 PM
Tobowlarbear@proton.me
1.34 KB
1 file attached
Settings
All settings
Get the Proton Mail apps
Layout
Density
Composer
Keyboard shortcuts
Writing assistant
Run on device
Run on Proton servers
Theme:Proton
Sync with system

Beta Access
notes.rs
1of1
1. Store the game,
2. Update the game,
3. Display the game

let game = [
[*, f, *],
[*, f, *],
[p, *, *],
]

let map = [
p = player_ship,
f1 = fly,
f2 = fly
]

let map = [
(0, 2) = p
(1, 1) = f1
(1, 0) = f2
]

let map = [
p = (0, 2)
f1 = (1, 1)
f2 = (1, 0)
]


Method of Game
1. move the player ship
    1. Find the current player ship position,
    2. Set the current player position to *
    3. Set the new player position to p

pub enum SpaceObject{
    PlayerShip,
    Bee,
    Fly,
}

pub type Cords = (u8, u8);

Vec<Vec<SpaceObject>>

BTreeMap<Cords, SpaceObject>

Vec<(Cords, SpaceObject)> //Sorted top to bottom left to right

2,1

let player = (2,0);
let flys = [(0,1), (1,1)];
let bees = [(1,2)];


1. turn the maping into the vector
2. sort the vector by cords such that objects are left to right top to bottom
3. loop through every cord in the game and check if an obj exist print it or else empty space

width = 3;
height = 3;


for arr in game {
    for obj in arr {
        print(obj);
    }
}

for x in width {
    for y in height {
        if let Some(obj) = object.get((x, y)) {
            print(obj);
        } else {
            print("*");
        }
    }
}

***
***
***


let vec: Vec<u8> = vec![1,2,3];
let tup: (u8, String, bool) = (1, "Hi", true);

p = game[0,0] = 0,0

f = game[2,1] = 2,1


