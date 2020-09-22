use crate::tables::{*, simd::mmx::*};
use crate::utils::bin::{Encode, Decode};
use crate::intel::{*, alu, enums::AddressingMode::*};

pub(super) fn asm_mmx(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	item._0f = true;

	// Unique MMX instruction with no operands
	if item.opcode[1] == 0x77{
		item.det.istr = "emms".to_string();
		item.size += 1;
		*index += 2;
		return Ok(item.clone());
	}

	let modrm = ModRegRm{
			byte: item.det.extended[item.size + 1].encode(),
			index: item.size + 1
	};
	let reg_extension: u8 = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode();
	item.modrm = Some(modrm);
		
	for x in MMX_MODRM.iter(){
		if item.opcode[1] == x.0 && (reg_extension == x.1 || x.1 == 0xff){
			item.det.istr = x.2.to_string();
			let prefix = item.det.istr.clone();
			return match alu::asm_mdrm(item, index, Settings{
				predefined: true,
				one_operand: x.4,
				immediate: false,
				op_indx: 1,
				s: Some(X16_32),
				x: x.3
			}){
				Ok(_) => {
					if item.opcode[1] != 0x7e && item.opcode[1] != 0x7f{
						asm_reg_eqv(item, prefix, item.modrm.unwrap().byte.r#mod().unwrap(), None);
					}
					asm_handle_special(item, index)
				},
				err @ _ => err
			};
		}
	}
	Err(InstructionError::InstructionNotRecognized)
}

pub(self) fn asm_handle_special(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	match item.opcode[1]{
		0x7e | 0x7f => {
			let mut comma: usize = 0;
			for (index, chr) in item.det.istr.chars().enumerate(){
				if chr == ','{
					comma = index + 2;
					break;
				}
			}
			asm_reg_eqv(item, String::new(), RegisterIndirectAddressing, Some(comma));
		},
		0x71 | 0x72 | 0x73 => {
			item.imm = Digit::from(&item.det.extended, item.size, 1);
			item.det.istr = format!("{}, 0x{:x}", item.det.istr, item.imm.as_u32());
			item.size += 1;
			*index += 1;
		},
		_ => ()
	}
	if item.det.istr.contains("ptr") && item.opcode[1] != 0x7e{
		item.det.istr = item.det.istr.replace("dword","qword");
	}
	Ok(item.clone())
}
pub(self) fn asm_reg_eqv(item: &mut Instruction, prefix: String, addressing: AddressingMode, source_index: Option<usize>){
	for x in ALU_MMX_REG.iter(){
		for i in x.0.iter(){
			if match source_index{
				Some(_) => item.det.istr[source_index.unwrap()..].contains(i),
				_ if addressing != RegisterAddressingMode && addressing != RegisterIndirectAddressing => item.det.istr[prefix.len()..prefix.len()+4].contains(i),
				_ => item.det.istr.contains(i)
			}{
				match (addressing, source_index){
					(RegisterAddressingMode, _) => item.det.istr = item.det.istr.replace(i,x.1),
					(RegisterIndirectAddressing, Some(_)) => item.det.istr.replace_range(source_index.unwrap()..,x.1),
					(_, _) => item.det.istr.replace_range(prefix.len()+1..prefix.len()+4,x.1)
				}
				break;
			}
		}
	}
}