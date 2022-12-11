use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
struct Pixel(char);

#[derive(Debug, Clone, Copy)]
struct Row {
    pixels: [Pixel; 40],
}

impl Row {
    fn new() -> Self {
        Row {
            pixels: [Pixel(' '); 40],
        }
    }

    fn make_visible(&mut self, x: usize) {
        self.pixels[x] = Pixel('#');
    }
}

#[derive(Debug)]
struct Screen {
    rows: [Row; 6],
}

impl Screen {
    fn new() -> Self {
        Screen {
            rows: [Row::new(); 6],
        }
    }

    fn draw(&self) {
        for row in &self.rows {
            for pixel in &row.pixels {
                print!("{}", pixel.0);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    cycles: usize,
    value: i32,
    kind: InstructionKinds,
}

#[derive(Debug, Clone, Copy)]
enum InstructionKinds {
    Noop,
    Addx,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => Ok(Instruction {
                cycles: 1,
                value: 0,
                kind: InstructionKinds::Noop,
            }),
            "addx" => {
                let value = parts.next().unwrap().parse().unwrap();
                Ok(Instruction {
                    cycles: 2,
                    value,
                    kind: InstructionKinds::Addx,
                })
            }
            _ => panic!("Invalid instruction"),
        }
    }
}

impl Instruction {
    fn noop() -> Self {
        Self {
            cycles: 1,
            value: 0,
            kind: InstructionKinds::Noop,
        }
    }

    fn decrement_cycles(&mut self) {
        self.cycles -= 1;
    }
}

#[derive(Debug)]
struct Machine {
    x: i32,
    cycle: u32,
    instruction: Instruction,
    signal_strengths: Vec<i32>,
    screen: Screen,
}

impl Machine {
    fn new() -> Self {
        Machine {
            x: 1,
            cycle: 0,
            instruction: Instruction::noop(),
            signal_strengths: Vec::new(),
            screen: Screen::new(),
        }
    }

    fn update_signal_strength(&mut self) {
        let current_strength = self.x * self.cycle as i32;
        self.signal_strengths.push(current_strength);
    }

    fn process(&mut self, instruction: Instruction) {
        self.load_instruction(instruction);
        while self.instruction.cycles > 0 {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;

        if self.should_calc_signal_strength() {
            self.update_signal_strength();
        }

        if self.is_sprite_visible() {
            self.draw_sprite();
        }

        match self.instruction.kind {
            InstructionKinds::Noop => self.handle_noop(),
            InstructionKinds::Addx => self.handle_addx(),
        }
    }

    fn should_calc_signal_strength(&self) -> bool {
        if self.cycle < 20 {
            return false;
        }
        (self.cycle - 20) % 40 == 0
    }

    fn handle_noop(&mut self) {
        self.instruction.decrement_cycles();
    }

    fn handle_addx(&mut self) {
        self.instruction.decrement_cycles();
        if self.instruction.cycles == 0 {
            self.x += self.instruction.value;
        }
    }

    fn load_instruction(&mut self, instruction: Instruction) {
        self.instruction = instruction;
    }

    fn current_y_pos(&self) -> u32 {
        self.cycle / 40
    }
    fn current_x_pos(&self) -> u32 {
        (self.cycle - 1) % 40
    }

    fn is_sprite_visible(&self) -> bool {
        let current_x = self.current_x_pos() as i32;
        (self.x - 1..=self.x + 1).contains(&current_x)
    }

    fn draw_sprite(&mut self) {
        let current_y = self.current_y_pos() as usize;
        let current_x = self.current_x_pos() as usize;
        self.screen.rows[current_y].make_visible(current_x);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut machine = Machine::new();
    let instructions: Vec<_> = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    for instruction in &instructions {
        machine.process(*instruction);
    }

    let value: i32 = machine.signal_strengths.iter().sum();

    Some(value as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut machine = Machine::new();
    let instructions: Vec<_> = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    for instruction in &instructions {
        machine.process(*instruction);
    }

    machine.screen.draw();

    Some(1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
