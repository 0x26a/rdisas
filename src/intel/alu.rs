use crate::tables::mdrm::MODRMBYTE;
use crate::tables::extm::EXTMODRMBYTE;
use crate::utils::bin::{Encode, Decode};
use crate::intel::{*, enums::{*, AddressingMode::*}};

pub(super) fn asm_special(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	let mut r: InstructionResult = Ok(item.clone());
	match item.opcode[match item._0f{
		true => 1,
		_ => 0
	}]{
		op @ 0x69 | op @ 0x6b => {
			item.det.istr = "imul".to_string();
			r = asm_mdrm(item, index, Settings{
				predefined: true,
				one_operand: false,
				immediate: false,
				op_indx: 0,
				s: None,
				x: None
			});
			let imm_size: usize = match op{
				0x6b => 1,
				_ => match item.det.x16{
					true => 2,
					_ => 4
				}
			};
			item.imm = Digit::from(&item.det.extended, item.size, imm_size);
			item.det.istr = format!("{}, 0x{:x}", item.det.istr, item.imm.as_u32());
			item.size += imm_size;
			*index += imm_size;
		},
		op @ 0xd0..0xd4 => {
			let op2: &str = match op{
				0xd0 | 0xd1 => "1",
				_ => "cl"
			};
			r = asm_mdrm(item, index, Settings{
				predefined: false,
				one_operand: true,
				immediate: false,
				op_indx: 0,
				s: None,
				x: None
			});
			item.det.istr = format!("{}, {}", item.det.istr,op2);
		},
		0xc0 | 0xc1 => {
			r = asm_mdrm(item, index, Settings{
				predefined: false,
				one_operand: true,
				immediate: false,
				op_indx: 0,
				s: None,
				x: None
			});
			item.imm = Digit::from(&item.det.extended, item.size, 1);
			item.det.istr = format!("{}, 0x{:x}", item.det.istr, item.imm.as_u32());
			item.size += 1;
			*index += 1;
		},
		0x62 => {
			r = asm_mdrm(item, index, Settings{
				predefined: false,
				one_operand: false,
				immediate: false,
				op_indx: 0,
				s: None,
				x: None
			});
			item.det.istr = match item.det.x16{
				true => item.det.istr.replace("word ptr","dword ptr"), 
				_ => item.det.istr.replace("dword ptr","qword ptr")
			};
		},
		op @ 0xc4..0xc6 => {
			item.det.istr = match op{
				0xc4 => "les",
				0xc5 => "lds",
				_ => ""
			}.to_string();
			r = asm_mdrm(item, index, Settings{
				predefined: true,
				one_operand: false,
				immediate: false,
				op_indx: 0,
				s: None,
				x: None
			});
			item.det.istr = match item.det.x16{
				true => item.det.istr.replace("word ptr","dword ptr"),
				_ => item.det.istr.replace("dword ptr","fword ptr")
			};
		},
		0xc8 => {
			item.size += 1;
			item.imm = Digit::from(&item.det.extended,item.size,2);
			let tmp = item.imm.as_u32() as u16;
			item.size += 2;
			item.imm = Digit::from(&item.det.extended,item.size,1);
			item.size += 1;
			item.det.istr = format!("enter 0x{:x}, 0x{:x}", tmp, item.imm.as_u32());
			*index += item.size;
			item.imm = Digit::from(&[(tmp & 0xff) as u8,(tmp >> 8) as u8,item.imm.as_u32() as u8, 0], 0, 3);
			item.modrm = None;
		},
		_ => r = Err(InstructionError::UnknownError)
	}
	if r.is_ok(){
		return Ok(item.clone());
	}
	r
}
pub(super) fn asm_mono(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	use crate::tables::mono::*;
	item.size += 1;
	*index += item.size;
	Ok(match item._0f{
		true => {	
			for x in MONOBYTE_0F.iter(){
				if item.opcode[1] == x.0{
					item.det.istr = match item.det.x16{
						true => (x.1)[1],
						false => (x.1)[0]
					}.to_string();
					break;
				}
			}
			item.clone()
		},
		_ => {
			for x in MONOBYTE.iter(){
				if item.opcode[0] == x.0{
					item.det.istr = match item.det.x16{
						true => (x.1)[1],
						false => (x.1)[0]
					}.to_string();
					break;
				}
			}
			item.clone()
		}
	})
}
pub(super) fn asm_plri(item: &mut Instruction, index: &mut usize) -> InstructionResult{
	use crate::tables::plri::*;
	let mut entry: (u8, bool, [&'static str;2]) = (0,false,["",""]);
	let (op, oplist, reldlist) = match item._0f{
		true => (1, PLURIBYTE_0F, RELATIVE_DISPLACEMENT_0F),
		_ => (0, PLURIBYTE, RELATIVE_DISPLACEMENT)
	};
	for x in oplist.iter(){
		if x.0 == item.opcode[op]{
			entry = *x;
		}
	}
	item.size += 1;
	item.det.istr = match item.det.x16{
		true => (entry.2)[1],
		false => (entry.2)[0]
	}.to_string();
	let size: usize = match entry.1{
		X8 => 1,
		X16_32 => match item.det.x16{
			true => match item.det.istr.contains("["){
				true => 4,
				false => 2
			},
			false => 4
		}
	};
	item.imm = Digit::from(&item.det.extended, item.size, size);
	item.size += size;
	*index += item.size;
	if reldlist.iter().any(|i| i == &item.opcode[op]){
		let relative_addr: i32 = match item.imm.neg(){
			true => *index as i32 - item.imm.as_neg_u32() as i32,
			_ => item.imm.as_u32() as i32 + *index as i32
		};
		item.det.istr = match item.opcode[op]{
			0x9a | 0xea => { /* far absolute */
				item.imm = Digit::from(&item.det.extended, item.size, 2);
				item.size += 2;
				*index += 2;
				let tmp = item.imm.as_u32();
				item.imm = Digit::from(&item.det.extended, item.size - (size + 2), size + 2);
				item.det.istr.replace("$",&format!("0x{:x}:0x{:x}",tmp,relative_addr))
			} ,
			_ => item.det.istr.replace("$",&format!("0x{:x}",relative_addr))
		};
	}else{
		item.det.istr = item.det.istr.replace("$",&format!("0x{:x}",item.imm.as_u32()));
	}
	Ok(item.clone())
}
pub(super) fn asm_mdrm(item: &mut Instruction, index: &mut usize, mut set: Settings) -> InstructionResult{
	let mut r: InstructionResult = Ok(item.clone());
	let op: u8 = item.opcode[set.op_indx];
	let modrm = item.modrm.unwrap();
	if !set.predefined{
		for x in MODRMBYTE.iter(){
			if x.0 == op{item.det.istr = x.1.to_string();}
		}
		if item.det.istr == String::new(){
			let reg_opcode_extension = ([0,0,0,0,0,modrm.byte[2],modrm.byte[3],modrm.byte[4]] as Byte).decode() as usize; 
			for x in EXTMODRMBYTE.iter(){
				if x.0 == op{
					if reg_opcode_extension >= x.1.len(){
						return Err(InstructionError::ModRmInvalid);
					}else{
						item.det.istr = (x.1)[reg_opcode_extension].0.to_string();
						set.one_operand = (x.1)[reg_opcode_extension].1;
						break;
					}
				}
			}
		}
	}
	item.size += 2;
	let mode = modrm.byte.r#mod();
	if !mode.is_some(){
		r = Err(InstructionError::ModRmInvalid);
		return r;
	}else{
		let (mut s, mut x) = (op.encode().s(), op.encode().d());
		asm_exception(item, &mut set);
		if set.s.is_some(){ s = set.s.unwrap(); }
		if set.x.is_some(){ x = set.x.unwrap(); }
		let d = x;
		let mode = mode.unwrap();
		println!("{:?}", mode);
		match mode{
			RegisterAddressingMode => {
				let (reg,rm) = (modrm.byte.reg(),modrm.byte.modrm_reg(modrm.byte.rm()));
				if reg.is_some() && rm.is_some(){
					if set.immediate{
						let dst = asm_reg(item, rm.unwrap(), s).0;
						if set.one_operand{
							item.det.istr = format!("{} {}", item.det.istr, dst);
						}else{
							let mut size = match s{
								0 => 1,
								1 => match item.det.x16{
										true => 2,
										_ => 4
									},
								_ => 0
							};
							if size % 2 == 0 && x == 1{
								size = 1;
							}
							item.imm = Digit::from(&item.det.extended, item.size, size);
							item.det.istr = format!("{} {}, 0x{:x}", item.det.istr, dst, item.imm.as_u32());
							item.size += size;
						}
					}else{
						let (mut src, mut dst) = {
							let tmp = (
								asm_reg(item, reg.unwrap().1, s).0, 
								asm_reg(item, rm.unwrap(), s).0
							);
							tmp
						};
						if set.one_operand{
							item.det.istr = format!("{} {}", item.det.istr, dst);
						}else{
							if d == 1{
								std::mem::swap(&mut src, &mut dst);
							}
							item.det.istr = format!("{} {}, {}", item.det.istr, dst, src);
						}
					}
				}
			},
			DisplacementOnlyAddressing => {
				let reg = modrm.byte.reg();
				if reg.is_some(){
					if set.immediate{
						let (_, opsize, mut size) = asm_imm_reg(item, reg.unwrap().1, s);
						if size % 2 == 0 && x == 1{
							size = 1;
						}
						item.disp = Digit::from(&item.det.extended, item.size, 4);
						item.size += 4;
						item.imm = Digit::from(&item.det.extended, item.size, size);
						item.det.istr = match set.one_operand{
							false => format!("{} {} @:[0x{:x}], 0x{:x}", item.det.istr, opsize, item.disp.as_u32(), item.imm.as_u32()),
							true => {
								item.size = item.size - item.imm.to_vec().len();
								item.imm = Digit::from(&[],0,0);
								format!("{} {} @:[0x{:x}]", item.det.istr, opsize, item.disp.as_u32())
							}
						};
						item.size += size;
					}else{
						let (mut src, mut dst): (&str,&str) = {
							let (reg, opsize) = asm_reg(item, reg.unwrap().1, s);
							item.disp = Digit::from(&item.det.extended, item.size, 4);
							(reg, &format!("{} @:[0x{:x}]", opsize, item.disp.as_u32()))
						};
						if (src, dst) == ("",""){
							r = Err(InstructionError::ModRmInvalid);
						}else{
							if !set.one_operand{
								if d == 1{
									std::mem::swap(&mut src, &mut dst);
								}
							}
							item.det.istr = match set.one_operand{
								false => format!("{} {}, {}", item.det.istr, dst, src),
								true => format!("{} {}", item.det.istr, dst)
							};
							item.size += 4;
						}
					}
				}
			},
			RegisterIndirectAddressing => {
				if set.immediate{
					let rm = modrm.byte.modrm_reg(modrm.byte.rm());
					if rm.is_some(){
						let (dst, opsize, mut size) = asm_imm_reg(item, rm.unwrap(), s);
						if size % 2 == 0 && x == 1{
							size = 1;
						}
						let dst = format!("{} @:[{}]", opsize, dst);
						item.imm = Digit::from(&item.det.extended, item.size, size);
						item.det.istr = match set.one_operand{
							false => {
								item.size += size;
								format!("{} {}, 0x{:x}", item.det.istr, dst, item.imm.as_u32())
							},
							true => {
								item.imm = Digit::from(&[],0,0);
								format!("{} {}", item.det.istr, dst)
							}
						};
					}
				}else{
					let mut memop: &str = "";
					let (src, dst): (Option<&str>,Option<&str>) = {
						let mut tmp: (Option<&str>,Option<&str>) = (None, None);
						let (reg,rm) = (modrm.byte.reg(),modrm.byte.modrm_reg(modrm.byte.rm()));
						if reg.is_some() && rm.is_some(){		
							let _tmp_reg = asm_reg(item, reg.unwrap().1, s);
							memop = _tmp_reg.1;
							tmp = match (_tmp_reg.0, asm_imm_reg(item, rm.unwrap(), s).0){
								("", _) | (_, "") => (None, None),
								(a @ _, b @ _) => (Some(a), Some(b))
							};
						}
						tmp
					};
					if src.is_none() || dst.is_none(){
						r = Err(InstructionError::ModRmInvalid);
					}else{
						let (mut src, dst) = (src.unwrap(),dst.unwrap());
						let mut dst: &str = &format!("{} @:[{}]", memop, dst);
						if !set.one_operand{
							if d == 1{
								std::mem::swap(&mut src, &mut dst);
							}
						}
						item.det.istr = match set.one_operand{
							false => format!("{} {}, {}", item.det.istr, dst, src),
							true => format!("{} {}", item.det.istr, dst)
						};
					}
				}
			},
			SIBNoDisplacement => {
				use crate::intel::enums::Register::*;
				let reg = modrm.byte.reg();
				if reg.is_some(){
					let (mut src, mut dst): (Option<String>,Option<String>) = {
						let mut size: usize = 0;
						let mut operand: Option<String> = None;
						let (_reg, mut opsize) = asm_reg(item, reg.unwrap().1, s);
						if set.immediate{
							let (_, a, b) = asm_imm_reg(item, reg.unwrap().1, s);
							opsize = a;
							size = b;
							if size % 2 == 0 && x == 1{
								size = 1;
							}
						}
						item.sib = Some(item.det.extended[modrm.index + 1].encode());
						let sib = item.sib.unwrap();
						let (base, index, scale) = (
							sib.base(mode),
							sib.index(),
							sib.scale()
						);
						if base.is_some() && index.is_some() && scale.is_some(){
							operand = match true{
								_ if base == Some(DisplacementOnly) => {
									item.disp = Digit::from(&item.det.extended, item.size + 1, 4);
									item.size += 4;
									Some(format!("{} @:[{}{}{}]", opsize, index.unwrap(), scale.unwrap(), item.disp.to_disp()))
								},
								_ => match index{
									Some(Illegal) => Some(format!("{} @:[{}]", opsize, base.unwrap())),
									_ => Some(format!("{} @:[{}+{}{}]", opsize, base.unwrap(), index.unwrap(), scale.unwrap()))
								} 
							};
							if set.immediate{
								item.size += 1;
								item.imm = Digit::from(&item.det.extended, item.size, size);
								item.size += size;
							}
						}
						(Some(_reg.to_string()), operand.clone())
					};
					if dst.is_none()				    	{ r = Err(InstructionError::ModRmOrSibInvalid); }
					else if src.is_none() && !set.immediate { r = Err(InstructionError::ModRmOrSibInvalid); }
					else{
						if !set.immediate{
							if !set.one_operand{
								if d == 1{
									std::mem::swap(&mut src, &mut dst);
								}
							}
							item.det.istr = match set.one_operand{
								false => format!("{} {}, {}", item.det.istr, dst.unwrap(), src.unwrap()),
								true => format!("{} {}", item.det.istr, dst.unwrap())
							};
							item.size += 1;
						}else{
							item.det.istr = match set.one_operand{
								false => format!("{} {}, 0x{:x}", item.det.istr, dst.unwrap(), item.imm.as_u32()),
								true => {
									item.size = item.size - item.imm.to_vec().len();
									item.imm = Digit::from(&[],0,0);
									format!("{} {}", item.det.istr, dst.unwrap())
								}
							};
						}
					}
				}
			},
			m @ OneByteAfterMod | m @ FourByteAfterMod => {
				let disp_size: usize = match m{
					OneByteAfterMod => 1,
					FourByteAfterMod => 4,
					_ => 0
				};
				let (reg,rm) = (modrm.byte.reg(),modrm.byte.modrm_reg(modrm.byte.rm()));
				if reg.is_some() && rm.is_some(){
					let mut has_sib: bool = false;
					let mut base: Option<Register> = None;
					let mut index: Option<Register> = None;
					let mut scale: Option<Index> = None;
					let (reg, rm) = (reg.unwrap(), rm.unwrap());
					if modrm.byte.rm() == [1,0,0]{
						item.sib = Some(item.det.extended[modrm.index + 1].encode());
						let sib = item.sib.unwrap();
						base = sib.base(mode);
						index = sib.index();
						scale = sib.scale();
						if base.is_none() || index.is_none() || scale.is_none(){
							r = Err(InstructionError::ModRmInvalid);
							return r;
						}
						has_sib = true;
						item.size += 1;
					}
					if set.immediate{
						let dst: String = {
							let tmp: String;
							item.disp = Digit::from(&item.det.extended, item.size, disp_size);
							item.size += disp_size;
							let (rm, opsize, size) = asm_imm_reg(item,rm,s);
							item.imm = Digit::from(&item.det.extended, item.size, size);
							if !set.one_operand{
								item.size += size;
							}
							tmp = match has_sib{
								false => format!("{} @:[{}{}]", opsize, rm, item.disp.to_disp()),
								_ => {
									let output: String;
									let index = index.unwrap();
									if index != Register::Illegal{
										output = format!("{} @:[{}+{}{}{}]", opsize, base.unwrap(), index, scale.unwrap(), item.disp.to_disp());
									}else{
										output = format!("{} @:[{}{}]", opsize, base.unwrap(), item.disp.to_disp());
									}
									output
								}
							};
							tmp.clone()
						};
						item.det.istr = match set.one_operand{
							false => format!("{} {}, 0x{:x}", item.det.istr, dst, item.imm.as_u32()),
							true => {
								item.imm = Digit::from(&[],0,0);
								format!("{} {}", item.det.istr, dst)
							}
						};
					}else{
						let (mut src, mut dst): (String,String) = {
							let (reg, opsize) = asm_reg(item, reg.1, s);
							let (rm, _, _) = asm_imm_reg(item, rm,s);
							item.disp = Digit::from(&item.det.extended, item.size, disp_size);
							let rm = match has_sib{
								false => format!("{} @:[{}{}]", opsize, rm, item.disp.to_disp()),
								_ => format!("{} @:[{}+{}{}{}]", opsize, base.unwrap(), index.unwrap(), scale.unwrap(), item.disp.to_disp())
							};
							(reg.to_string(), rm.clone())
						};
						if d == 1{
							std::mem::swap(&mut src, &mut dst);
						}
						item.det.istr = match set.one_operand{
							false => format!("{} {}, {}", item.det.istr, dst, src),
							true => format!("{} {}", item.det.istr, dst)
						};
						item.size += disp_size;
					}
				}
			}
		}
	}
	if r.is_ok(){
		*index += item.size;
		r = Ok(item.clone());
	}
	r
}
pub(super) fn asm_exception(item: &mut Instruction, set: &mut Settings){
	let list: &[u8] = match item._0f{
		true => &[],
		_ => {
			match item.opcode[0]{
				0x62 => set.s = Some(1),
				0x63 => {
					item.det.x16 = true;
					set.x = Some(0);
				},
				0x8d | 0x69 => set.s = Some(1),
				0xd8 => item.det.x16 = false,
				0xc6 => set.x = Some(1),
				0xc7 => set.x = Some(0),
				0xc5 | 0xc4  => {
					set.x = Some(1);
					set.s = Some(1);
				},
				_ => ()
			}
			&[0xf6, 0xf7, 0xd8]
		}
	};
	let expr = list.iter().any(|i| i == &item.opcode[(item._0f as u8) as usize]);
	set.immediate = match set.immediate{
		_ if expr => false,
		a @ _ => a
	};
}
pub(super) fn asm_imm_reg(item: &mut Instruction, reg: [&'static str;3], s: u8) -> (&'static str, &'static str, usize){
	return match s{
		0 => (reg[0], "byte ptr", 1),
		1 => match item.det.x16{
			true => (reg[0], "word ptr", 2),
			false => (reg[0], "dword ptr", 4)
		},
		_ => ("","",0)
	};
}
pub(super) fn asm_reg(item: &mut Instruction, reg: [&'static str;3], s: u8) -> (&'static str, &'static str){
	return match s{
		0 => (reg[2], "byte ptr"),
		1 => match item.det.x16{
			true => (reg[1], "word ptr"),
			false => (reg[0], "dword ptr")
		}
		_ => ("","")
	}
}