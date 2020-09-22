mod alu; // Standard x86 Instruction Set
mod alu2; // Support two-bytes opcodes for the Standard x86 Instruction Set

mod fpu; // Floating Point Instruction Set
mod mmx; // MMX Instruction Extension
mod sse; // SSE Instruction Extension
mod vmx; // VMX Instruction Extension

use crate::utils::*;
use self::{enums::*};

pub const MAX_INSTRUCTION_SIZE: usize = 15;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum InstructionError{
	UnknownError,
	InstructionNotRecognized,
	FeatureNotSupported,
	ModRmInvalid,
	ModRmOrSibInvalid,
	TooManyPrefixes
}
pub type InstructionResult = Result<Instruction, InstructionError>;

pub(crate) mod enums{
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub enum MainType{
		ALU,
		FPU,
		MMX,
		VMX,
		SSE
	}
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub enum SubType{
		MonoB, 
		PluriB,
		ModrmB,
		ExtModrmB,
		ExtOpB,
		SpecialOp
	}
	#[derive(Clone, PartialEq, Copy, Debug)]
	pub enum Register{
		Eax,
		Ecx,
		Edx,
		Ebx,
		Esp,
		Ebp,
		Esi,
		Edi,
		Illegal,
		DisplacementOnly
	}
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub enum AddressingMode{
		RegisterIndirectAddressing,
		SIBNoDisplacement,
		DisplacementOnlyAddressing,
		OneByteAfterMod,
		FourByteAfterMod,
		RegisterAddressingMode
	}
	#[derive(Clone, Copy, Debug)]
	pub enum Index{
		Mul1,
		Mul2,
		Mul4,
		Mul8
	}
}
#[repr(C)]
#[derive(Clone)]
pub struct Instruction{
	pub(crate) mtype: Option<MainType>,
	pub(crate) r#type: Option<SubType>,
	pub(crate) prefixes: Vec<u8>,
	pub(crate) opcode: Vec<u8>,
	pub(crate) modrm: Option<ModRegRm>,
	pub(crate) sib: Option<Byte>,
	pub(crate) disp: Digit,
	pub(crate) imm: Digit,
	pub(crate) size: usize,	
	pub(crate) det: Details,
	pub(crate) _0f: bool

}
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ModRegRm{
	pub(crate) byte: Byte,
	pub(crate) index: usize
}
#[repr(C)]
#[derive(Clone)]
pub(crate) struct Digit{
	raw: Vec<u8>,
	nb: u32,
	size: usize
}
#[repr(C)]
#[derive(Clone)]
pub(crate) struct Details{
	pub(crate) istr: String,
	pub(crate) extended: [u8;15],
	pub(crate) x16: bool,
}
#[repr(C)]
#[derive(Clone)]
pub(crate) struct Settings{
	predefined: bool,
	one_operand: bool,
	immediate: bool,
	op_indx: usize,
	s: Option<u8>,
	x: Option<u8>
}
pub(crate) const DEFAULT_INSTRUCTION: Instruction = Instruction{
	mtype: None,
	r#type: None,
	prefixes: Vec::new(),
	opcode: Vec::new(),
	modrm: None,
	sib: None,
	disp: Digit{
		raw: Vec::new(),
		nb: 0,
		size: 0
	},
	imm: Digit{
		raw: Vec::new(),
		nb: 0,
		size: 0
	},
	size: 0,
	det: Details{
		istr: String::new(),
		extended: [0;15],
		x16: false,
	},
	_0f: false
};

impl Instruction{
	// public
	pub fn size(&self) -> usize{ self.size }
	pub fn immediate(&self) -> Vec<u8>{ self.imm.to_vec() }
	pub fn to_string(&self) -> String{ self.det.istr.clone() }
	pub fn prefixes(&self) -> Vec<u8>{ self.prefixes.clone() }
	pub fn displacement(&self) -> Vec<u8>{ self.disp.to_vec() }
	pub fn to_vec(&self) -> Vec<u8>{
		let mut hex: Vec<u8> = Vec::new();
		for x in 0..self.size{
			hex.push(self.det.extended[x]);
		}
		hex.clone()
	}
	pub fn mnemonic(&self) -> String{
		return match self.det.istr.contains(" "){
			false => self.det.istr.clone(),
			_ => {
				let mut  tmp = String::new();
				for (index, chr) in self.det.istr.chars().enumerate(){
					if chr == ' '{
						tmp =  self.det.istr[..index].to_string();
						break;
					}
				}
				tmp
			}
		};
	}
	// internal
	pub(crate) fn process(data: [u8;MAX_INSTRUCTION_SIZE], index: &mut usize) -> InstructionResult{
		let mut x: usize = 0;
		let mut item: InstructionResult = Ok(DEFAULT_INSTRUCTION);

		decode::prefixes(data, &mut item, &mut x);
		decode::recognize(data, &mut item, &mut x);
		
		if item.is_err(){
			return item;
		}
		item.unwrap().decode(data, index)
	}
	pub(self) fn decode(&mut self, data: [u8;MAX_INSTRUCTION_SIZE], index: &mut usize) -> InstructionResult{
		use crate::intel::enums::SubType::*;

		self.det.extended = data;
		self.det.x16 = self.prefixes.iter().any(|i| i == &0x66);

		let decoded = match self.mtype{
			Some(t @ _) => match t{
				MainType::ALU => match self.r#type{
					Some(t2 @ _) => match t2{
						SubType::MonoB => self::alu::asm_mono(self,index),
						SubType::PluriB => self::alu::asm_plri(self,index),
						t3 @ SubType::ModrmB | t3 @ SubType::ExtModrmB => {
							let immediate = match t3{
								ModrmB => false,
								_ => true
							};
							self::alu::asm_mdrm(self,index,Settings{
								predefined: false,
								one_operand: false,
								immediate: immediate,
								op_indx: 0,
								s: None,
								x: None
							})
						}
						SubType::ExtOpB => self::alu2::asm_x86_0f(self,index),
						SubType::SpecialOp => self::alu::asm_special(self,index)
					},
					_ => Err(InstructionError::FeatureNotSupported)
				},
				MainType::FPU => self::fpu::asm_x87(self,index),
				MainType::MMX => self::mmx::asm_mmx(self,index),
				MainType::SSE => self::sse::asm_sse(self,index),
				MainType::VMX => self::vmx::asm_vmx(self,index)
			},
			_ => Err(InstructionError::FeatureNotSupported) 
		};
		if decoded.is_ok(){
			let decoded = decoded.unwrap().asm_prfx();
			return decoded;
		}
		decoded
	}

	pub(self) fn asm_prfx(&mut self) -> InstructionResult{
		use crate::prefixes::*;

		let mut r: InstructionResult = Ok(self.clone());

		for item in SEGMENT_PREFIXES_STR.iter(){
			if self.prefixes.iter().any(|i| i == &item.0){
				self.det.istr = match self.det.istr.contains("@"){
					true => self.det.istr.replace("@",item.1),
					_ => format!("{} {}", item.1, self.det.istr)
				};
			}
		}
		if self.det.istr.contains("@"){
			self.det.istr = self.det.istr.replace("@","ds");
		}
		for x in self.prefixes.iter(){
			self.det.istr = match *x{
				0xf0 => format!("lock {}", self.det.istr),
				p if p == 0xf2 && self.mtype.unwrap() != MainType::SSE => format!("repnz {}", self.det.istr),
				p if p == 0xf3 && self.mtype.unwrap() != MainType::SSE => format!("repz {}", self.det.istr),
				_ => self.det.istr.clone()
			};
		}
		if r.is_ok(){
			r = Ok(self.clone());
		}
		r
	}
}
impl Digit{
	// Threat immediate(& rel) / displacement more easily 
	pub(crate) fn from(array: &[u8], base: usize, size: usize) -> Digit{
		let mut dg = Digit{raw: Vec::new(), nb: 0, size: size};
		for x in 0..size{
			dg.raw.push(array[base + x]);
		}
		let mut tmp: [u8;4] = [0;4];
		let len = match dg.raw.len(){
			_ if dg.raw.len() <= 4 => dg.raw.len(),
			_ => 4
		};
		for x in 0..len{
			tmp[x] = dg.raw[x];
		}
		dg.nb = ((((tmp[3] as u16) << 8) | tmp[2] as u16) as u32) << 16 | (((tmp[1] as u16) << 8) | tmp[0] as u16) as u32;
		dg
	}
	pub(crate) fn to_vec(&self) -> Vec<u8>{
		self.raw.clone()
	}
	pub(crate) fn to_disp(&self) -> String{
		if self.size == 1{
			return match self.nb >= 0x80{
				true => format!("-0x{:x}", (0xff as u32 - self.nb) + 1),
				_ => format!("+0x{:x}", self.nb)
			};
		}
		else{
			return match self.nb >= 0x80000000{
				true => format!("-0x{:x}", (0xffffffff as u64 - self.nb as u64) + 1),
				_ => format!("+0x{:x}", self.nb)
			};
		}

	}
	pub(crate) fn as_u32(&self) -> u32{
		self.nb
	}
	pub(crate) fn as_neg_u32(&self) -> u32{
		return match self.size{
			1 => match self.nb >= 0x80{
				true => (0xff as u32 - self.nb) + 1,
				_ => self.nb
			},
			2 => match self.nb >= 0x8000{
				true => (0xffff as u32 - self.nb) + 1,
				_ => self.nb
			},
			4 => match self.nb >= 0x80000000{
				true => (0xffffffff as u32 - self.nb) + 1,
				_ => self.nb
			},
			_ => 0 
		};
	}
	pub(crate) fn neg(&self) -> bool{
		return match self.size{
			1 => match self.nb >= 0x80{
				true => true,
				_ => false
			},
			2 => match self.nb >= 0x8000{
				true => true,
				_ => false
			},
			4 => match self.nb >= 0x80000000{
				true => true,
				_ => false
			},
			_ => false
		};
	}
}
pub(super) mod decode{
	use crate::prefixes;
	use crate::tables::*;
	use crate::utils::bin::*;
	use crate::intel::{*, Instruction,ModRegRm};
	pub(super) fn prefixes(data: [u8;MAX_INSTRUCTION_SIZE], ptr_item: &mut InstructionResult, x: &mut usize) -> u8{
		if ptr_item.is_ok(){
			let mut item = ptr_item.clone().unwrap();
			loop{
				if prefixes::PREFIX_LIST.iter().any(|i| i == &data[*x]){
					if *x >= 4{
						*ptr_item = Err(InstructionError::TooManyPrefixes);
						return 0;
					}
					item.prefixes.push(data[*x]);
					*x += 1;
					continue;
				}
				break;
			}
			item.size += item.prefixes.len();
			*ptr_item = Ok(item);
		}
		0
	}

	pub(super) fn recognize(data: [u8;MAX_INSTRUCTION_SIZE], ptr_item: &mut InstructionResult, x: &mut usize) -> u8{
		if ptr_item.is_ok(){
			let mut item = ptr_item.clone().unwrap();
			/* /!\ Debugging purpose */
			// println!("TARGET => {:02x} {:02x}", data[*x], data[*x + 1]);
			if data[*x] != prefixes::TWOBYTES{
				recognize_1b(data, &mut item, x);
			}else{
				recognize_2b(data, &mut item, x);
			}
			if item.mtype.is_none(){
				match item.opcode[0]{
					0xd8..0xe0 => item.mtype = Some(MainType::FPU),
					_ => *ptr_item = Err(InstructionError::InstructionNotRecognized)
				}
			}
			if ptr_item.is_ok(){
				*ptr_item = Ok(item);
			}
		}
		0
	}
	pub(self) fn recognize_1b(data: [u8;MAX_INSTRUCTION_SIZE], item: &mut Instruction, x: &mut usize) -> u8{
			item.opcode.push(data[*x]);
			let (a, b, c, d, e) = (
				[0x69,0x6b,0xd0,0xd1,0xd3,0xc0,0xc1,0x62,0xc4,0xc5,0xc8].iter().any(|i| i == &item.opcode[0]),
				mono::MONOBYTE.iter().any(|(i,..)| i == &item.opcode[0]),
				plri::PLURIBYTE.iter().any(|(i,..)| i == &item.opcode[0]),
				mdrm::MODRMBYTE.iter().any(|(i,..)| i == &item.opcode[0]),
				extm::EXTMODRMBYTE.iter().any(|(i,..)| i == &item.opcode[0])
			);
			match true{
				_ if a => {
					item.mtype = Some(MainType::ALU);
					*x += 1;
					item.r#type = Some(SubType::SpecialOp);
					item.modrm = Some(ModRegRm{
						byte: data[*x].encode(),
						index: *x
					});
				},
				_ if b  => {
					item.mtype = Some(MainType::ALU);
					item.r#type = Some(SubType::MonoB);
				},
				_ if c  => {
					item.mtype = Some(MainType::ALU);
					item.r#type = Some(SubType::PluriB);
				},
				_ if d => {
					item.mtype = Some(MainType::ALU);
					*x += 1;
					item.r#type = Some(SubType::ModrmB);
					item.modrm = Some(ModRegRm{
						byte: data[*x].encode(),
						index: *x
					});
				},
				_ if e => {
					item.mtype = Some(MainType::ALU);
					*x += 1;
					item.r#type = Some(SubType::ExtModrmB);
					item.modrm = Some(ModRegRm{
						byte: data[*x].encode(),
						index: *x
					});
				},
				_ => ()
			}
		0
	}

	#[allow(overlapping_patterns)] // Case properly handled (cause: Intel's usage of prefix to differenciate some MMX instruction with SSE).
	pub(self) fn recognize_2b(data: [u8;MAX_INSTRUCTION_SIZE], item: &mut Instruction, x: &mut usize) -> u8{
		item.size += 1;
		item.opcode.push(data[*x]);
		item.opcode.push(data[*x + 1]);
		*x += 1;
		match item.opcode[1]{
			_ if (item.opcode[1] == 0x01 && (0xc1..0xc5).contains(&data[*x + 1])) ||
				 (item.opcode[1] == 0x38 && [0x80,0x81].contains(&data[*x + 1]) && item.prefixes.contains(&0x66)) ||
				 ([0x78,0x79,0xc7].iter().any(|i| i == &item.opcode[1])) 
			=> item.mtype = Some(MainType::VMX),	

			0x4a | 0x4b | 0x06 | 0x08 | 0x09 | 0x0b | 0x30 | 0x31 | 0x32 | 0x33 | 
			0x34 | 0x35 | 0x37 | 0xa0 | 0xa1 | 0xa2 | 0xa8 | 0xa9 | 0xaa | 0xb9 | 
			0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x46 | 0x47 | 0x48 | 0x49 | 
			0x4c | 0x4d | 0x4e | 0x4f | 0x0d | 0x02 | 0x03 | 0xa3 | 0xab | 0xaf | 
			0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5 | 0xb8 | 0xbb | 0xbc | 0xbd | 
			0xc0 | 0xc1 | 0xa5 | 0xac | 0xa4 | 0xb6 | 0xb7 | 0xbe | 0xbf | 0x00 | 
			0x01 | 0xae | 0xba | 0x20 | 0x21 | 0x22 | 0x23 |
			0xc8..0xd0 | 0x80..0xae => {
				item.mtype = Some(MainType::ALU);
				item.r#type = Some(SubType::ExtOpB);
			},
			_ if item.opcode[1] == 0x38 && [0xf0,0xf1].contains(&data[*x + 1]) && !item.prefixes.contains(&0x2f) => {
				item.mtype = Some(MainType::ALU);
				item.r#type = Some(SubType::ExtOpB);
			},
			0x60..0x6c | 0x6e | 0x6f | 0x71..0x78 | 0x7e | 0x7f | 0xd1..0xd4 |
			0xd5 | 0xd8 | 0xd9 | 0xdb..0xe0 | 0xe1 | 0xe2 |
			0xe5 | 0xe8 | 0xe9 | 0xeb..0xf0 | 0xf1..0xf4 | 
			0xf5 | 0xf8 | 0xf9 | 0xfa | 0xfc..0xff => {
				if item.det.x16 || item.prefixes.iter().any(|i| i == &0xf3 || i == &0xf2){
					item.mtype = Some(MainType::SSE);
				}else{
					item.mtype = Some(MainType::MMX);
				}
			},
			0x10..0x18 | 0x28..0x30 | 0x38 | 0x3a | 0x50..0x80 | 0xc2..0xc7 | 0xd0..0xff => {
				item.mtype = Some(MainType::SSE);
			}
			_ => ()
		}
		0
	}
}