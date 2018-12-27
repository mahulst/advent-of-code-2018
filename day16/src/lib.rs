use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Register {
    pub reg0: i32,
    pub reg1: i32,
    pub reg2: i32,
    pub reg3: i32,

}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct OpCode {
    pub id: i32,
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

pub fn addr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = a + b;


    Register::insert(reg, id, value)
}

pub fn addi(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = a + b;


    Register::insert(reg, id, value)
}

pub fn mulr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;

    let value = a * b;


    Register::insert(reg, id, value)
}

pub fn muli(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = a * b;


    Register::insert(reg, id, value)
}

pub fn banr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = a & b;


    Register::insert(reg, id, value)
}

pub fn bani(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = a & b;


    Register::insert(reg, id, value)
}

pub fn borr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = a | b;


    Register::insert(reg, id, value)
}

pub fn bori(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = a | b;


    Register::insert(reg, id, value)
}

pub fn setr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let id = opcode.c;
    let value = a;


    Register::insert(reg, id, value)
}

pub fn seti(reg: &Register, opcode: &OpCode) -> Register {
    let a = opcode.a;
    let id = opcode.c;
    let value = a;


    Register::insert(reg, id, value)
}

pub fn gtir(reg: &Register, opcode: &OpCode) -> Register {
    let a = opcode.a;
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = if a > b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn gtri(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = if a > b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn gtrr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = if a > b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn eqir(reg: &Register, opcode: &OpCode) -> Register {
    let a = opcode.a;
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = if a == b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn eqri(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = opcode.b;
    let id = opcode.c;
    let value = if a == b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn eqrr(reg: &Register, opcode: &OpCode) -> Register {
    let a = Register::get(reg, opcode.a);
    let b = Register::get(reg, opcode.b);
    let id = opcode.c;
    let value = if a == b { 1 } else { 0 };


    Register::insert(reg, id, value)
}

pub fn deduce(instructions: &Vec<(Register, OpCode, Register)>) {
    let mut map = HashMap::new();
    instructions.iter().for_each(|i| {
        let set = get_opcode_ids(&i.0, &i.1, &i.2);

        map.insert(i.1.id, set);
    });

    while !map.is_empty() {
        let mut function_id: Vec<i32> = vec![];
        let mut opcode_id = -1;
        {
            let entry = map.iter().find(|(id, set)| {
                set.len() == 1
            }).unwrap();

            function_id = entry.1.iter().map(|i| *i).collect();
            opcode_id = *entry.0;
        }

        map.iter_mut().for_each(|(id, set)| {
            function_id.iter().for_each(|fn_index| {
                set.remove(fn_index);
            });
        });

        map.remove(&opcode_id);
        println!("opcode {} belongs to function index {:?}", opcode_id, function_id);
    }
}

pub fn opcode_id_to_fn(id: i32) -> fn(&Register, &OpCode) -> Register {
    let fns = [
        addr,
        addi,
        mulr,
        muli,
        banr,
        bani,
        borr,
        bori,
        setr,
        seti,
        gtir,
        gtri,
        gtrr,
        eqir,
        eqri,
        eqrr];

    let mut map: HashMap<i32, usize> = HashMap::new();
    map.insert(9, 12);
    map.insert(3, 13);
    map.insert(11, 11);
    map.insert(12, 14);
    map.insert(1, 15);
    map.insert(8, 10);
    map.insert(2, 8);
    map.insert(0, 4);
    map.insert(6, 5);
    map.insert(15, 9);
    map.insert(14, 2);
    map.insert(5, 3);
    map.insert(10, 1);
    map.insert(13, 0);
    map.insert(7, 6);
    map.insert(4, 7);

    let function_id = map.get(&id).expect("Get function");

    fns[*function_id]
}

pub fn get_opcode_ids(before: &Register, opcode: &OpCode, after: &Register) -> HashSet<i32> {
    let fns = [
        addr,
        addi,
        mulr,
        muli,
        banr,
        bani,
        borr,
        bori,
        setr,
        seti,
        gtir,
        gtri,
        gtrr,
        eqir,
        eqri,
        eqrr];

    let mut i = 0;
    let mut set = HashSet::new();
    for fun in fns.iter() {
        if *after == fun(before, opcode) {
            set.insert(i);
        }
        i += 1;
    }
    set
}

pub fn count_possible_opcodes(before: &Register, opcode: &OpCode, after: &Register) -> u32 {
    let fns = [
        addr,
        addi,
        mulr,
        muli,
        banr,
        bani,
        borr,
        bori,
        setr,
        seti,
        gtir,
        gtri,
        gtrr,
        eqir,
        eqri,
        eqrr];

    let mut result = 0;

    for fun in fns.iter() {
        if *after == fun(before, opcode) {
            result += 1;
        }
    }

    result
}

pub fn parse_all_instructions(input: &str) -> Vec<(Register, OpCode, Register)> {
    let lines: Vec<&str> = input.lines().collect();

    let instructions = lines.chunks_exact(4).map(|l| {
        let before: crate::Register = l.get(0).unwrap().parse().unwrap();
        let opcode: crate::OpCode = l.get(1).unwrap().parse().unwrap();
        let after: crate::Register = l.get(2).unwrap().parse().unwrap();

        (before, opcode, after)
    }).collect();

    instructions
}

impl Register {
    pub fn new(reg0: i32, reg1: i32, reg2: i32, reg3: i32) -> Register {
        Register { reg1, reg2, reg3, reg0 }
    }

    fn get(&self, id: i32) -> i32 {
        match id {
            0 => self.reg0,
            1 => self.reg1,
            2 => self.reg2,
            3 => self.reg3,
            _ => { panic!("Invalid register"); }
        }
    }

    fn insert(&self, id: i32, value: i32) -> Register {
        match id {
            0 => Register::new(value, self.reg1, self.reg2, self.reg3),
            1 => Register::new(self.reg0, value, self.reg2, self.reg3),
            2 => Register::new(self.reg0, self.reg1, value, self.reg3),
            3 => Register::new(self.reg0, self.reg1, self.reg2, value),
            _ => { panic!("Invalid register"); }
        }
    }
}

impl FromStr for Register {
    type Err = ();
    fn from_str(input: &str) -> Result<Register, ()> {
        let re =
            Regex::new(r"\[(?P<reg1>\d+), (?P<reg2>\d+), (?P<reg3>\d+), (?P<reg0>\d+)\]$")
                .expect("unwrapping register");

        match re.captures(input) {
            Some(caps) => Ok(Register::new(
                caps["reg1"].parse().expect("reg1"),
                caps["reg2"].parse().expect("reg2"),
                caps["reg3"].parse().expect("reg3"),
                caps["reg0"].parse().expect("reg0"),
            )),
            None => Err(())
        }
    }
}

impl OpCode {
    fn new(id: i32, a: i32, b: i32, c: i32) -> OpCode {
        OpCode { id, a, b, c }
    }
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(input: &str) -> Result<OpCode, ()> {
        let re =
            Regex::new(r"(?P<id>\d+) (?P<a>\d+) (?P<b>\d+) (?P<c>\d+)$")
                .expect("unwrapping opcode");

        match re.captures(input) {
            Some(caps) => Ok(OpCode::new(
                caps["id"].parse().expect("unwrapping opcode id"),
                caps["a"].parse().expect("unwrapping opcode a"),
                caps["b"].parse().expect("unwrapping opcode b"),
                caps["c"].parse().expect("unwrapping opcode c"),
            )),
            None => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Register;

    #[test]
    fn it_should_parse_input() {
        // Arrange
        let input = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

        // Act

        let mut lines: Vec<&str> = input.lines().collect();
        lines.reverse();
        let before: crate::Register = lines.pop().unwrap().parse().unwrap();
        let opcode: crate::OpCode = lines.pop().unwrap().parse().unwrap();
        let after: crate::Register = lines.pop().unwrap().parse().unwrap();

        // Assert
        assert_eq!(before, crate::Register::new(3, 2, 1, 1));
        assert_eq!(opcode, crate::OpCode::new(9, 2, 1, 2));
        assert_eq!(after, crate::Register::new(3, 2, 2, 1));
    }

    #[test]
    fn it_should_addr() {
        // Arrange
        let register = crate::Register::new(1, 2, 0, 3);
        let opcode = crate::OpCode::new(0, 0, 1, 3);

        // Act
        let result = crate::addr(&register, &opcode);

        // Assert
        assert_eq!(result, Register::new(1, 2, 0, 3));
    }

    #[test]
    fn it_should_addi() {
        // Arrange
        let register = crate::Register::new(1, 0, 0, 0);
        let opcode = crate::OpCode::new(0, 0, 7, 3);

        // Act
        let result = crate::addi(&register, &opcode);

        // Assert
        assert_eq!(result, Register::new(1, 0, 0, 8));
    }

    #[test]
    fn it_should_mulr() {
        // Arrange
        let register = crate::Register::new(1, 2, 3, 3);
        let opcode = crate::OpCode::new(0, 1, 2, 3);

        // Act
        let result = crate::mulr(&register, &opcode);

        // Assert
        assert_eq!(result, Register::new(1, 2, 3, 6));
    }

    #[test]
    fn it_should_muli() {
        // Arrange
        let register = crate::Register::new(1, 2, 3, 3);
        let opcode = crate::OpCode::new(0, 1, 9, 3);

        // Act
        let result = crate::muli(&register, &opcode);

        // Assert
        assert_eq!(result, Register::new(1, 2, 3, 18));
    }

    #[test]
    fn it_should_complete_sample() {
        // Arrange
        let before = crate::Register::new(3, 2, 1, 1);
        let opcode = crate::OpCode::new(9, 2, 1, 2);
        let after = crate::Register::new(3, 2, 2, 1);

        // Act
        let result1 = crate::mulr(&before, &opcode);
        let result2 = crate::addi(&before, &opcode);
        let result3 = crate::seti(&before, &opcode);

        // Assert
        assert_eq!(result1, after);
        assert_eq!(result2, after);
        assert_eq!(result3, after);
    }

    #[test]
    fn it_should_count_all_possible_opcodes() {
        // Arrange
        let before = crate::Register::new(3, 2, 1, 1);
        let opcode = crate::OpCode::new(9, 2, 1, 2);
        let after = crate::Register::new(3, 2, 2, 1);

        // Act
        let result = crate::count_possible_opcodes(&before, &opcode, &after);

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn it_should_parse_all_instructions() {
        // Arrange
        let input = r"Before: [1, 0, 2, 1]
2 3 2 0
After:  [1, 0, 2, 1]

Before: [1, 0, 2, 2]
11 3 2 1
After:  [1, 0, 2, 2]

Before: [0, 2, 3, 0]
7 0 3 3
After:  [0, 2, 3, 0]

";

        // Act
        let result = crate::parse_all_instructions(input);

        // Assert
        assert_eq!(result, vec![
            (
                crate::Register::new(1, 0, 2, 1),
                crate::OpCode::new(2, 3, 2, 0),
                crate::Register::new(1, 0, 2, 1),
            ),
            (
                crate::Register::new(1, 0, 2, 2),
                crate::OpCode::new(11, 3, 2, 1),
                crate::Register::new(1, 0, 2, 2),
            ),
            (
                crate::Register::new(0, 2, 3, 0),
                crate::OpCode::new(7, 0, 3, 3),
                crate::Register::new(0, 2, 3, 0),
            )
        ]);
    }
}
