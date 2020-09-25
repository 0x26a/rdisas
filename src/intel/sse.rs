/*
---------------------------------------------------------------------------------------------------------------
SSE SHOULD BE CONSIDERED AS NOT SUPPORTED. THE TABLES AREN'T DONE AND THE ALGORITHM LINKED TO IT ISN'T FINISHED
---------------------------------------------------------------------------------------------------------------
*/

use crate::utils::bin::{Encode, Decode};
use crate::tables::{*, Size::*, simd::sse::*, simd::mmx::*};
use crate::intel::{*, alu, enums::AddressingMode::*};

pub(super) fn asm_sse(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	let modrm = ModRegRm{
			byte: item.det.extended[item.size + 1].encode(),
			index: item.size + 1
	};

	let byte_extension: u8 = item.det.extended[item.size + 1];
	let regf_extension: u8 = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode();

	item.modrm = Some(modrm);

	let mut prfx_f: bool = false;

	for x in SSE_MODRM.iter(){
		if item.prefixes.iter().any(|i| i == &0x66 || i == &0xf2 || i == &0xf3){
			for i in [0x66,0xf2,0xf3].iter(){
				if item.prefixes.contains(&i) && x.0.contains(&i){
					prfx_f = item.opcode[1] == x.1;
					break;
				}
			}
			if !prfx_f{ continue; }
		}
		if item.opcode[1] == x.1{
			if prfx_f{ prfx_f = !prfx_f; }
			let ext_f: bool = match (x.2, x.3){
				(Some(a @ _), _) => match a{
					e if e == byte_extension => {
						item.size += 1;
						item.modrm = Some(ModRegRm{
							byte: item.det.extended[item.size + 1].encode(),
							index: item.size + 1
						});
						true
					},
					_ => false
				},
				(_, Some(b @ _)) => b == regf_extension,
				(_, _) => true
			};
			if ext_f{
				item.det.x16 = false;
				item.det.istr = x.4.to_string();

				let mut memsize = x.6;
				let mut mnemonic = item.det.istr.clone();

				return match (alu::asm_mdrm(item, index, Settings{
					predefined: true,
					one_operand: false,
					immediate: false,
					op_indx: 1,
					s: Some(X16_32),
					x: x.5
				}), item.modrm.unwrap().byte.r#mod()){
					(Ok(_), Some(addressing @ _)) => {
						// post-process
						{
							mnemonic = match asm_handle_mnemonic_variations(item, &mut memsize, 0..mnemonic.len(), addressing){
								Some(m @ _) => m,
								_ => mnemonic
							};
							asm_reg_eqv(item, mnemonic.len()+1..mnemonic.len()+4, addressing, x.5);
							asm_reg_transmutation(item, mnemonic.len()+1..mnemonic.len()+4, x.7, addressing);
							asm_handle_special(item, index);
							asm_adjust_size(item, memsize);
						}
						Ok(item.clone())
					},
					(err @ Err(_), _) => err,
					(_, _) => Err(InstructionError::ModRmInvalid)
				};
			}
		}
	}
	Err(InstructionError::InstructionNotRecognized)
}
pub(self) fn asm_handle_special(item: &mut Instruction, index: &mut usize){
	match item.opcode[1]{
		_ => ()	
	}
}

pub(self) fn asm_adjust_size(item: &mut Instruction, size: Size){
	if item.det.istr.contains("ptr"){
		let replacement: &str = match size{
			Word => "word",
			Dword => "dword",
			Qword => "qword",
			XmmWord => "xmmword",
			_ => ""
		};
		for x in ["dword","word","byte"].iter(){
			if item.det.istr.contains(x){
				item.det.istr = item.det.istr.replace(x, replacement);
				break;
			}
		}
	}
}
pub(self) fn asm_reg_transmutation(item: &mut Instruction, dst: std::ops::Range<usize>, register_exception: Option<NoSseOperand>, addressing: AddressingMode){
	match register_exception{
		Some(reg @ _) => {
			let rng: std::ops::Range<usize> = match reg.loc{
				Role::Dst => dst.start..dst.end + 1,
				Role::Src => {
					let mut index: usize = 0;
					for (i, chr) in item.det.istr.chars().enumerate(){
						if chr == ','{
							index = i;
						}
					}
					index + 2..item.det.istr.len()
				},
			};
			for (j, x) in ALU_SSE_REG.iter().enumerate(){
				if item.det.istr[rng.clone()].contains(x.1){
					item.det.istr.replace_range(rng.clone(), match reg.r#type{
						RegisterType::Alu => x.0[0],
						RegisterType::Mmx => ALU_MMX_REG[j].1
					});
					break;
				}
			}
		},
		_ => ()
	}
}
pub(self) fn asm_handle_mnemonic_variations(item: &mut Instruction, size: &mut Size, mnemonic: std::ops::Range<usize>, addressing: AddressingMode) -> Option<String>{
	let mut new_mnemonic: Option<String> = None;
	if item.prefixes.iter().any(|i| i == &0x66 || i == &0xf2 || i == &0xf3){
		for x in SSE_PREFIX_MNEMONIC_VARIATION.iter(){
			if item.opcode[1] == x.0{
				if item.prefixes.contains(&x.1){
					item.det.istr.replace_range(mnemonic, x.2);
					new_mnemonic = Some(x.2.to_string());
					match x.3{
						Some(s @ _) => *size = s,
						_ => ()
					}
					break;
				}
			}
		}
	}else{
		for x in SSE_ADDRESSING_MNEMONIC_VARIATION.iter(){
			if item.opcode[1] == x.0{
				if addressing != x.1{
					item.det.istr.replace_range(mnemonic, x.2);
					new_mnemonic = Some(x.2.to_string());
					break;
				}
			}
		}
	}
	new_mnemonic
}
pub(self) fn asm_reg_eqv(item: &mut Instruction, dst: std::ops::Range<usize>, addressing: AddressingMode, order: Option<u8>){
	let mut valid_target: bool;
	let mut exit: bool = false;
	for x in ALU_SSE_REG.iter(){
		valid_target = true;
		for i in x.0.iter(){
			if item.det.istr[dst.start..].contains(i){
				match addressing{
					RegisterAddressingMode => item.det.istr = item.det.istr.replace(i, x.1),
					_ => {
						let (target_range, replacement): (std::ops::Range<usize>, &str) = match order{
							Some(o @ _) => match o{
								REG_MEM => (dst.clone(), x.1),
								MEM_REG => {
									let mut index: usize = 0;
									for (i, chr) in item.det.istr.chars().enumerate(){
										if chr == ','{
											index = i;
										}
									}
									(index + 2..item.det.istr.len(), x.1)
								},
								_ => (0..item.det.istr.len(), "")
							},
							_ => (0..item.det.istr.len(), "")
						};
						valid_target = match item.det.istr[target_range.clone()].contains(i){
							a @ true => {
								item.det.istr.replace_range(target_range.clone(), replacement);
								a
							},
							a @ _ => a
						};
					}
				}
				exit = valid_target;
				break;
			}
		}
		if exit && addressing != RegisterAddressingMode { break; }
	}
}  
