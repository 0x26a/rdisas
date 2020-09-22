# Rdisas
An Intel x86 disassembler in the form of a Rust library.

## Warnings
The library was only built for learning purpose, and was shared for anyone who may also want to learn from its code.

Therefore, the author of the project doesn't advice anyone to include the following Rust crate in a professional project as it may be unstable.

*Rdisas will probably be published on crates.io once the author will have determined that it has become stable enough.*

*The library is currently in an active state of development.*

## Features
- [x] x86 Instruction Set
- [x] Floating Point Instruction Set 
- [x] MMX
- [x] VMX
- [ ] SSE

## Example

A basic disassembly example:

```rust
extern crate rdisas;
use rdisas::Disassembler;

fn main(){
    // Raw x86 Intel machine code
    let assembly = b"\x5E\x31\xC9\x8A\x04\x0E\x3C\x00\x74\x03\x41\xEB\xF6\x89\xC8\xC3";
    
    let disassembly = Disassembler::new(assembly).disassemble();
    // Returns a vector of 'InstructionResult'
    let disassembly = disassembly.extract();
    
    for item in disassembly.iter(){
        let instruction = item.as_ref()?;
        println!("{}", instruction);
    }
   
}
```
## Output
```
pop esi
xor ecx, ecx
mov al, byte ptr ds:[esi+ecx*1]
cmp al, 0x0
je 0xf
inc ecx
jmp 0x3
mov eax, ecx
ret
```
## More Example

Code to retrieve mnemonic, displacement and immediate from instruction:

```rust
// mov dword ptr ds:[0xaabbccdd], 0x12345678
let assembly = b"\xC7\x05\xDD\xCC\xBB\xAA\x78\x56\x34\x12";
    
let output = Disassembler::new(assembly).disassemble().extract();
let instruction = output[0].as_ref()?;

assert_eq!(instruction.prefixes(), []);
assert_eq!(instruction.mnemonic(), "mov");
assert_eq!(instruction.displacement(), [0xdd, 0xcc, 0xbb, 0xaa]);
assert_eq!(instruction.immediate(), [0x78,0x56,0x34,0x12]);
```

Code to treat output data:


```rust
let assembly = b"\x5E\x31\xC9\x8A\x04\x0E\x3C\x00\x74\x03\x41\xEB\xF6\x89\xC8\xC3";
    
let  output = Disassembler::new(assembly).disassemble().extract();
    
for item in output.iter(){
    let instruction = item.as_ref()?;
    
    println!("The processor interprets \"{}\" as \"{:?}\".", instruction.to_string(), instruction.to_vec());
}
```

Code to check if the disassembler returned an error:

```rust
let assembly = b"\x5E\x31\xC9\x8A\x04\x0E\x3C\x00\x74\x03\x41\xEB\xF6\x89\xC8\xC3";
    
let disassembly = Disassembler::new(assembly).disassemble();
assert!(disassembly.success());
```
