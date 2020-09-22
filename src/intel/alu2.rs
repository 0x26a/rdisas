// Treat instruction with the 0F prefix (NO SSEx OR MMX)

use crate::utils::bin::{Encode, Decode};
use crate::intel::*;

pub(super) fn asm_x86_0f(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	item._0f = true;
	let op = item.opcode[1];

	// unique case handler
	let mut movbe: bool = false;
	{
		if op == 0x38 && [0xf0,0xf1].contains(&item.det.extended[item.size + 1]){
			item.size += 1;
			item.opcode.push(item.det.extended[item.size]);
			movbe = true;
		}
	}
	let indx = match op{
		0xc8..0xd0 => item.size,
		_ => item.size + 1
	};
	let modrm = ModRegRm{
			byte: item.det.extended[indx].encode(),
			index: indx
	};
	let re: u8 = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode();
	item.modrm = Some(modrm);

	let (a, b, c, d) = (
		crate::tables::mono::MONOBYTE_0F.iter().any(|(a,..)| a == &op),
		crate::tables::plri::PLURIBYTE_0F.iter().any(|(a,..)| a == &op),
		crate::tables::mdrm::MODRMBYTE_0F.iter().any(|(a,..)| a == &op),
		crate::tables::extm::EXTMODRMBYTE_0F.iter().any(|(a,b,..)| a == &op && b == &re)
	);
	return match true{
		_ if a => self::bridge::asm_mono_0f(item, index),
		_ if b => self::bridge::asm_plri_0f(item, index),
		_ if c => self::bridge::asm_mdrm_0f(item, index, None, movbe),
		_ if d => self::bridge::asm_mdrm_0f(item, index, Some(re), false),
		_ => Err(InstructionError::InstructionNotRecognized)
	}
}
// Basically a bridge between 0F instructions and functions defined in alu.rs
pub(self) mod bridge{
	use crate::intel::{*, alu};
	use crate::tables::{*, mdrm::*, extm::*};

	pub(super) fn asm_mono_0f(item: &mut Instruction, index: &mut usize) -> InstructionResult{
		item.modrm = None;
		alu::asm_mono(item, index)
	}
	pub(super) fn asm_plri_0f(item: &mut Instruction, index: &mut usize) -> InstructionResult{
		item.modrm = None;
		alu::asm_plri(item, index)
	} 
	pub(super) fn asm_mdrm_0f(item: &mut Instruction, index: &mut usize, re: Option<u8>, movbe: bool) -> InstructionResult{
		let (list, mut re) = match re{
			Some(_) => (EXTMODRMBYTE_0F, re.unwrap()),
			None => (MODRMBYTE_0F, 255)
		};
		for x in list.iter(){
			if movbe{re = item.det.extended[item.size];}
			if x.0 == item.opcode[1] && x.1 == re{
				item.det.istr = x.2.to_string();
				return asm_handle_special(alu::asm_mdrm(item, index, Settings{
					predefined: true,
					one_operand: x.6,
					immediate: x.5,
					op_indx: 1,
					s: x.4,
					x: x.3
				}), index, re);
			}
		}
		Err(InstructionError::InstructionNotRecognized)
	}
	pub(self) fn asm_handle_special(result: InstructionResult, index: &mut usize, re: u8) -> InstructionResult{
		if !result.is_ok(){
			return result;
		}
		let mut item = result.unwrap();
		if re == 255{
			match item.opcode[1]{	
				0xa5 | 0xad => {
					item.det.istr = format!("{}, cl", item.det.istr);
				},
				0xa4 | 0xac => {
					item.imm = Digit::from(&item.det.extended, item.size, 1);
					item.det.istr = format!("{}, 0x{:x}", item.det.istr, item.imm.as_u32());
					item.size += 1;
					*index += 1;
				},
				0xb6 | 0xbe => {
					let dst: &str = &(item.det.istr)[6..8];
					for x in REG_TABLE.iter(){
						if dst == x[2]{
							item.det.istr.replace_range(6..8, match item.det.x16{
								true => x[1],
								_ => x[0]
							});
							break;
						}
					}
				},
				0xb7 | 0xbf => {
					if item.det.istr.contains("ptr"){
						item.det.istr = item.det.istr.replace("byte","word");
						item.det.istr = item.det.istr.replace("dword","word");
					}else{
						let src: &str = &(item.det.istr)[11..];
						for x in REG_TABLE.iter(){
							if src == x[0]{
								item.det.istr.replace_range(11..,x[1]);
								break;
							}
						}
					}
				},
				op @ 0x20..0x24 => {
					let reg = match op{
						0x20 | 0x22 => "cr0",
						0x21 | 0x23 => "db0",
						_ => ""
					};
					item.det.istr = match op{
						0x20 | 0x21 => format!("{}, {}", item.det.istr, reg),
						0x22 | 0x23 => format!("{} {}, {}", &(item.det.istr)[0..3], reg, &(item.det.istr)[4..7]),
						_ => "".to_string()
					};
				},
				0xc8..0xd0 => {
					// r32 -> 0xc8+r (bswap)
					item.size = item.size - 1;
					*index = *index - 1;
				},
				_ => ()
			}
		}else{
			match (item.opcode[1], re){
				(0, 0..6) | (1, 4) | (1, 6) => {
					if item.det.istr.contains("ptr"){
						for x in ["byte","dword"].iter(){
							if item.det.istr.contains(x){
								item.det.istr = item.det.istr.replace(x,"word");
							}
						}
					}else{
						for x in REG_TABLE.iter(){
							if item.det.istr.contains(x[0]){
								item.det.istr = item.det.istr.replace(x[0],x[1]);
								break;
							}
						}
					}
				},
				(1, 0..4) | (1, 7) | (0xae, 0..2) | (0xae, 4..6) => {
					for x in ["dword","word","byte"].iter(){
						if item.det.istr.contains(x){
							item.det.istr = item.det.istr.replace(&format!("{} ptr ", x),"");
							break;
						}
					}
				},
				(0xba, 4..8) => {
					item.imm = Digit::from(&item.det.extended, item.size, 1);
					item.det.istr = format!("{}, 0x{:x}", item.det.istr, item.imm.as_u32());
					item.size += 1;
					*index += 1;
				},
				_ => ()
			}
		}
		Ok(item.clone())
	}
}