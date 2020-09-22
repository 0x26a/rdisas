use crate::tables::*;
use crate::utils::bin::{Encode, Decode};
use crate::intel::{*, alu};

pub(super) fn asm_vmx(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	if ![0x78,0x79,0xc7].iter().any(|i| i == &item.opcode[1]){
		item.size += 1;
	}
	return match (item.opcode[1], item.det.extended[item.size]){
		(0x01, ext @ _) => {
			item.det.istr = match ext{
				0xc1 => "vmcall",
				0xc2 => "vmlaunch",
				0xc3 => "vmresume",
				0xc4 => "vmxoff",
				_ => ""
			}.to_string();
			item.size += 2;
			*index += 2;
			Ok(item.clone())
		},
		_ => {
			let modrm = ModRegRm{
					byte: item.det.extended[item.size + 1].encode(),
					index: item.size + 1
			};
			let re: u8 = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode();
			item.modrm = Some(modrm);

			let (oop, s, x) = match (item.opcode[1], item.det.extended[item.size], re){
				(0x38, e @ _, _) => {
					item.det.istr = match e{
						0x80 => "invept",
						0x81 => "invvpid",
						_ => ""
					}.to_string();
					item.det.x16 = false;
					(Some(0), Some(X16_32), Some(REG_MEM))
				},
				(op @ 0x78 | op @ 0x79, _, _) => {
					item.det.istr = match op{
						0x78 => "vmread",
						0x79 => "vmwrite",
						_ => ""
					}.to_string();
					(Some(0), Some(X16_32), Some(match op{
						0x78 => MEM_REG,
						0x79 => REG_MEM,
						_ => 0
					}))
				},
				(0xc7, _, re @ 6 | re @ 7) => {
					item.det.istr = match (re, item.det.x16, item.prefixes.contains(&0xf3)){
						(6, false, false) => "vmptrld",
						(6, true, false) => "vmclear",
						(6, false, true) => "vmxon",
						(7, _, _) => "vmptrst",
						(_, _, _) => ""
					}.to_string();
					item.det.x16 = false;
					(Some(1), Some(X16_32), Some(MEM_REG))
				},
				_ => (None, None, None)
			};
			if [oop,s,x].iter().any(|i| i.is_none()){
				return Err(InstructionError::InstructionNotRecognized);
			}
			let oop: bool = match oop.unwrap(){
				0 => false,
				_ => true
			};
			return match alu::asm_mdrm(item, index, Settings{
				predefined: true,
				one_operand: oop,
				immediate: false,
				op_indx: 1,
				s: s,
				x: x
			}){
				Ok(mut item @ _) => {
					asm_handle_special(&mut item)
				},
				e @ Err(_) => e
			};
		}
	};
}

pub(self) fn asm_handle_special(item: &mut Instruction) -> InstructionResult{
	match item.opcode[1]{
		0x38 => {
			for x in ["dword","word","byte"].iter(){
				if item.det.istr.contains(x){
					item.det.istr = item.det.istr.replace(x, "oword");
					break;
				}
			}
		},
		0xc7 => {
			for x in ["dword","word","byte"].iter(){
				if item.det.istr.contains(x){
					item.det.istr = item.det.istr.replace(x, "qword");
					break;
				}
			}
		},
		_ => ()
	}
	Ok(item.clone())
}