use std::fmt::Display;

#[derive(Default)]
struct Machine {
    size: usize,
    data: Box<[bool]>,
    head: usize,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self
            .data
            .iter()
            .take(self.size)
            .fold(format!("head: {}, ", self.head), |acc, b| {
                format!("{}{} -> ", acc, u8::from(*b))
            });
        s.push_str(&format!("{}", u8::from(self.data[self.head])));
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
    insts: Box<[Instruction]>,
    inst_count: usize,
    cur: usize,
}
fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let mut program = Program::default();
    if argv.len() == 1 {
        program.machine.size = 5;
        program.machine.data = vec![false; program.machine.size].into_boxed_slice();
        program.machine.machine_randomize();
    } else {
        program.machine.size = argv[1].len();
        program.machine.data = vec![false; program.machine.size].into_boxed_slice();
        (0..program.machine.size)
            .for_each(|i| program.machine.data[i] = argv[1].as_bytes()[i] == b'1');
    }
    program.insts = Box::new([Instruction::new(
        false,
        State::new(true, Direction::Right, 2),
        State::new(false, Direction::Right, 0),
    )]);
    program.inst_count = program.insts.len();

    println!("{}", program.machine);
    while program.cur < program.inst_count {
        program.cur = program
            .machine
            .machine_execute(&program.insts[program.cur], program.inst_count);
        println!("{}", program.machine);
    }
}
