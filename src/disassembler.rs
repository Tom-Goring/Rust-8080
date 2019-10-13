use crate::i8080::cpu::Byte;
use crate::i8080::cpu::Address;
use crate::i8080::memory::Memory;

pub fn disassemble_8080_op(program: &Memory, pc: Address) -> Byte {

    let op_code: Byte = program[pc];
    let num_bytes;

    println!("{:04X}  ", pc);

    match op_code {
        // 00
        0x00 => {  num_bytes = 1 },
        0x01 => { println!("LXI B, ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x02 => { println!("STAX B"); num_bytes = 1 },
        0x03 => { println!("INX B"); num_bytes = 1 },
        0x04 => { println!("INR B"); num_bytes = 1 },
        0x05 => { println!("DCR B"); num_bytes = 1 },
        0x06 => { println!("MVI B, #${:02X}", program[(pc + 1)]); num_bytes = 2; },
        0x07 => { println!("RLC"); num_bytes = 1 },

        // 08
        0x08 => {  num_bytes = 1 },
        0x09 => { println!("DAD B"); num_bytes = 1 },
        0x0a => { println!("LDAX B"); num_bytes = 1 },
        0x0b => { println!("DCX B"); num_bytes = 1 },
        0x0c => { println!("INR C"); num_bytes = 1 },
        0x0d => { println!("DCR C"); num_bytes = 1 },
        0x0e => { println!("MVI C, #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x0f => { println!("RRC"); num_bytes = 1 },

        // 10
        0x10 => {  num_bytes = 1 },
        0x11 => { println!("LXI D, ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x12 => { println!("STAX D"); num_bytes = 1 },
        0x13 => { println!("INX D"); num_bytes = 1 },
        0x14 => { println!("INR D"); num_bytes = 1 },
        0x15 => { println!("DCR D"); num_bytes = 1 },
        0x16 => { println!("MVI D #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x17 => { println!("RAL"); num_bytes = 1 },

        // 18
        0x18 => {  num_bytes = 1 },
        0x19 => { println!("DAD D"); num_bytes = 1 },
        0x1a => { println!("LDAX D"); num_bytes = 1 },
        0x1b => { println!("DCX D"); num_bytes = 1 },
        0x1c => { println!("INR E"); num_bytes = 1 },
        0x1d => { println!("DCR E"); num_bytes = 1 },
        0x1e => { println!("MVI E #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x1f => { println!("RAR"); num_bytes = 1 },

        // 20
        0x20 => {  num_bytes = 1 },
        0x21 => { println!("LXI H, ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x22 => { println!("SHLD ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x23 => { println!("INX H"); num_bytes = 1 },
        0x24 => { println!("INR H"); num_bytes = 1 },
        0x25 => { println!("DCR H"); num_bytes = 1 },
        0x26 => { println!("MVI H #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x27 => { println!("DDA"); num_bytes = 1 },

        // 28
        0x28 => {  num_bytes = 1 },
        0x29 => { println!("DAD H"); num_bytes = 1 },
        0x2a => { println!("LHLD ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x2b => { println!("DCX H"); num_bytes = 1 },
        0x2c => { println!("INR L"); num_bytes = 1 },
        0x2d => { println!("DCR L"); num_bytes = 1 },
        0x2e => { println!("MVI L #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x2f => { println!("CMA"); num_bytes = 1 },

        // 30
        0x30 => {  num_bytes = 1 },
        0x31 => { println!("LXI SP, ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x32 => { println!("STA ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x33 => { println!("INX SP"); num_bytes = 1 },
        0x34 => { println!("INR M"); num_bytes = 1 },
        0x35 => { println!("DCR M"); num_bytes = 1 },
        0x36 => { println!("MVI M, #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x37 => { println!("STC"); num_bytes = 1 },

        // 38
        0x38 => {  num_bytes = 1 },
        0x39 => { println!("DAD SP"); num_bytes = 1 },
        0x3a => { println!("LDA ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0x3b => { println!("DCX SP"); num_bytes = 1 },
        0x3c => { println!("INR A"); num_bytes = 1 },
        0x3d => { println!("DCR A"); num_bytes = 1 },
        0x3e => { println!("MVI A, #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0x3f => { println!("CMC"); num_bytes = 1 },

        // 40
        0x40 => { println!("MOV B,B"); num_bytes = 1 },
        0x41 => { println!("MOV B,C"); num_bytes = 1 },
        0x42 => { println!("MOV B,D"); num_bytes = 1 },
        0x43 => { println!("MOV B,E"); num_bytes = 1 },
        0x44 => { println!("MOV B,H"); num_bytes = 1 },
        0x45 => { println!("MOV B,L"); num_bytes = 1 },
        0x46 => { println!("MOV B,M"); num_bytes = 1 },
        0x47 => { println!("MOV B,A"); num_bytes = 1 },

        // 48
        0x48 => { println!("MOV C,B"); num_bytes = 1 },
        0x49 => { println!("MOV C,C"); num_bytes = 1 },
        0x4a => { println!("MOV C,D"); num_bytes = 1 },
        0x4b => { println!("MOV C,E"); num_bytes = 1 },
        0x4c => { println!("MOV C,H"); num_bytes = 1 },
        0x4d => { println!("MOV C,L"); num_bytes = 1 },
        0x4e => { println!("MOV C,M"); num_bytes = 1 },
        0x4f => { println!("MOV C,A"); num_bytes = 1 },

        // 50
        0x50 => { println!("MOV D,B"); num_bytes = 1 },
        0x51 => { println!("MOV D,C"); num_bytes = 1 },
        0x52 => { println!("MOV D,D"); num_bytes = 1 },
        0x53 => { println!("MOV D,E"); num_bytes = 1 },
        0x54 => { println!("MOV D,H"); num_bytes = 1 },
        0x55 => { println!("MOV D,L"); num_bytes = 1 },
        0x56 => { println!("MOV D,M"); num_bytes = 1 },
        0x57 => { println!("MOV D,A"); num_bytes = 1 },

        // 58
        0x58 => { println!("MOV E,B"); num_bytes = 1 },
        0x59 => { println!("MOV E,C"); num_bytes = 1 },
        0x5a => { println!("MOV E,D"); num_bytes = 1 },
        0x5b => { println!("MOV E,E"); num_bytes = 1 },
        0x5c => { println!("MOV E,H"); num_bytes = 1 },
        0x5d => { println!("MOV E,L"); num_bytes = 1 },
        0x5e => { println!("MOV E,M"); num_bytes = 1 },
        0x5f => { println!("MOV E,A"); num_bytes = 1 },

        // 60
        0x60 => { println!("MOV H,B"); num_bytes = 1 },
        0x61 => { println!("MOV H,C"); num_bytes = 1 },
        0x62 => { println!("MOV H,D"); num_bytes = 1 },
        0x63 => { println!("MOV H,E"); num_bytes = 1 },
        0x64 => { println!("MOV H,H"); num_bytes = 1 },
        0x65 => { println!("MOV H,L"); num_bytes = 1 },
        0x66 => { println!("MOV H,M"); num_bytes = 1 },
        0x67 => { println!("MOV H,A"); num_bytes = 1 },

        // 68
        0x68 => { println!("MOV L,B"); num_bytes = 1 },
        0x69 => { println!("MOV L,C"); num_bytes = 1 },
        0x6a => { println!("MOV L,D"); num_bytes = 1 },
        0x6b => { println!("MOV L,E"); num_bytes = 1 },
        0x6c => { println!("MOV L,H"); num_bytes = 1 },
        0x6d => { println!("MOV L,L"); num_bytes = 1 },
        0x6e => { println!("MOV L,M"); num_bytes = 1 },
        0x6f => { println!("MOV L,A"); num_bytes = 1 },

        // 70
        0x70 => { println!("MOV M,B"); num_bytes = 1 },
        0x71 => { println!("MOV M,C"); num_bytes = 1 },
        0x72 => { println!("MOV M,D"); num_bytes = 1 },
        0x73 => { println!("MOV M,E"); num_bytes = 1 },
        0x74 => { println!("MOV M,H"); num_bytes = 1 },
        0x75 => { println!("MOV M,L"); num_bytes = 1 },
        0x76 => { println!("HLT"); num_bytes = 1 },
        0x77 => { println!("MOV M,A"); num_bytes = 1 },

        // 78
        0x78 => { println!("MOV A,B"); num_bytes = 1 },
        0x79 => { println!("MOV A,C"); num_bytes = 1 },
        0x7a => { println!("MOV A,D"); num_bytes = 1 },
        0x7b => { println!("MOV A,E"); num_bytes = 1 },
        0x7c => { println!("MOV A,H"); num_bytes = 1 },
        0x7d => { println!("MOV A,L"); num_bytes = 1 },
        0x7e => { println!("MOV A,M"); num_bytes = 1 },
        0x7f => { println!("MOV A,A"); num_bytes = 1 },

        // 80
        0x80 => { println!("ADD B"); num_bytes = 1 },
        0x81 => { println!("ADD C"); num_bytes = 1 },
        0x82 => { println!("ADD D"); num_bytes = 1 },
        0x83 => { println!("ADD E"); num_bytes = 1 },
        0x84 => { println!("ADD H"); num_bytes = 1 },
        0x85 => { println!("ADD L"); num_bytes = 1 },
        0x86 => { println!("ADD M"); num_bytes = 1 },
        0x87 => { println!("ADD A"); num_bytes = 1 },

        // 88
        0x88 => { println!("ADC B"); num_bytes = 1 },
        0x89 => { println!("ADC C"); num_bytes = 1 },
        0x8a => { println!("ADC D"); num_bytes = 1 },
        0x8b => { println!("ADC E"); num_bytes = 1 },
        0x8c => { println!("ADC H"); num_bytes = 1 },
        0x8d => { println!("ADC L"); num_bytes = 1 },
        0x8e => { println!("ADC M"); num_bytes = 1 },
        0x8f => { println!("ADC A"); num_bytes = 1 },

        // 90
        0x90 => { println!("SUB B"); num_bytes = 1 },
        0x91 => { println!("SUB C"); num_bytes = 1 },
        0x92 => { println!("SUB D"); num_bytes = 1 },
        0x93 => { println!("SUB E"); num_bytes = 1 },
        0x94 => { println!("SUB H"); num_bytes = 1 },
        0x95 => { println!("SUB L"); num_bytes = 1 },
        0x96 => { println!("SUB M"); num_bytes = 1 },
        0x97 => { println!("SUB A"); num_bytes = 1 },

        // 98
        0x98 => { println!("SBB B"); num_bytes = 1 },
        0x99 => { println!("SBB C"); num_bytes = 1 },
        0x9a => { println!("SBB D"); num_bytes = 1 },
        0x9b => { println!("SBB E"); num_bytes = 1 },
        0x9c => { println!("SBB H"); num_bytes = 1 },
        0x9d => { println!("SBB L"); num_bytes = 1 },
        0x9e => { println!("SBB M"); num_bytes = 1 },
        0x9f => { println!("SBB A"); num_bytes = 1 },

        // a0
        0xa0 => { println!("ANA B"); num_bytes = 1 },
        0xa1 => { println!("ANA C"); num_bytes = 1 },
        0xa2 => { println!("ANA D"); num_bytes = 1 },
        0xa3 => { println!("ANA E"); num_bytes = 1 },
        0xa4 => { println!("ANA H"); num_bytes = 1 },
        0xa5 => { println!("ANA L"); num_bytes = 1 },
        0xa6 => { println!("ANA M"); num_bytes = 1 },
        0xa7 => { println!("ANA A"); num_bytes = 1 },

        // a8
        0xa8 => { println!("XRA B"); num_bytes = 1 },
        0xa9 => { println!("XRA C"); num_bytes = 1 },
        0xaa => { println!("XRA D"); num_bytes = 1 },
        0xab => { println!("XRA E"); num_bytes = 1 },
        0xac => { println!("XRA H"); num_bytes = 1 },
        0xad => { println!("XRA L"); num_bytes = 1 },
        0xae => { println!("XRA M"); num_bytes = 1 },
        0xaf => { println!("XRA A"); num_bytes = 1 },

        // b0
        0xb0 => { println!("ORA B"); num_bytes = 1 },
        0xb1 => { println!("ORA C"); num_bytes = 1 },
        0xb2 => { println!("ORA D"); num_bytes = 1 },
        0xb3 => { println!("ORA E"); num_bytes = 1 },
        0xb4 => { println!("ORA H"); num_bytes = 1 },
        0xb5 => { println!("ORA L"); num_bytes = 1 },
        0xb6 => { println!("ORA M"); num_bytes = 1 },
        0xb7 => { println!("ORA A"); num_bytes = 1 },

        // b8
        0xb8 => { println!("CMP B"); num_bytes = 1 },
        0xb9 => { println!("CMP C"); num_bytes = 1 },
        0xba => { println!("CMP D"); num_bytes = 1 },
        0xbb => { println!("CMP E"); num_bytes = 1 },
        0xbc => { println!("CMP H"); num_bytes = 1 },
        0xbd => { println!("CMP L"); num_bytes = 1 },
        0xbe => { println!("CMP M"); num_bytes = 1 },
        0xbf => { println!("CMP A"); num_bytes = 1 },

        // c0
        0xc0 => { println!("RNZ"); num_bytes = 1 },
        0xc1 => { println!("POP B"); num_bytes = 1 },
        0xc2 => { println!("JNZ ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xc3 => { println!("JMP ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xc4 => { println!("CNZ ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xc5 => { println!("PUSH B"); num_bytes = 1 },
        0xc6 => { println!("ADI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xc7 => { println!("RST 0"); num_bytes = 1 },

        // c8
        0xc8 => { println!("RZ"); num_bytes = 1 },
        0xc9 => { println!("RET"); num_bytes = 1 },
        0xca => { println!("JZ ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xcb => { println!("*JMP ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xcc => { println!("CZ ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xcd => { println!("CALL ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xce => { println!("ACI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xcf => { println!("RST 1"); num_bytes = 1 },

        // d0
        0xd0 => { println!("RNC"); num_bytes = 1 },
        0xd1 => { println!("POP D"); num_bytes = 1 },
        0xd2 => { println!("JNC ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xd3 => { println!("OUT #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xd4 => { println!("CNC ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xd5 => { println!("PUSH D"); num_bytes = 1 },
        0xd6 => { println!("SUI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xd7 => { println!("RST 2"); num_bytes = 1 },

        // d8
        0xd8 => { println!("RC"); num_bytes = 1 },
        0xd9 => { println!("*RET"); num_bytes = 1 },
        0xda => { println!("JC ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xdb => { println!("IN #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xdc => { println!("CC ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xdd => { println!("*CALL ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xde => { println!("SBI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xdf => { println!("RST 3"); num_bytes = 1 },

        // e0
        0xe0 => { println!("RPO"); num_bytes = 1 },
        0xe1 => { println!("POP H"); num_bytes = 1 },
        0xe2 => { println!("JPO ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xe3 => { println!("XTHL"); num_bytes = 1 },
        0xe4 => { println!("CPO ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xe5 => { println!("PUSH H"); num_bytes = 1 },
        0xe6 => { println!("ANI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xe7 => { println!("RST 4"); num_bytes = 1 },

        // e8
        0xe8 => { println!("RPE"); num_bytes = 1 },
        0xe9 => { println!("PCHL"); num_bytes = 1 },
        0xea => { println!("JPE ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xeb => { println!("XCHG"); num_bytes = 1 },
        0xec => { println!("CPE ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xed => { println!("*CALL ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xee => { println!("XRI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xef => { println!("RST 5"); num_bytes = 1 },

        // f0
        0xf0 => { println!("RP"); num_bytes = 1 },
        0xf1 => { println!("POP PSW"); num_bytes = 1 },
        0xf2 => { println!("JP ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xf3 => { println!("DI"); num_bytes = 1 },
        0xf4 => { println!("CP ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3 },
        0xf5 => { println!("PUSH PSW"); num_bytes = 1 },
        0xf6 => { println!("ORI #${:02X}", program[(pc + 1)]); num_bytes = 2 },
        0xf7 => { println!("RST 6"); num_bytes = 1 },

        // f8
        0xf8 => {println!("RM"); num_bytes = 1; },
        0xf9 => {println!("SPHL"); num_bytes = 1; },
        0xfa => {println!("JM ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3; },
        0xfb => {println!("EI"); num_bytes = 1; },
        0xfc => {println!("CM ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3; },
        0xfd => {println!("*CALL ${:02X}{:02X}", program[(pc + 2)], program[(pc + 1)]); num_bytes = 3; },
        0xff => {println!("RST 7"); num_bytes = 1; },
        0xfe => {println!("CPI #${:02X}", program[(pc + 1)]); num_bytes = 2; },
        _ => {println!("NOP!"); num_bytes = 0;}
    }

    num_bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i8080;

    #[test]
    fn test_disassembler() {
        let mut cpu = i8080::cpu::CPU::new();

        for x in 0..0xFE {
            cpu.memory[x] = x as u8;
        }

        for x in 0..0xFE {
            disassemble_8080_op(&cpu.memory, x);
        }
    }
}