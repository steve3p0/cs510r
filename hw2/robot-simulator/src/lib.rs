// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot
{
    x_axis: isize,
    y_axis: isize,
    facing: Direction
}

impl Robot
{
    pub fn new(x: isize, y: isize, d: Direction) -> Self
    {
        Robot { x_axis: x, y_axis: y, facing: d }
    }

    pub fn turn_right(mut self) -> Self
    {
        self.facing = match self
        {
            Robot { facing: Direction::North, .. } => Direction::East,
            Robot { facing: Direction::East,  .. } => Direction::South,
            Robot { facing: Direction::South, .. } => Direction::West,
            Robot { facing: Direction::West, .. } => Direction::North
        };

        self
    }

    pub fn turn_left(mut self) -> Self
    {
        self.facing = match self
        {
            Robot { facing: Direction::North, .. } => Direction::West,
            Robot { facing: Direction::West,  .. } => Direction::South,
            Robot { facing: Direction::South, .. } => Direction::East,
            Robot { facing: Direction::East, .. } => Direction::North
        };

        self
    }

    pub fn advance(mut self) -> Self
    {
        self.y_axis = match self
        {
            Robot { facing: Direction::North, .. } => self.y_axis + 1,
            Robot { facing: Direction::South, .. } => self.y_axis - 1,
            _ => self.y_axis
        };

        self.x_axis = match self
        {
            Robot { facing: Direction::East, .. } => self.x_axis + 1,
            Robot { facing: Direction::West, .. } => self.x_axis - 1,
            _ => self.x_axis
        };

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self
    {
        for c in instructions.chars()
        {
            self = match c
            {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("Error invalid instruction")
            };
        }

        self
    }

    pub fn position(&self) -> (isize, isize)
    {
        (self.x_axis, self.y_axis)
    }

    pub fn direction(&self) -> &Direction
    {
        &self.facing
    }
}
