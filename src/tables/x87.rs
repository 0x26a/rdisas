use crate::tables::{Size, Size::*};
pub const FPU_EXTB_MONO: &[([u8;2], &'static str)] = 
&[
	([0xd8,0xd1],"fcom"),
	([0xd8,0xd9],"fcomp"),
	([0xd9,0xd0],"fnop"),
	([0xd9,0xe0],"fchs"),
	([0xd9,0xe1],"fabs"),                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
	([0xd9,0xe4],"ftst"),   
	([0xd9,0xe5],"fxam"),   
	([0xd9,0xe8],"fld1"),   
	([0xd9,0xe9],"fldl2t"), 
	([0xd9,0xea],"fldl2e"), 
	([0xd9,0xeb],"fldpi"),  
	([0xd9,0xec],"fldlg2"), 
	([0xd9,0xed],"fldln2"), 
	([0xd9,0xee],"fldz"),   
	([0xd9,0xf0],"f2xm1"),  
	([0xd9,0xf1],"fyl2x"),  
	([0xd9,0xf2],"fptan"),  
	([0xd9,0xf3],"fpatan"), 
	([0xd9,0xf4],"fxtract"),
	([0xd9,0xf5],"fprem1"), 
	([0xd9,0xf6],"fdecstp"),
	([0xd9,0xf7],"fincstp"),
	([0xd9,0xf8],"fprem"),  
	([0xd9,0xf9],"fyl2xp1"),
	([0xd9,0xfa],"fsqrt"),  
	([0xd9,0xfb],"fsincos"),
	([0xd9,0xfc],"frndint"),
	([0xd9,0xfd],"fscale"), 
	([0xd9,0xfe],"fsin"),   
	([0xd9,0xff],"fcos"),   
	([0xda,0xe9],"fucompp"),
	([0xdb,0xe0],"fneni"),  
	([0xdb,0xe1],"fndisi"), 
	([0xdb,0xe2],"fclex"),  
	([0xdb,0xe3],"finit"),  
	([0xdb,0xe4],"fnsetpm"),
	([0xdd,0xe1],"fucom"),
	([0xdd,0xe9],"fucomp"),
	([0xde,0xc1],"faddp"),
	([0xde,0xc9],"fmulp"),
	([0xde,0xd9],"fcompp"),
	([0xde,0xe1],"fsubrp"),
	([0xde,0xe9],"fsubp"),
	([0xde,0xf1],"fdivrp"),                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
	([0xde,0xf9],"fdivp"),                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
	([0xdf,0xe0],"fstsw")                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 
];
pub const FPU_EXTB_MODRM: &[(u8,u8,&'static str,Size)] = 
&[
	(0xd8,0,"fadd",Dword),
	(0xd8,1,"fmul",Dword),
	(0xd8,2,"fcom",Dword),
	(0xd8,3,"fcomp",Dword),
	(0xd8,4,"fsub",Dword),
	(0xd8,5,"fsubr",Dword),
	(0xd8,6,"fdiv",Dword),
	(0xd8,7,"fdivr",Dword),
	(0xd9,0,"fld",Dword),
	(0xd9,1,"fxch",Dword),
	(0xd9,2,"fst",Dword),
	(0xd9,3,"fstp",Dword),
	(0xd9,4,"fldenv",Byte28),
	(0xd9,5,"fldcw",Word),
	(0xd9,6,"fnstenv",Byte28),
	(0xd9,7,"fnstcw",Word),
	(0xda,0,"fiadd",Dword),
	(0xda,1,"fimul",Dword),
	(0xda,2,"ficom",Dword),
	(0xda,3,"ficomp",Dword),
	(0xda,4,"fisub",Dword),
	(0xda,5,"fisubr",Dword),
	(0xda,6,"fidiv",Dword),
	(0xda,7,"fidivr",Dword),
	(0xdb,0,"fild",Dword),
	(0xdb,1,"fisttp",Dword),
	(0xdb,2,"fist",Dword),
	(0xdb,3,"fistp",Dword),
	(0xdb,5,"fld",Tbyte),
	(0xdb,6,"fcomi",Dword),
	(0xdb,7,"fstp",Tbyte),
	(0xdc,0,"fadd",Qword),
	(0xdc,1,"fmul",Qword),
	(0xdc,2,"fcom",Qword),
	(0xdc,3,"fcomp",Qword),
	(0xdc,4,"fsub",Qword),
	(0xdc,5,"fsubr",Qword),
	(0xdc,6,"fdiv",Qword),
	(0xdc,7,"fdivr",Qword),
	(0xdd,0,"fld",Qword),
	(0xdd,1,"fisttp",Qword),
	(0xdd,2,"fst",Qword),
	(0xdd,3,"fstp",Qword),
	(0xdd,4,"frstor",Nop),
	(0xdd,5,"fucomp",Qword),
	(0xdd,6,"fnsave",Nop),
	(0xdd,7,"fnstsw",Word),
	(0xde,0,"fiadd",Word),
	(0xde,1,"fimul",Word),
	(0xde,2,"ficom",Word),
	(0xde,3,"ficomp",Word),
	(0xde,4,"fisub",Word),
	(0xde,5,"fisubr",Word),
	(0xde,6,"fidiv",Word),
	(0xde,7,"fidivr",Word),
	(0xdf,0,"fild",Word),
	(0xdf,1,"fisttp",Word),
	(0xdf,2,"fist",Word),
	(0xdf,3,"fistp",Word),
	(0xdf,4,"fbld",Tbyte),
	(0xdf,5,"fild",Qword),
	(0xdf,6,"fbstp",Tbyte),
	(0xdf,7,"fistp",Qword)
];
pub const ALU_FPU_REG: [([&'static str;3],&'static str);8] = 
[
	(["eax","ax","al"],"st(0)"),
	(["ecx","cx","cl"],"st(1)"),
	(["edx","dx","dl"],"st(2)"),
	(["ebx","bx","bl"],"st(3)"),
	(["esp","sp","ah"],"st(4)"),
	(["ebp","bp","ch"],"st(5)"),
	(["esi","si","dh"],"st(6)"),
	(["edi","di","bh"],"st(7)")
];

pub const PROC_EXCEPTION: &[([u8;2],[&'static str;2])] = 
&[
	([0xda, 0], ["fiadd", "fcmovb"]),
	([0xda, 1], ["fimul", "fcmove"]),
	([0xda, 2], ["ficom", "fcmovbe"]),
	([0xda, 3], ["ficomp", "fcmovu"]),
	([0xdb, 0], ["fild", "fcmovnb"]),
	([0xdb, 1], ["fisttp", "fcmovne"]),
	([0xdb, 2], ["fist", "fcmovnbe"]),
	([0xdb, 3], ["fistp", "fcmovnu"]),
	([0xdb, 5], ["fld", "fucomi"]),
	([0xdc, 2], ["fcom", "fcom2"]),
	([0xdc, 3], ["fcomp", "fcomp3"]),
	([0xdc, 4], ["fsub", "fsubr"]),
	([0xdc, 5], ["fsubr", "fsub"]),
	([0xdc, 6], ["fdiv", "fdivr"]),
	([0xdc, 7], ["fdivr", "fdiv"]),
	([0xdd, 1], ["fisttp", "fxch4"]),
	([0xdd, 4], ["frstor", "fucom"]),
	([0xde, 0], ["fiadd", "faddp"]),
	([0xde, 1], ["fimul", "fmulp"]),
	([0xde, 2], ["ficom", "fcomp5"]),
	([0xde, 4], ["fisub", "fsubrp"]),
	([0xde, 5], ["fisubr", "fsubp"]),
	([0xde, 6], ["fidiv", "fdivrp"]),
	([0xde, 7], ["fidivr", "fdivp"]),
	([0xdf, 0], ["fild", "ffreep"]),
	([0xdf, 1], ["fisttp", "fxch7"]),
	([0xdf, 2], ["fist", "fstp8"]),
	([0xdf, 3], ["fistp", "fstp9"]),
	([0xdf, 5], ["fild", "fucomip"]),
	([0xdf, 6], ["fbstp", "fcomip"])
];