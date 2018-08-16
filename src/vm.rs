use enum_primitive::FromPrimitive;
use ops::OP;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;
use std::vec::Vec;
use util::u8s_to_u16;

const NUM_REGISTERS: usize = 8;
const DATA_SIZE: usize = 32768;

pub struct VM {
    pc: usize,
    registers: [u16; NUM_REGISTERS],
    data: [u16; DATA_SIZE],
    stack: Vec<u16>,
    debug: bool,
    symbol_is_op: bool,
    input_buffer: VecDeque<u8>,
}

type OpFunc = fn(&mut VM);

type ALUFunc = fn(u64, u64) -> u64;

fn translate_op(op: OP) -> OpFunc {
    match op {
        OP::Halt => VM::halt,
        OP::Jmp => VM::jmp,
        OP::Jt => VM::jt,
        OP::Jf => VM::jf,
        OP::NoOp => VM::noop,
        OP::Out => VM::out,
        OP::Set => VM::set,
        OP::Eq => VM::eq,
        OP::Gt => VM::gt,
        OP::Add => VM::add,
        OP::Mul => VM::mul,
        OP::Modulo => VM::modulo,
        OP::Or => VM::or,
        OP::And => VM::and,
        OP::Push => VM::push,
        OP::Pop => VM::pop,
        OP::Not => VM::not,
        OP::Call => VM::call,
        OP::Ret => VM::ret,
        OP::Rmem => VM::rmem,
        OP::Wmem => VM::wmem,
        OP::Input => VM::input,
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            pc: 0,
            registers: [0; NUM_REGISTERS],
            data: [0; DATA_SIZE],
            stack: Vec::new(),
            debug: false,
            symbol_is_op: true,
            input_buffer: VecDeque::new(),
        }
    }

    pub fn from_file(filename: &str) -> VM {
        let mut file = File::open(filename).expect("file not found");

        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)
            .expect("something went wrong reading the file");

        VM::from_data(&contents)
    }

    fn from_data(data: &[u8]) -> VM {
        let mut vm = VM::new();

        let mut idx: usize = 0;
        while idx < data.len() {
            let value = u8s_to_u16(data[idx], data[idx + 1]);
            vm.data[idx / 2] = value;
            idx += 2;
        }
        vm
    }

    fn next_byte(&mut self) -> u16 {
        let val = self.data[self.pc];

        if self.debug {
            if self.symbol_is_op {
                print!("\n{}: {:?} ", self.pc, OP::from_u16(val).unwrap());
                self.symbol_is_op = false;
            } else {
                print!("{} ", val);
            }
        }
        self.pc += 1;

        val
    }

    fn load_val(&self, addr: usize) -> u16 {
        match addr {
            0...32767 => addr as u16,
            32768...32775 => self.registers[addr - DATA_SIZE],
            _ => panic!("Invalid memory location: {}", addr),
        }
    }

    fn store_register(&mut self, addr: usize, mut val: u16) {
        if (DATA_SIZE..DATA_SIZE + NUM_REGISTERS).contains(&(val as usize)) {
            val = self.load_val(val as usize);
        }

        if DATA_SIZE <= addr && addr <= DATA_SIZE + NUM_REGISTERS {
            self.registers[addr - DATA_SIZE] = val
        } else if addr <= 7 {
            self.registers[addr] = val
        } else {
            panic!("Invalid register: {}", addr)
        }
    }

    fn read_mem(&self, addr: usize) -> u16 {
        self.data[addr]
    }

    fn write_mem(&mut self, addr: usize, val: u16) {
        self.data[addr] = val
    }

    fn stack_push(&mut self, val: u16) {
        self.stack.push(val)
    }

    fn stack_pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    fn next_op(&mut self) -> OP {
        self.symbol_is_op = true;
        let op_byte = self.next_byte();
        match OP::from_u16(op_byte) {
            Some(op) => op,
            None => panic!("Invalid op code: {}", op_byte),
        }
    }

    fn next_val(&mut self) -> u16 {
        let a = self.next_byte() as usize;
        self.load_val(a)
    }

    fn alu_op(&mut self, func: ALUFunc) {
        let a = self.next_byte();
        let b = self.next_val();
        let c = self.next_val();
        let result = func(u64::from(b), u64::from(c)) % DATA_SIZE as u64;
        self.store_register(a as usize, result as u16);
    }

    fn noop(&mut self) {}

    fn halt(&mut self) {
        println!("Halting!");
        process::exit(0);
    }

    fn out(&mut self) {
        print!("{}", (self.next_val() as u8) as char)
    }

    fn jmp(&mut self) {
        self.pc = self.next_val() as usize
    }

    fn jt(&mut self) {
        let a = self.next_val();
        let b = self.next_val();
        if a != 0 {
            self.pc = b as usize
        }
    }

    fn jf(&mut self) {
        let a = self.next_val();
        let b = self.next_val();
        if a == 0 {
            self.pc = b as usize
        }
    }

    fn set(&mut self) {
        let a = self.next_byte();
        let b = self.next_val();
        self.store_register(a as usize, b);
    }

    fn add(&mut self) {
        self.alu_op(|a, b| a + b)
    }

    fn mul(&mut self) {
        self.alu_op(|a, b| a * b)
    }

    fn modulo(&mut self) {
        self.alu_op(|a, b| a % b)
    }

    fn or(&mut self) {
        self.alu_op(|a, b| a | b)
    }

    fn eq(&mut self) {
        self.alu_op(|a, b| if a == b { 1 } else { 0 })
    }

    fn gt(&mut self) {
        self.alu_op(|a, b| if a > b { 1 } else { 0 })
    }

    fn and(&mut self) {
        self.alu_op(|a, b| a & b)
    }

    fn push(&mut self) {
        let a = self.next_val();
        self.stack_push(a);
    }

    fn pop(&mut self) {
        let a = self.next_byte();
        let b = self.stack_pop();
        self.store_register(a as usize, b);
    }

    fn not(&mut self) {
        let a = self.next_byte();
        let b = self.next_val();
        self.store_register(a as usize, !b & 0x7fff);
    }

    fn call(&mut self) {
        let a = self.next_val();
        let old_pc = self.pc as u16;
        self.stack_push(old_pc);
        self.pc = a as usize;
    }

    fn ret(&mut self) {
        self.pc = self.stack_pop() as usize;
    }

    fn rmem(&mut self) {
        let a = self.next_byte() as usize;
        let b = self.next_val() as usize;
        let val = self.read_mem(b);
        self.store_register(a, val);
    }

    fn wmem(&mut self) {
        let a = self.next_val();
        let b = self.next_val();
        self.write_mem(a as usize, b);
    }

    fn hack_teleporter(&mut self) {
        self.data[6027] = 1;
        self.data[6028] = 32768;
        self.data[6029] = 6;
        self.data[6030] = 18;
        self.registers[7] = 25734;
        println!("Teleporter hacked!");
    }

    fn next_input(&mut self) -> u16 {
        while self.input_buffer.is_empty() {
            print!(">>");
            io::stdout().flush().expect("Could not flush stdout");
            let mut line = String::new();

            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).unwrap();
            line = line.to_ascii_lowercase();

            if line.eq("hack_teleporter\n") {
                self.hack_teleporter();
                continue;
            }

            for c in line.into_bytes() {
                self.input_buffer.push_back(c);
            }
        }
        u16::from(self.input_buffer.pop_front().unwrap())
    }

    fn input(&mut self) {
        let a = self.next_byte();
        let input = self.next_input();
        self.store_register(a as usize, input);
    }

    fn run_op(&mut self, op: OP) {
        translate_op(op)(self);
    }

    pub fn run(&mut self) {
        loop {
            let op = self.next_op();
            self.run_op(op);
        }
    }
}
