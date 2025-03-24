use std::fmt::Display;

#[derive(Default)]
struct Machine {
    size: usize,
    data: Vec<bool>,
    head: usize,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("head: {}, ", self.head);
        for i in 0..self.size {
            s.push_str(&format!("{} -> ", i32::from(self.data[i])));
        }
        write!(f, "{s}")
    }
}

impl Machine {
    fn new(data: Vec<bool>, size: usize, head: usize) -> Self {
        Self { data, size, head }
    }

    fn machine_randomize(&mut self) {
        for i in 0..self.size {
            self.data[i] = rand::random();
        }
    }

    fn machine_execute(&mut self, inst: &Instruction) -> usize {
        if self.data[self.head] == inst.expected {
            self.data[self.head] = inst.write_yes;
            self.head = (self.head as isize + inst.dir_yes as isize) as usize;
            return inst.next_yes;
        }
        self.data[self.head] = inst.write_no;
        self.head = (self.head as isize + inst.dir_no as isize) as usize;
        inst.next_no
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left = -1,
    Stay = 0,
    Right = 1,
}

struct Instruction {
    expected: bool,

    write_yes: bool,
    dir_yes: Direction,
    next_yes: usize,

    write_no: bool,
    dir_no: Direction,
    next_no: usize,
}

impl Instruction {
    fn new(
        expected: bool,
        write_yes: bool,
        dir_yes: Direction,
        next_yes: usize,
        write_no: bool,
        dir_no: Direction,
        next_no: usize,
    ) -> Self {
        Self {
            expected,
            write_yes,
            dir_yes,
            next_yes,
            write_no,
            dir_no,
            next_no,
        }
    }
}
fn main() {
    let mut machine = Machine::default();
    machine.size = 8;
    machine.data = vec![false; machine.size];
    machine.machine_randomize();
    let insts = [Instruction::new(
        false,
        true,
        Direction::Right,
        2,
        false,
        Direction::Right,
        0,
    )];
    println!("{machine}");
    let cur = insts.iter().fold(0, |mut acc, inst|{
        acc = machine.machine_execute(inst);
        println!("{machine}");
        acc
    });
    println!("cur: {cur}");
}
