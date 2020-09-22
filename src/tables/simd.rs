pub mod mmx{
	use crate::tables::*;

	pub const ALU_MMX_REG: [([&'static str;3],&'static str);8] = 
	[
		(["eax","ax","al"],"mm0"),
		(["ecx","cx","cl"],"mm1"),
		(["edx","dx","dl"],"mm2"),
		(["ebx","bx","bl"],"mm3"),
		(["esp","sp","ah"],"mm4"),
		(["ebp","bp","ch"],"mm5"),
		(["esi","si","dh"],"mm6"),
		(["edi","di","bh"],"mm7")
	];
	pub const MMX_MODRM: &[(u8, u8, &'static str, Option<u8>, bool)] = 
	&[	
		(0x60, 0xff, "punpcklbw", Some(REG_MEM), false),
		(0x61, 0xff, "punpcklwd", Some(REG_MEM), false),
		(0x62, 0xff, "punpckldq", Some(REG_MEM), false),
		(0x63, 0xff, "packsswb", Some(REG_MEM), false),
		(0x64, 0xff, "pcmpgtb", Some(REG_MEM), false),
		(0x65, 0xff, "pcmpgtw", Some(REG_MEM), false),
		(0x66, 0xff, "pcmpgtd", Some(REG_MEM), false),
		(0x67, 0xff, "packuswb", Some(REG_MEM), false),
		(0x68, 0xff, "punpckhbw", Some(REG_MEM), false),
		(0x69, 0xff, "punpckhwd", Some(REG_MEM), false),
		(0x6a, 0xff, "punpckhdq", Some(REG_MEM), false),
		(0x6b, 0xff, "packssdw", Some(REG_MEM), false),
		(0x6e, 0xff, "movd", Some(REG_MEM), false),
		(0x6f, 0xff, "movq", Some(REG_MEM), false),
		(0x71, 0x02, "psrlw", Some(REG_MEM), true),
		(0x71, 0x04, "psraw", Some(REG_MEM), true),
		(0x71, 0x06, "psllw", Some(REG_MEM), true),
		(0x72, 0x02, "psrld", Some(REG_MEM), true),
		(0x72, 0x04, "psrad", Some(REG_MEM), true),
		(0x72, 0x06, "pslld", Some(REG_MEM), true),
		(0x73, 0x02, "psrlq", Some(REG_MEM), true),
		(0x73, 0x06, "psllq", Some(REG_MEM), true),
		(0x74, 0xff, "pcmpeqb", Some(REG_MEM), false),
		(0x75, 0xff, "pcmpeqw", Some(REG_MEM), false),
		(0x76, 0xff, "pcmpeqd", Some(REG_MEM), false),
		(0x7e, 0xff, "movd", Some(MEM_REG), false),
		(0x7f, 0xff, "movq", Some(MEM_REG), false),
		(0xd1, 0xff, "psrlw", Some(REG_MEM), false),
		(0xd2, 0xff, "psrld", Some(REG_MEM), false),
		(0xd3, 0xff, "psrlq", Some(REG_MEM), false),
		(0xd5, 0xff, "pmullw", Some(REG_MEM), false),
		(0xd8, 0xff, "psubusb", Some(REG_MEM), false),
		(0xd9, 0xff, "psubusw", Some(REG_MEM), false),
		(0xdb, 0xff, "pand", Some(REG_MEM), false),
		(0xdc, 0xff, "paddusb", Some(REG_MEM), false),
		(0xdd, 0xff, "paddusw", Some(REG_MEM), false),
		(0xdf, 0xff, "pandn", Some(REG_MEM), false),
		(0xe1, 0xff, "psraw", Some(REG_MEM), false),
		(0xe2, 0xff, "psrad", Some(REG_MEM), false),
		(0xe5, 0xff, "pmulhw", Some(REG_MEM), false),
		(0xe8, 0xff, "psubsb", Some(REG_MEM), false),
		(0xe9, 0xff, "psubsw", Some(REG_MEM), false),
		(0xeb, 0xff, "por", Some(REG_MEM), false),
		(0xec, 0xff, "paddsb", Some(REG_MEM), false),
		(0xed, 0xff, "paddsw", Some(REG_MEM), false),
		(0xef, 0xff, "pxor", Some(REG_MEM), false),
		(0xf1, 0xff, "psllw", Some(REG_MEM), false),
		(0xf2, 0xff, "pslld", Some(REG_MEM), false),
		(0xf3, 0xff, "psllq", Some(REG_MEM), false),
		(0xf5, 0xff, "pmaddwd", Some(REG_MEM), false),
		(0xf8, 0xff, "psubb", Some(REG_MEM), false),
		(0xf9, 0xff, "psubw", Some(REG_MEM), false),
		(0xfa, 0xff, "psubd", Some(REG_MEM), false),
		(0xfc, 0xff, "paddb", Some(REG_MEM), false),
		(0xfd, 0xff, "paddw", Some(REG_MEM), false),
		(0xfe, 0xff, "paddd", Some(REG_MEM), false)
	];
}

pub mod sse{
	use crate::tables::{*, Size::*};
	use crate::intel::enums::{*, AddressingMode::*};
	use self::{Role::*, RegisterType:: *, NoSseOperand as N};

	// Support of SSE instructions using MMX registers
	#[derive(Clone, Copy, Debug, PartialEq, Hash)]
	pub enum Role{
		Src,
		Dst
	}
	#[derive(Clone, Copy, Debug, PartialEq, Hash)]
	pub enum RegisterType{
		Alu,
		Mmx
	}
	#[repr(C)]
	#[derive(Clone, Copy)]
	pub struct NoSseOperand{
		pub r#type: RegisterType,
		pub loc: Role
	}

	pub const ALU_SSE_REG: [([&'static str;3],&'static str);8] = 
	[
		(["eax","ax","al"],"xmm0"),
		(["ecx","cx","cl"],"xmm1"),
		(["edx","dx","dl"],"xmm2"),
		(["ebx","bx","bl"],"xmm3"),
		(["esp","sp","ah"],"xmm4"),
		(["ebp","bp","ch"],"xmm5"),
		(["esi","si","dh"],"xmm6"),
		(["edi","di","bh"],"xmm7")
	];

	pub const SSE_MODRM: &[(&[u8], u8, Option<u8>, Option<u8>, &'static str, Option<u8>, Size, Option<NoSseOperand>)] =
	&[
// prefix, opcode, external byte, reg (mod r/m) extension, mnemonic, operand order, mem size
		(&[], 0x10, None, None, "movups", Some(REG_MEM), XmmWord, None),
		(&[0xf3], 0x10, None, None, "movss", Some(REG_MEM), Dword, None),
		(&[0x66], 0x10, None, None, "movupd", Some(REG_MEM), XmmWord, None),
		(&[0xf2], 0x10, None, None, "movsd", Some(REG_MEM), Qword, None),
		(&[], 0x11, None, None, "movups", Some(MEM_REG), XmmWord, None),
		(&[0xf3], 0x11, None, None, "movss", Some(MEM_REG), Dword, None),
		(&[0x66], 0x11, None, None, "movupd", Some(MEM_REG), XmmWord, None),
		(&[0xf2], 0x11, None, None, "movsd", Some(MEM_REG), Qword, None),
		(&[0x66,0xf2,0xf3], 0x12, None, None, "movhlps", Some(REG_MEM), Qword, None),
		(&[0x66], 0x13, None, None, "movlps", Some(MEM_REG), Qword, None),
		(&[0x66], 0x14, None, None, "unpcklps", Some(REG_MEM), XmmWord, None),
		(&[0x66], 0x15, None, None, "unpckhps", Some(REG_MEM), XmmWord, None),
		(&[0x66,0xf2,0xf3], 0x16, None, None, "movhlps", Some(REG_MEM), Qword, None),
		(&[0x66], 0x17, None, None, "movhps", Some(MEM_REG), Qword, None),
		(&[0x66], 0x28, None, None, "movaps", Some(REG_MEM), XmmWord, None),
		(&[0x66], 0x29, None, None, "movaps", Some(MEM_REG), XmmWord, None),
		(&[0x66], 0x2a, None, None, "cvtpi2ps", Some(REG_MEM), Qword, Some(N{r#type: Mmx, loc: Src})),
		(&[0xf2,0xf3], 0x2a, None, None, "cvtsi2sd", Some(REG_MEM), Dword, Some(N{r#type: Alu, loc: Src})),
		(&[0x66], 0x2b, None, None, "movntps", Some(MEM_REG), XmmWord, None),
		(&[0x66], 0x2c, None, None, "cvttps2pi", Some(REG_MEM), Qword, Some(N{r#type: Mmx, loc: Dst})),
		(&[0xf2,0xf3], 0x2c, None, None, "cvttsd2si", Some(REG_MEM), Qword, Some(N{r#type: Alu, loc: Dst})),
		(&[0x66], 0x2d, None, None, "cvtps2pi", Some(REG_MEM), Qword, Some(N{r#type: Alu, loc: Dst})),
		(&[0xf2,0xf3], 0x2d, None, None, "cvtsd2si", Some(REG_MEM), Qword, Some(N{r#type: Mmx, loc: Dst})),
		(&[0x66], 0x2e, None, None, "ucomiss", Some(REG_MEM), Dword, None),
		(&[0x66], 0x2f, None, None, "comiss", Some(REG_MEM), Dword, None),

	];

	// Prefixes causing mnemonic/memory operand's size variations
	pub const SSE_PREFIX_MNEMONIC_VARIATION: &[(u8, u8, &'static str, Option<Size>)] = 
	&[
		(0x10, 0xf3, "movss", Some(Dword)),
		(0x10, 0x66, "movupd", Some(XmmWord)),
		(0x10, 0xf2, "movsd", Some(Qword)),
		(0x11, 0xf3, "movss", Some(Dword)),
		(0x11, 0x66, "movupd", Some(XmmWord)),
		(0x11, 0xf2, "movsd", Some(Qword)),
		(0x12, 0x66, "movlpd", None),
		(0x12, 0xf2, "movddup", None),
		(0x12, 0xf3, "movsldup", Some(XmmWord)),
		(0x13, 0x66, "movlpd", None),
		(0x14, 0x66, "unpcklpd", None),
		(0x15, 0x66, "unpckhpd", None),
		(0x16, 0x66, "movhpd", None),
		(0x16, 0xf3, "movshdup", Some(XmmWord)),
		(0x17, 0x66, "movhpd", None),
		(0x28, 0x66, "movapd", None),
		(0x29, 0x66, "movapd", None),
		(0x2a, 0x66, "cvtpi2pd", None),
		(0x2a, 0xf3, "cvtsi2ss", None),
		(0x2b, 0x66, "movntpd", None),
		(0x2c, 0x66, "cvttpd2pi", Some(XmmWord)),
		(0x2c, 0xf3, "cvttss2si", Some(Dword)),
		(0x2d, 0x66, "cvtpd2pi", Some(XmmWord)),
		(0x2d, 0xf3, "cvtss2si", Some(Dword)),
		(0x2e, 0x66, "ucomisd", Some(Qword)),
		(0x2f, 0x66, "comisd", Some(Qword))
	];

	// Addressing mode causing mnemonic variations
	pub const SSE_ADDRESSING_MNEMONIC_VARIATION: &[(u8, AddressingMode /*If not*/, &'static str)] = 
	&[
		(0x12, RegisterAddressingMode, "movlps"),
		(0x16, RegisterAddressingMode, "movhps")
	];
}	