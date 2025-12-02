use std::fs;

fn main() {
    let file_text =
        fs::read_to_string("input.txt").unwrap_or_else(|e| panic!("could not open input.txt: {e}"));

    let counter_state = file_text.lines().map(Rotation::parse).fold(
        CounterState {
            current_pos: 50,
            zero_count: 0,
        },
        |state, rotation| state.with_rotation(rotation),
    );

    println!("{counter_state:?}");
}

#[derive(Debug)]
struct CounterState {
    current_pos: u32,
    zero_count: u32,
}

impl CounterState {
    fn with_rotation(self, r: Rotation) -> Self {
        let current_pos = r.apply(self.current_pos);
        Self {
            zero_count: if current_pos == 0 {
                self.zero_count + 1
            } else {
                self.zero_count
            },
            current_pos,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn from_str(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("bad direction letter"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Dir,
    count: u32,
}

impl Rotation {
    fn parse(input: &str) -> Self {
        let dir_value = &input[..1];
        let direction = Dir::from_str(dir_value);

        Rotation {
            direction,
            count: u32::from_str_radix(&input[1..], 10).unwrap(),
        }
    }

    fn apply(self, current_pos: u32) -> u32 {
        (current_pos
            + match self.direction {
                Dir::Left => 100 - (self.count % 100),
                Dir::Right => self.count,
            })
            % 100
    }
}
