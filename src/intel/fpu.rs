use crate::tables::{Size, x87::*};
use crate::utils::bin::{Encode, Decode};
use crate::intel::{*, alu, enums::{*, AddressingMode::*}};

pub(super) fn asm_x87(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	{ // no operands
		for x in FPU_EXTB_MONO.iter(){
			if [item.det.extended[item.size],item.det.extended[item.size + 1]] == x.0{
				item.det.istr = x.1.to_string();
				item.size += x.0.len() + 1;
				*index += item.size;
				return Ok((*item).clone());			
			}
		}
	}
	
	{ // mod r/m
		item.modrm = Some(ModRegRm{
			byte: item.det.extended[item.size + 1].encode(),
			index: item.size + 1
		});
		let mut op_found: bool = false;
		let mut si: Option<Size> = None;
		let modrm = item.modrm.unwrap();
		let reg_opcode_extension = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode(); 
		for x in FPU_EXTB_MODRM.iter(){
			if item.det.extended[item.size] == x.0{
				op_found = true;
				if reg_opcode_extension == x.1{
					item.det.istr = x.2.to_string();
					si = Some(x.3);
					break;
				}
			}
		}
		if si.is_none(){
			return match op_found{
				true => Err(InstructionError::ModRmInvalid),
				_ => Err(InstructionError::InstructionNotRecognized)
			};
		}else{
			let decoded = alu::asm_mdrm(item, index, Settings{
				predefined: true, 
				one_operand: true, 
				immediate: true, // When ON, doesn't count d-bit rule, which is needed here.
				op_indx: 0,
				s: None,
				x: None
			});
			if decoded.is_err(){
				return decoded;
			}
			let m = decoded.unwrap().modrm.unwrap().byte.r#mod().unwrap();
			asm_proc_exception(item,m,reg_opcode_extension);
			if m == RegisterAddressingMode{
				asm_reg_eqv(item);
			}else{
				asm_adjust_size(item, si.unwrap());
			}
		}
	}
	Ok((*item).clone())
}

pub(self) fn asm_proc_exception(item: &mut Instruction, am: AddressingMode, reg_oe: u8){
	for x in PROC_EXCEPTION.iter(){
		if item.opcode[0] == (x.0)[0]{
			if reg_oe == (x.0)[1]{
				if am == RegisterAddressingMode{
					item.det.istr = item.det.istr.replace((x.1)[0],(x.1)[1]);
					break;
				}
			}
		}
	}
}
pub(self) fn asm_adjust_size(item: &mut Instruction, size: Size){
	use crate::tables::Size::*;
	let s = match size{
		Byte28 => "(28-bytes) ptr",
		Word => "word ptr",
		Dword => "dword ptr",
		Qword => "qword ptr",
		Tbyte => "tbyte ptr",
		_ => ""
	};
	for x in ["dword ptr","word ptr","byte ptr"].iter(){
		if item.det.istr.contains(x){
			item.det.istr = item.det.istr.replace(x,s);
			break;
		}
	}
}

pub(self) fn asm_reg_eqv(item: &mut Instruction){
	for x in ALU_FPU_REG.iter(){
		for i in x.0.iter(){
			if item.det.istr.contains(i){
				item.det.istr = item.det.istr.replace(i,x.1);
			}
		}
	}
}