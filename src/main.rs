use std::fmt::Display;

#[derive(Default)]
struct Machine {
    size: usize,
    data: Vec<bool>,
    head: usize,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .fold(format!("head: {}, ", self.head), |acc, b| {
                format!("{}{} -> ", acc, u8::from(*b))
            });
        write!(f, "{s}")
    }
}

impl Machine {
    fn machine_randomize(&mut self) {
        self.data.iter_mut().for_each(|b| *b = rand::random());
    }

    fn machine_execute(&mut self, inst: &Instruction, inst_count: usize) -> usize {
        if self.head >= self.size {
            return inst_count;
        }
        if self.data[self.head] == inst.expected {
            self.data[self.head] = inst.yes.write;
            if self.head == 0 && (inst.yes.dir as i8) < 0 {
                return inst_count;
            }
            self.head = (self.head as isize + inst.yes.dir as isize) as usize;
            return inst.yes.next;
        }
        self.data[self.head] = inst.no.write;
        if self.head == 0 && (inst.no.dir as i8) < 0 {
            return inst_count;
        }
        self.head = (self.head as isize + inst.no.dir as isize) as usize;
        inst.no.next
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left = -1,
    Stay = 0,
    Right = 1,
}
#[derive(Clone, Copy)]
struct State {
    write: bool,
    dir: Direction,
    next: usize,
}

impl State {
    fn new(write: bool, dir: Direction, next: usize) -> Self {
        Self { write, dir, next }
    }
}
#[derive(Clone, Copy)]
struct Instruction {
    expected: bool,
    yes: State,
    no: State,
}

impl Instruction {
    fn new(expected: bool, yes: State, no: State) -> Self {
        Self { expected, yes, no }
    }
}
#[derive(Default)]
struct Program {
    machine: Machine,
    insts: Vec<Instruction>,
    inst_count: usize,
    cur: usize,
}
fn main() {
    let mut program = Program::default();
    program.machine.size = 16;
    program.machine.data = vec![false; program.machine.size];
    program.machine.machine_randomize();

    program.insts = vec![Instruction::new(
        false,
        State::new(true, Direction::Right, 2),
        State::new(false, Direction::Right, 0),
    )];
    program.inst_count = program.insts.len();
    
    println!("{}", program.machine);
    while program.cur < program.inst_count {
        program.cur = program
            .machine
            .machine_execute(&program.insts[program.cur], program.inst_count);
        println!("{}", program.machine);
    }
}
