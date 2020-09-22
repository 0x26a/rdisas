use crate::tables::*;

pub const EXTMODRMBYTE: &[(u8, &[(&'static str, bool)])] = 
&[
	(0x80, &[("add", false),("or",false),("adc", false),("sbb", false),("and", false),("sub", false),("xor", false),("cmp", false)]),
	(0x81, &[("add", false),("or",false),("adc", false),("sbb", false),("and", false),("sub", false),("xor", false),("cmp", false)]),
	(0x82, &[("add", false),("or", false),("adc", false),("sbb", false),("and", false),("sub", false),("xor", false),("cmp", false)]),
	(0x83, &[("add", false),("or", false),("adc", false),("sbb", false),("and", false),("sub", false),("xor", false),("cmp", false)]),
	(0x8f, &[("pop", true),("pop", true),("pop", true),("pop", true),("pop", true),("pop", true),("pop", true),("pop", true)]),
	(0xc0, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xc1, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xc6, &[("mov", false)]),
	(0xc7, &[("mov", false)]),
	(0xd0, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xd1, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xd2, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xd3, &[("rol", true),("ror", true),("rcl", true),("rcr", true),("shl", true),("shr", true),("sal", true),("sar", true)]),
	(0xf6, &[("test", false),("test", false),("not", true),("neg", true),("mul", true),("imul", true),("div", true),("idiv", true),]),
	(0xf7, &[("test", false),("test", false),("not", true),("neg", true),("mul", true),("imul", true),("div", true),("idiv", true),]),
	(0xfe, &[("inc", true), ("dec",  true)]),
	(0xff, &[("inc", true),("dec", true),("call", true),("callf", true),("jmp", true),("jmpf", true),("push", true)])
];

pub const EXTMODRMBYTE_0F: &[(u8, u8, &'static str, Option<u8>, Option<u8>, bool, bool)] = 
&[
//  op  ext str     d             s             imm    one-o
	(0, 0, "sldt", Some(REG_MEM), Some(X16_32), false, true),
	(0, 1, "str", Some(REG_MEM), Some(X16_32), false, true),
	(0, 2, "lldt", Some(REG_MEM), Some(X16_32), false, true),
	(0, 3, "ltr", Some(REG_MEM), Some(X16_32), false, true),
	(0, 4, "verr", Some(MEM_REG), Some(X16_32), false, true),
	(0, 5, "verw", Some(MEM_REG), Some(X16_32), false, true),
	(1, 0, "sgdt", Some(MEM_REG), Some(X16_32), false, true),
	(1, 1, "sidt", Some(MEM_REG), Some(X16_32), false, true),
	(1, 2, "lgdt", Some(MEM_REG), Some(X16_32), false, true),
	(1, 3, "lidt", Some(MEM_REG), Some(X16_32), false, true),
	(1, 4, "smsw", Some(REG_MEM), Some(X16_32), false, true),
	(1, 6, "lmsw", Some(REG_MEM), Some(X16_32), false, true),
	(1, 7, "invlpg", Some(MEM_REG), Some(X16_32), false, true),
	(0xae, 0, "fxsave", Some(MEM_REG), Some(X16_32), false, true),
	(0xae, 1, "fxrstor", Some(MEM_REG), Some(X16_32), false, true),
	(0xae, 4, "xsave", Some(MEM_REG), Some(X16_32), false, true),
	(0xae, 5, "xrstor", Some(MEM_REG), Some(X16_32), false, true),
	(0xba, 4, "bt", Some(MEM_REG), Some(X16_32), false, true),
	(0xba, 5, "bts", Some(MEM_REG), Some(X16_32), false, true),
	(0xba, 6, "btr", Some(MEM_REG), Some(X16_32), false, true),
	(0xba, 7, "btc", Some(MEM_REG), Some(X16_32), false, true),
];