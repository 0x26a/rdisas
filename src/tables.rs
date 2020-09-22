pub mod x87;
pub mod mono;
pub mod plri;
pub mod mdrm;
pub mod extm;
pub mod simd;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Size{
	Byte28,
	Word,
	Dword,
	Qword,
	Tbyte,
	XmmWord,
	Nop
}

pub const REG_MEM: u8 = 1;
pub const MEM_REG: u8 = 0;

pub const X8: u8 = 0;
pub const X16_32: u8 = 1;

pub const REG_TABLE: [[&'static str;3];8] = 
[
	["eax","ax","al"],
	["ecx","cx","cl"],
	["edx","dx","dl"],
	["ebx","bx","bl"],
	["esp","sp","ah"],
	["ebp","bp","ch"],
	["esi","si","dh"],
	["edi","di","bh"]
];