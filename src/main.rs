use std::fmt;
use std::io;

fn main() {
    println!("Hello, world!");

    let ships = vec![
        Ship::new(ShipType::AircraftCarrier),
        Ship::new(ShipType::Battleship),
        Ship::new(ShipType::Cruiser),
        Ship::new(ShipType::Submarine),
        Ship::new(ShipType::Destroyer)
    ];

    let mut board = Board::new(10, 10);
    board.draw();

    for ship in ships.iter() {
        println!();
        println!("> Placing {}, size {}", ship.ship_type.name(), ship.size());
        loop {

            board.draw();

            let placement = read_placement();

            match board.place_ship(placement, ship) {
                Result::Ok(_) => {
                    println!("ship placed");
                    break;
                },
                Result::Err(msg) => println!("could not place ship: {}", msg)
            }

        }
    }

    println!("--< all ships places >------------------------------------------");
    board.draw();
}

fn read_placement() -> ShipPlacement {
    println!("enter coordinate (x y):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("could not read input");

    let parts: Vec<&str> = input.trim().split(' ').collect();

    let x: u8 = parts[0].parse().expect("number expected");
    let y: u8 = parts[1].parse().expect("number expected");

    let coordinate = Coordinate{x, y};

    let mut input = String::new();
    println!("enter orientation (h v):");
    io::stdin().read_line(&mut input).expect("could not read orientation");

    let orientation = if input.trim() == "h" {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    };

    ShipPlacement{coordinate, orientation}
}

#[derive(Copy, Clone)]
struct Coordinate {
    x: u8,
    y: u8
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

enum Orientation {
    Horizontal,
    Vertical
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "horizontal"),
            Orientation::Vertical => write!(f, "vertical")
        }
    }
}

impl Orientation {
    fn next(&self, coordinate: &Coordinate) -> Coordinate {
        match self {
            Orientation::Horizontal => Coordinate{x: coordinate.x + 1, .. *coordinate},
            Orientation::Vertical => Coordinate{y: coordinate.y + 1, .. *coordinate}
        }
    }
}

struct ShipPlacement {
    coordinate: Coordinate,
    orientation: Orientation
}

impl ShipPlacement {
    fn coordinates(&self, length: u8) -> Vec<Coordinate> {
        let mut coords = Vec::new();
        let mut current: Coordinate = self.coordinate;

        for _ in 0..length {
            coords.push(current);
            current = self.orientation.next(&current);
        }

        coords
    }
}

struct BoardCell<'a> {
    ship: Option<&'a Ship>
}

struct Board<'a> {
    width:  u8,
    height: u8,
    cells: Vec<BoardCell<'a>>
}

impl<'a> Board<'a> {
    fn new(width: u8, height: u8) -> Board<'a> {
        let area: usize = (width * height) as usize;
        let mut cells: Vec<BoardCell> = Vec::with_capacity(area);
        for _ in 0..(area) {
            cells.push(BoardCell{ship: None});
        }

        Board{ width, height, cells }
    }

    fn place_ship(&mut self, placement: ShipPlacement, ship: &'a Ship) -> Result<bool, &str> {
        for c in placement.coordinates(ship.health) {
            if !self.valid_coordinate(&c) {
                return Err("invalid coordinate");
            }

            if self.has_ship_at(&c) {
                return Err("overlaps with ship");
            }
        }

        for c in placement.coordinates(ship.health) {
            self.set_ship_at(ship, &c);
        }

        Ok(true)
    }

    fn set_ship_at(&mut self, ship: &'a Ship, coordinate: &Coordinate) {
        let index = self.coordinate_to_index(coordinate);
        self.cells[index].ship = Some(ship);
    }

    fn has_ship_at(&self, coordinate: &Coordinate) -> bool {
        match self.cell_at(coordinate).ok() {
            None => false,
            Some(cell) => cell.ship.is_some()
        }
    }

    fn draw(&self) {
        print!("  ");
        for x in 0..self.width {
            print!(" {}", x + 1);
        }
        println!();
        for y in 1..self.height + 1 {
            print!("{:>2} ", y);

            for x in 1..self.width + 1 {
                let cell = self.cell_at(&Coordinate{x, y}).unwrap();
                let output = match cell.ship {
                    Some(ship) => "S ",
                    None => "~ "
                };
                print!("{}", output);
            }

            println!();
        }
    }

    fn valid_coordinate(&self, coordinate: &Coordinate) -> bool {
        self.cell_at(coordinate).is_ok()
    }

    fn coordinate_to_index(&self, coordinate: &Coordinate) -> usize {
        (coordinate.y as usize - 1) * self.width as usize + (coordinate.x) as usize - 1
    }

    fn cell_at(&self, coordinate: &Coordinate) -> Result<&'a BoardCell, &str> {
        let index: usize = self.coordinate_to_index(coordinate);

        self.cells.get(index).ok_or("invalid coordinate")
    }
}

#[derive(Debug)]
struct Ship {
    ship_type: ShipType,
    health: u8
}

impl Ship {
    fn new(ship_type: ShipType) -> Ship {
        let size = ship_type.size();
        Ship{
            ship_type,
            health: size
        }
    }

    fn size(&self) -> u8 {
        self.ship_type.size()
    }
}

#[derive(Debug)]
enum ShipType {
    Destroyer,
    Submarine,
    Cruiser,
    Battleship,
    AircraftCarrier
}

impl ShipType {
    fn size(&self) -> u8 {
        match self {
            ShipType::Destroyer => 2,
            ShipType::Submarine => 3,
            ShipType::Cruiser => 4,
            ShipType::Battleship => 5,
            ShipType::AircraftCarrier => 6
        }
    }

    fn name(&self) -> &str {
        match self {
            ShipType::Destroyer => "Destroyer",
            ShipType::Submarine => "Submarine",
            ShipType::Cruiser => "Cruiser",
            ShipType::Battleship => "Battleship",
            ShipType::AircraftCarrier => "Aircraft Carrier"
        }
    }
}

