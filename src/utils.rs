use crate::intel::enums::{Register::*,
								AddressingMode::*,
								Index::*,
								*
};
pub(crate) type Byte = [u8;8];

pub(crate) mod bin{
	use crate::utils::Byte;
	const P2: [u8;8] = [128, 64, 32, 16, 8, 4, 2, 1];
	
	pub trait Decode{fn decode(&self) -> u8;}
	pub trait Encode{fn encode(&self) -> Byte;}
	impl Encode for u8{
		fn encode(&self) -> Byte{
			let mut b = *self;
			let mut bin: Byte = [0;8];
			for x in 0..8{
				match P2[x] > b{
					true => bin[x] = 0,
					_ => {
						bin[x] = 1;
						b = b - P2[x];
					}
				}
			}
			bin
		}
	}
	impl Decode for Byte{
		fn decode(&self) -> u8{
			let mut int: u8 = 0;
			for x in 0..8{
				match (*self)[x]{
					1 => int += P2[x],
					_ => (),
				}
			}
			int
		}
	}
}
pub(crate) mod display{
	use crate::intel::enums::*;
	use crate::intel::Instruction;

	impl std::fmt::Display for Index{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
			use Index::*;
			let scale: String = match *self{
				Mul1 => "*1",
				Mul2 => "*2",
				Mul4 => "*4",
				Mul8 => "*8"
			}.to_string();
			write!(f,"{}", scale)

		}
	}
	impl std::fmt::Display for Register{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
			use Register::*;

			let reg: String = match *self{
				Eax => "eax",
				Ecx => "ecx",
				Edx => "edx",
				Ebx => "ebx",
				Esp => "esp",
				Ebp => "ebp",
				Esi => "esi",
				Edi => "edi",
				Illegal => "", // C-DEBUG-NONEMPTY violated because value used in format!() (see alu.rs).
				_ => ""
			}.to_string();
			write!(f,"{}", reg)
		}
	}
	impl std::fmt::Display for Instruction{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
			write!(f, "{}", self.to_string())
		}
	}
	impl std::fmt::LowerHex for Instruction{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
			
			let mut hex: String = String::new();
			let raw = self.to_vec();
			for x in 0..raw.len(){
				match raw[x] <= 0xf{
					true => hex.push_str(&format!("0{:x} ", raw[x])),
					false => hex.push_str(&format!("{:x} ", raw[x]))
				}
			}
			write!(f, "{}", hex)
		}
	}
	impl std::fmt::UpperHex for Instruction{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
			let mut hex: String = String::new();

			let raw = self.to_vec();
			for x in 0..raw.len(){
				match raw[x] <= 0xf{
					true => hex.push_str(&format!("0{:X} ", raw[x])),
					false => hex.push_str(&format!("{:X} ", raw[x]))
				}
			}
			write!(f, "{}", hex)
		}
	}
}
pub(crate) trait Assembly{
	fn d(&self) -> u8;
	fn s(&self) -> u8;

	fn rm(&self) -> [u8;3];
	fn r#mod(&self) -> Option<AddressingMode>;
	fn reg(&self) -> Option<(Register, [&'static str;3])>;
	fn modrm_reg(&self, data: [u8;3]) -> Option<[&'static str;3]>;
	fn scale(&self) -> Option<Index>;
	fn index(&self) -> Option<Register>;
	fn base(&self, r#mod: AddressingMode) -> Option<Register>;
}
impl Assembly for Byte{
	fn s(&self) -> u8{ (*self)[7] }
	fn d(&self) -> u8{ (*self)[6] }

	fn r#mod(&self) -> Option<AddressingMode>{
		return match [(*self)[0],(*self)[1]]{
			[0,0] => match (*self).rm(){
				[1,0,0] => Some(SIBNoDisplacement),
				[1,0,1] => Some(DisplacementOnlyAddressing),
				_ => Some(RegisterIndirectAddressing)
			},
			[0,1] => Some(OneByteAfterMod),
			[1,0] => Some(FourByteAfterMod),
			[1,1] => Some(RegisterAddressingMode),
			_ => None
		};
	}
	fn reg(&self) -> Option<(Register, [&'static str;3])>{
		return match [(*self)[2],(*self)[3],(*self)[4]]{
			[0,0,0] => Some((Eax,["eax","ax","al"])),
			[0,0,1] => Some((Ecx,["ecx","cx","cl"])),
			[0,1,0] => Some((Edx,["edx","dx","dl"])),
			[0,1,1] => Some((Ebx,["ebx","bx","bl"])),
			[1,0,0] => Some((Esp,["esp","sp","ah"])),
			[1,0,1] => Some((Ebp,["ebp","bp","ch"])),
			[1,1,0] => Some((Esi,["esi","si","dh"])),
			[1,1,1] => Some((Edi,["edi","di","bh"])),
			_ => None
		};
	}
	fn modrm_reg(&self, data: [u8;3]) -> Option<[&'static str;3]>{
		return match data{
			[0,0,0] => Some(["eax","ax","al"]),
			[0,0,1] => Some(["ecx","cx","cl"]),
			[0,1,0] => Some(["edx","dx","dl"]),
			[0,1,1] => Some(["ebx","bx","bl"]),
			[1,0,0] => Some(["esp","sp","ah"]),
			[1,0,1] => Some(["ebp","bp","ch"]),
			[1,1,0] => Some(["esi","si","dh"]),
			[1,1,1] => Some(["edi","di","bh"]),
			_ => None
		};
	}	
	fn rm(&self) -> [u8;3]{ [(*self)[5],(*self)[6],(*self)[7]] }

	fn base(&self, r#mod: AddressingMode) -> Option<Register>{
		let permutation: Byte = [0,0,(*self)[5],(*self)[6],(*self)[7],0,0,0] as Byte;
		let reg = permutation.reg();
		if reg.is_some(){
			let reg = reg.unwrap().0;
			if reg == Ebp{
				return match r#mod{
					SIBNoDisplacement		   |
					DisplacementOnlyAddressing |
					RegisterIndirectAddressing => Some(DisplacementOnly),
					
					OneByteAfterMod | FourByteAfterMod => Some(reg),
					_ => None
				};
			}
			return Some(reg);
		}	
		None
	}
	fn index(&self) -> Option<Register>{
		let reg = (*self).reg();
		if reg.is_some(){
			let reg = reg.unwrap().0;
			return match reg{
				Esp => Some(Illegal),
				_ => Some(reg)
			};
		}
		None
	}
	fn scale(&self) -> Option<Index>{
		return match [(*self)[0],(*self)[1]]{
			[0,0] => Some(Mul1),
			[0,1] => Some(Mul2),
			[1,0] => Some(Mul4),
			[1,1] => Some(Mul8),
			_ => None
		};
	}
}