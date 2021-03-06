pub const MONOBYTE: &[(u8, [&'static str;2])] = 
&[
	(0x06, ["push es","pushw es"]),
	(0x07, ["pop es","popw es"]),
	(0x0e, ["push cs","pushw cs"]),
	(0x16, ["push ss","pushw ss"]),
	(0x17, ["pop ss","popw ss"]),
	(0x1e, ["push ds","pushw ds"]),
	(0x1f, ["pop ds","popw ds"]),
	(0x27, ["daa";2]),
	(0x2f, ["das";2]),
	(0x37, ["aaa";2]),
	(0x3f, ["aas";2]),
	(0x40, ["inc eax","inc ax"]),
	(0x41, ["inc ecx","inc cx"]),
	(0x42, ["inc edx","inc dx"]),
	(0x43, ["inc ebx","inc bx"]),
	(0x44, ["inc esp","inc sp"]),
	(0x45, ["inc ebp","inc bp"]),
	(0x46, ["inc esi","inc si"]),
	(0x47, ["inc edi","inc di"]),
	(0x48, ["dec eax","dec ax"]),
	(0x49, ["dec ecx","dec cx"]),
	(0x4a, ["dec edx","dec dx"]),
	(0x4b, ["dec ebx","dec bx"]),
	(0x4c, ["dec esp","dec sp"]),
	(0x4d, ["dec ebp","dec bp"]),
	(0x4e, ["dec esi","dec si"]),
	(0x4f, ["dec edi","dec di"]),
	(0x50, ["push eax","push ax"]),
	(0x51, ["push ecx","push cx"]),
	(0x52, ["push edx","push dx"]),
	(0x53, ["push ebx","push bx"]),
	(0x54, ["push esp","push sp"]),
	(0x55, ["push ebp","push bp"]),
	(0x56, ["push esi","push si"]),
	(0x57, ["push edi","push di"]),
	(0x58, ["pop eax","pop ax"]),
	(0x59, ["pop ecx","pop cx"]),
	(0x5a, ["pop edx","pop dx"]),
	(0x5b, ["pop ebx","pop bx"]),
	(0x5c, ["pop esp","pop sp"]),
	(0x5d, ["pop ebp","pop bp"]),
	(0x5e, ["pop esi","pop si"]),
	(0x5f, ["pop edi","pop di"]),
	(0x60, ["pusha","pushaw"]),
	(0x61, ["popa","popaw"]),
	(0x6c, ["insb";2]),
	(0x6d, ["insw";2]),
	(0x6e, ["outsb";2]),
	(0x6f, ["outsw";2]),
	(0x90, ["nop","nop"]),
	(0x91, ["xchg eax, ecx","xchg ax, cx"]),
	(0x92, ["xchg eax, edx","xchg ax, dx"]),
	(0x93, ["xchg eax, ebx","xchg ax, bx"]),
	(0x94, ["xchg eax, esp","xchg ax, sp"]),
	(0x95, ["xchg eax, ebp","xchg ax, bp"]),
	(0x96, ["xchg eax, esi","xchg ax, si"]),
	(0x97, ["xchg eax, edi","xchg ax, di"]),
	(0x98, ["cwde","cbw"]),
	(0x99, ["cdq","cwd"]),
	(0x9b, ["fwait","fwait"]),
	(0x9c, ["pushf","pushfw"]),
	(0x9d, ["popf","popfw"]),
	(0x9e, ["sahf","sahf"]),
	(0x9f, ["lahf","lahf"]),
	(0xa4, ["movsb";2]),
	(0xa5, ["movsw";2]),
	(0xa6, ["cmpsb";2]),
	(0xa7, ["cmpsw";2]),
	(0xaa, ["stosb";2]),
	(0xab, ["stosw";2]),
	(0xac, ["lodsb";2]),
	(0xad, ["lodsw";2]),
	(0xae, ["scasb";2]),
	(0xaf, ["scasw";2]),
	(0xc3, ["ret","retw"]),
	(0xc9, ["leave","leavew"]),
	(0xcb, ["retf","retfw"]),
	(0xcc, ["int3";2]),
	(0xce, ["into";2]),
	(0xcf, ["iret","iretw"]),
	(0xd7, ["xlat";2]),
	(0xec, ["in al, dx","in al, dx"]),
	(0xed, ["in eax, dx","in ax, dx"]),
	(0xee, ["out dx, al","out dx, al"]),
	(0xef, ["out dx, eax","out dx, ax"]),
	(0xf4, ["hlt";2]),
	(0xf5, ["cmc";2]),
	(0xf8, ["clc";2]),
	(0xf9, ["stc";2]),
	(0xfa, ["cli";2]),
	(0xfb, ["sti";2]),
	(0xfc, ["cld";2]),
	(0xfd, ["std";2])
];

pub const MONOBYTE_0F: &[(u8, [&'static str;2])] = 
&[
	(0x06, ["clts";2]),
	(0x08, ["invd";2]),
	(0x09, ["wbinvd";2]),
	(0x0b, ["(bad)";2]),
	(0x30, ["wrmsr";2]),
	(0x31, ["rdtsc";2]),
	(0x32, ["rdmsr";2]),
	(0x33, ["rdpmc";2]),
	(0x34, ["sysenter";2]),
	(0x35, ["sysexit";2]),
	(0x37, ["getsec";2]),
	(0xa0, ["push fs","pushw fs"]),
	(0xa1, ["pop fs","popw fs"]),
	(0xa2, ["cpuid";2]),
	(0xa8, ["push gs","pushw gs"]),
	(0xa9, ["pop gs","popw gs"]),
	(0xaa, ["rsm";2]),
	(0xb9, ["(bad)";2])

];