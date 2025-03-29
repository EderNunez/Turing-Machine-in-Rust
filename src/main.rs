use std::fmt::Display;

#[derive(Default)]
struct Tape {
    data: Vec<bool>,
    count: usize,
    capacity: usize,
}

impl Tape {
    fn tape_randomize(&mut self) {
        self.data.iter_mut().for_each(|b| *b = rand::random());
    }
}

#[derive(Default)]
struct Machine {
    tape: Tape,
    head: usize,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self
            .tape
            .data
            .iter()
            .take(self.tape.count)
            .fold(format!("head: {}, ", self.head), |acc, b| {
                format!("{}{} -> ", acc, u8::from(*b))
            });
        s.remove(s.len() - 2);
        s.remove(s.len() - 2);
        write!(f, "{s}")
    }
}

impl Machine {
    fn machine_execute(&mut self, inst: &Instruction, inst_count: usize) -> usize {
        if self.head >= self.tape.capacity {
            self.tape.data.resize(self.tape.capacity * 2, false);
            self.tape.capacity *= 2;
        }
        if self.tape.data[self.head] == inst.expected {
            self.tape.data[self.head] = inst.yes.write;
            if self.head == 0 && (inst.yes.dir as i8) < 0 {
                return inst_count;
            }
            self.head = (self.head as isize + inst.yes.dir as isize) as usize;
            if self.head > self.tape.count {
                self.tape.count = self.head;
            }
            return inst.yes.next;
        }
        self.tape.data[self.head] = inst.no.write;
        if self.head == 0 && (inst.no.dir as i8) < 0 {
            return inst_count;
        }
        self.head = (self.head as isize + inst.no.dir as isize) as usize;
        if self.head > self.tape.count {
            self.tape.count = self.head;
        }
        inst.no.next
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left = -1,
    Stay = 0,
    Right = 1,
}
#[derive(Clone)]
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
    let argv = std::env::args().collect::<Vec<_>>().into_boxed_slice();
    let mut program = Program::default();
    if argv.len() == 1 {
        program.machine.tape.capacity = 3;
        program.machine.tape.count = program.machine.tape.capacity;
        program.machine.tape.data = vec![false; program.machine.tape.capacity];
        program.machine.tape.tape_randomize();
    } else {
        program.machine.tape.capacity = argv[1].len();
        program.machine.tape.count = program.machine.tape.capacity;
        program.machine.tape.data = vec![false; program.machine.tape.capacity];
        (0..program.machine.tape.capacity)
            .for_each(|i| program.machine.tape.data[i] = argv[1].as_bytes()[i] == b'1');
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
