// Based on Steve Klabnik's Adventure circa 2014
// https://github.com/steveklabnik/adventure

use std::io;

#[derive(PartialEq)]
enum Command {
    Go(Direction),
}


#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Exit {
    direction: Direction,
    target: usize, // the room number
}

impl Exit {
    fn can_go(&self, direction: &Direction) -> bool {
        self.direction == *direction
    }
}

struct Room {
    description: String,
    exits: Vec<Exit>,
}

impl Room {
    fn can_go(&self, direction: Direction) -> bool {
        self.exits.iter().find(|e| e.can_go(&direction)).is_some()
    }

    fn exit_to(&self, direction: Direction) -> Option<usize> {
        Some(
            self.exits
                .iter()
                .find(|e| e.direction == direction)
                .unwrap()
                .target,
        )
    }

    fn is_escape(&self) -> bool {
        self.exits.len() == 0
    }
}

fn main() {

    use Direction::East as east;
    use Direction::North as north;
    use Direction::South as south;
    use Direction::West as west;

    let mut rooms = vec![
        Room {
            description:
                "You are a righteous member of the band WYLD RUSTACEANS standing in the garage where you practice. You are in danger of flunking most heinously if you don't find Rufus' time machine. Sean's House is to the south and the high school is to the east."
                    .to_string(),
            exits: vec![
                Exit {
                    direction: south,
                    target: 2,
                },
                Exit {
                    direction: east,
                    target: 1,
                },
            ],
        },
        Room {
            description:
                "You stand in front of the high school. Grades are posted in the window. Sean has a total score of 30%. He'll need to ace his history test to pass the class. There is a garage to the west and a Circle K to the south."
                    .to_string(),
            exits: vec![
                Exit {
                    direction: west,
                    target: 0,
                },
                Exit {
                    direction: south,
                    target: 3,
                },
            ],
        },
        Room {
            description:
                "You walk into Sean's house. Sean's dad bellows: IF YOU FAIL THAT HISTORY CLASS, YOU'LL GO TO MILITARY SCHOOL. Bogus..."
                    .to_string(),
            exits: vec![Exit {
                direction: north,
                target: 0,
            }],
        },
        Room {
            description: "You stand in front of the Circle K. There is a school to the north and a phone booth to the south."
                .to_string(),
            exits: vec![
                Exit {
                    direction: north,
                    target: 1,
                },
                Exit {
                    direction: south,
                    target: 4,
                },
            ],
        },
        Room {
            description: "Dungeon exit".to_string(),
            exits: vec![],
        },
    ];

    let mut current_room = 0;

    println!("* * * SEAN & KAIT'S EXCELLENT ADVENTURE * * *\n\n");

    while !rooms[current_room].is_escape() {
        current_room = enter(rooms.get_mut(current_room).unwrap()).unwrap_or(current_room);
    }

    println!("After Kait and Sean climb into the phone booth, the booth begins flying through time and space beginning their MOST EXCELLENT ADVENTURE");
}

fn enter(room: &mut Room) -> Option<usize> {

    use Direction::East as east;
    use Direction::North as north;
    use Direction::South as south;
    use Direction::West as west;

    let go = Command::Go;

    let mut command: Option<Command> = None;

    while command == None {
        println!("{}", room.description);
        println!("\nWhat do you do?\n");

        for exit in room.exits.iter() {
            match exit.direction {
                north => println!("* Go (n)orth"),
                east => println!("* Go (e)ast"),
                south => println!("* Go (s)outh"),
                west => println!("* Go (w)est"),
            }
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        command = match input.trim() {
            "n" if room.can_go(north) => Some(go(north)),
            "e" if room.can_go(east) => Some(go(east)),
            "s" if room.can_go(south) => Some(go(south)),
            "w" if room.can_go(west) => Some(go(west)),
            _ => {
                println!("Please type a valid command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        Command::Go(north) => room.exit_to(north),
        Command::Go(east) => room.exit_to(east),
        Command::Go(south) => room.exit_to(south),
        Command::Go(west) => room.exit_to(west),
    }
}