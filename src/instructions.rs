enum_from_primitive!{
    #[derive(Debug)]
    pub enum Instruction {
            // BRK = 0x00,

            // Stack    
            PHP = 0x08,
            PLP = 0x28,
            PHA = 0x48,
            PLA = 0x68,
            TXS = 0x9a,
            TSX = 0xba,

            // Branch   
            BPL = 0x10,
            BMI = 0x30,
            BVC = 0x50,
            BVS = 0x70,
            BCC = 0x90,
            BCS = 0xb0,
            BNE = 0xd0,
            BEQ = 0xf0,

            // Flag instructions
            CLC = 0x18,
            SEC = 0x38,
            // CLI = 0x58,
            SEI = 0x78,
            CLV = 0xb8,
            CLD = 0xd8,
            SED = 0xf8,

            // Register instructions
            DEY = 0x88,
            DEX = 0xca,
            INX = 0xe8,
            INY = 0xc8,
            TAX = 0xaa,
            TXA = 0x8a,
            TAY = 0xa8,
            TYA = 0x98,

            // Compares
            CPYImm = 0xc0,
            // CPY_z_pg = 0xc4,
            // CPY_abs = 0xcc,
            CPXImm = 0xe0,
            // CPX_z_pg = 0xe4,
            // CPX_abs = 0xec,

            // Loads
            LDAInxX = 0xa1,
            LDAZpg = 0xa5,
            LDAImm = 0xa9,
            LDAAbs = 0xad,
            LDAIndY = 0xb1,
            // LDAZPgX = 0xb5,
            // LDA_ax = 0xbd,
            // LDA_ay = 0xb9,

            LDXImm = 0xa2,
            LDXZpg = 0xa6,
            LDXAbs = 0xae,
            // LDXZPgY = 0xb6,
            // LDX_ay = 0xbe,

            LDYImm  = 0xa0,
            // LDY_z_pg = 0xa4,
            // LDY_abs = 0xac,
            // LDYZPgX = 0xb4,
            // LDY_ax = 0xbc,

            // Stores
            // STA_inx_x = 0x81,
            STAZpg = 0x85,
            STAAbs  = 0x8d,
            STAIndY = 0x91,
            // STAZPgX = 0x95,
            // STA_ax = 0x9d,
            // STA_ay = 0x99,

            STXZpg = 0x86,
            STXAbs  = 0x8e,
            // STXZPgY = 0x96,

            STYZpg = 0x84,
            // STY_abs = 0x8c,
            // STYZPgX = 0x94,

            // Jumps
            JSRAbs  = 0x20,
            JMPAbs  = 0x4c,
            // JMP_ind = 0x6c,

            RTI = 0x40,
            RTS = 0x60,

            // Bit tests
            BITZpg = 0x24,
            // BIT_abs = 0x2c,

            // ALU operations
            // ORA_inx_x = 0x01,
            // ORA_z_pg = 0x05,
            ORAImm  = 0x09,
            // ORA_abs = 0x0d,
            // ORA_ind_y = 0x11,
            // ORAZPgX = 0x15,
            // ORA_ax = 0x19,
            // ORA_ay = 0x1d,

            // AND_inx_x = 0x21,
            // AND_z_pg = 0x25,
            ANDImm  = 0x29,
            // AND_abs = 0x2d,
            // AND_ind_y = 0x31,
            ANDZPgX = 0x35,
            // AND_ax = 0x39,
            // AND_ay = 0x3d,

            // EOR_inx_x = 0x41,
            // EOR_z_pg = 0x45,
            EORImm = 0x49,
            // EOR_abs = 0x4d,
            // EOR_ind_y = 0x51,
            // EORZPgX = 0x55,
            // EOR_ax = 0x59,
            // EOR_ay = 0x5d,

            // ADC_inx_x = 0x61, 
            // ADC_z_pg  = 0x65, 
            ADCImm  = 0x69,
            // ADC_abs = 0x6d,
            // ADC_ind_y = 0x71, 
            // ADCZPgX = 0x75, 
            // ADC_ax = 0x79,
            // ADC_ay = 0x7d,

            // CMP_inx_x = 0xc1, 
            // CMP_z_pg  = 0xc5, 
            CMPImm = 0xc9,
            // CMP_abs = 0xcd,
            // CMP_ind_y = 0xd1, 
            // CMPZPgX = 0xd5, 
            // CMP_ax = 0xd9,
            // CMP_ay = 0xdd,

            // SBC_inx_x = 0xe1, 
            // SBC_z_pg  = 0xe5, 
            SBCImm = 0xe9,
            // SBC_abs = 0xed,
            // SBC_ind_y = 0xf1, 
            // SBCZPgX = 0xf5, 
            // SBC_ax = 0xf9,
            // SBC_ay = 0xfd,

            // ASL_z_pg  = 0x06, 
            ASL = 0x0a,
            // ASL_abs = 0x0e,
            // ASLZPgX = 0x16, 
            // ASL_ax = 0x1e,

            // LSR_z_pg  = 0x46, 
            LSR = 0x4a,
            // LSR_abs = 0x4e,
            // LSRZPgX = 0x56, 
            // LSR_ax = 0x5e,

            // Rotates
            // ROL_z_pg  = 0x26, 
            ROL = 0x2a,
            // ROL_abs = 0x2e,
            // ROLZPgX = 0x36, 
            // ROL_ax = 0x3e,

            // ROR_z_pg  = 0x66, 
            ROR = 0x6a,
            // ROR_abs = 0x6e,
            // RORZPgX = 0x76, 
            // ROR_ax = 0x7e,

            // Increments
            DECZpg = 0xc6,
            // DEC_abs = 0xce,
            // DECZPgX = 0xd6, 
            // DEC_ax = 0xde,

            // INC_z_pg  = 0xe6, 
            // INC_abs = 0xee,
            // INCZPgX = 0xf6, 
            // INC_ax = 0xfe,

            // The ever important nop
            // Observe all its majesty
            NOP = 0xea,
    }
}
