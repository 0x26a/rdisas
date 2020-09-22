/*
	Rdisas (c), by 0x26a
*/
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]

mod utils;
mod tables;
mod prefixes;
pub mod intel;
use intel::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Disassembler{
	asm: &'static [u8]
}
impl Disassembler{
	pub fn new(input: &'static [u8]) -> Disassembler{
		Disassembler{asm: input}
	}
	pub fn disassemble(&self) -> Disassembly{
		let mut x: usize = 0;
		let mut instruction: InstructionResult;
		let mut output: Vec<InstructionResult> = Vec::new();
		let mut extended: [u8;MAX_INSTRUCTION_SIZE] = [0;MAX_INSTRUCTION_SIZE];
        
		while x < self.asm.len(){
			for i in 0..extended.len(){
				extended[i] = match x + i < self.asm.len(){
					true => self.asm[x + i],
					_ => 0
				};
			}
			if extended != [0;MAX_INSTRUCTION_SIZE]{
				instruction = Instruction::process(extended, &mut x);
				output.push(instruction.clone());
				match instruction{
					Ok(_) => continue,
					_ => ()
				}
			}
			break;
		}
		Disassembly::new(output)
	}
}

#[repr(C)]
#[derive(Clone)]
pub struct Disassembly{
	disasm: Vec<InstructionResult>
}
impl Disassembly{
	pub(crate) fn new(output: Vec<InstructionResult>) -> Disassembly{
		Disassembly{disasm: output}
	}
	pub fn extract(&self) -> Vec<InstructionResult>{
		self.disasm.clone()
	}
	pub fn success(&self) -> bool{
		return match self.disasm.len(){
			0 => false,
			len @ _ => match self.disasm[len - 1]{
				Ok(_) => true,
				_ => false
			}
		};
	}
}