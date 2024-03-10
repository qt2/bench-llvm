// use bytes::Buf;
use crate::FastCursor;
use smallvec::SmallVec;

#[repr(u8)]
enum OpCode {
    EOF,
    Int,
    SetLocal,
    GetLocal,
    Jump,
    JumpIfFalse,
    Mul,
    Mod,
    LT,
    Increment,
}

#[derive(Debug, Default)]
pub struct VM {
    stack: SmallVec<[i64; 8]>,
    locals: [i64; 8],
}

impl VM {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn eval(&mut self, bytecode: &[u8]) {
        // let mut bytecode = Cursor::new(bytecode);
        let mut bytecode = FastCursor::new(bytecode);

        loop {
            match bytecode.get_u8() {
                0 => {
                    break;
                }
                1 => {
                    let v = bytecode.get_i64_le();
                    self.stack.push(v);
                }
                2 => {
                    let addr = bytecode.get_u8();
                    let v = self.stack.pop().unwrap();
                    self.locals[addr as usize] = v;
                }
                3 => {
                    let addr = bytecode.get_u8();
                    let v = self.locals[addr as usize];
                    self.stack.push(v);
                }
                4 => {
                    let pos = bytecode.get_u8();
                    bytecode.set_position(pos as usize);
                }
                5 => {
                    let pos = bytecode.get_u8();
                    if self.stack.pop().unwrap() == 0 {
                        bytecode.set_position(pos as usize);
                    }
                }
                6 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                7 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                }
                8 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                9 => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + 1);
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let mut vm = VM::new();
        let bytecode = vec![
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            0, // EOF
        ];
        vm.eval(&bytecode);
        println!("{:?}", vm.stack)
    }

    #[test]
    fn test_iter() {
        let mut vm = VM::new();
        let bytecode = vec![
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            0, // 0
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            1, // 1
            3, // Get @22
            1, // 1
            1, // Int
            5, 0, 0, 0, 0, 0, 0, 0,  // 5
            8,  // LT
            5,  // JumpIfFalse @34
            50, // to 50
            3,  // Get
            0,  // 0
            3,  // Get
            1,  // 1
            6,  // Mul
            2,  // Set
            0,  // 0
            3,  // Get
            1,  // 1
            9,  // Increment
            2,  // Set
            1,  // 1
            4,  // Jump
            22, // to 22
            0,  // EOF @50
        ];
        vm.eval(&bytecode);
        println!("{:?}", vm);

        let mut vm = VM::new();
        let bytecode = vec![
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            0, // 0
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            1, // 1
            3, // Get @22
            1, // 1
            1, // Int
            0x40, 0x42, 0x0f, 0, 0, 0, 0, 0,  // 1000000
            8,  // LT
            5,  // JumpIfFalse @34
            60, // to 60
            3,  // Get
            0,  // 0
            3,  // Get
            1,  // 1
            6,  // Mul
            1,  // Int
            0x07, 0xe1, 0xf5, 0x05, 0, 0, 0, 0,  // 10000007
            7,  // Mod
            2,  // Set
            0,  // 0
            3,  // Get
            1,  // 1
            9,  // Increment
            2,  // Set
            1,  // 1
            4,  // Jump
            22, // to 22
            0,  // EOF @60
        ];
        vm.eval(&bytecode);
        println!("{:?}", vm);
    }
}
