enum_from_primitive!{
    #[derive(Debug)]
    pub enum Instruction {
            // 0x00 => {}, // BRK

            // Stack    
            PHP = 0x08,
            PLP = 0x28,
            PHA = 0x48,
            PLA = 0x68,
            TXS = 0x9a,
            // 0xba => {}, // TSX       

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
            // 0x58 => {}, // CLI      
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
            // 0xa8 => {}, // TAY       
            // 0x98 => {}, // TYA       

            // Compares
            CPYImm = 0xc0,
            // 0xc4 => {}, // CPY_z_pg 
            // 0xcc => {}, // CPY_abs  
            CPXImm = 0xe0,
            // 0xe4 => {}, // CPX_z_pg 
            // 0xec => {}, // CPX_abs  

            // Loads
            // 0xa1 => {}, // LDA_inx_x 
            LDAZpg = 0xa5,
            LDAImm = 0xa9,
            LDAAbs = 0xad,
            LDAIndY = 0xb1,
            // 0xb5 => {}, // LDA_dx    
            // 0xbd => {}, // LDA_ax    
            // 0xb9 => {}, // LDA_ay    

            LDXImm = 0xa2,
            LDXZpg = 0xa6,
            // 0xae => {}, // LDX_abs  
            // 0xb6 => {}, // LDX_dy   
            // 0xbe => {}, // LDX_ay   

            LDYImm  = 0xa0,
            // 0xa4 => {}, // LDY_z_pg 
            // 0xac => {}, // LDY_abs  
            // 0xb4 => {}, // LDY_dx   
            // 0xbc => {}, // LDY_ax   

            // Stores
            // 0x81 => {}, // STA_inx_x
            STAZpg = 0x85,
            STAAbs  = 0x8d,
            STAIndY = 0x91,
            // 0x95 => {}, // STA_dx   
            // 0x9d => {}, // STA_ax   
            // 0x99 => {}, // STA_ay   

            STXZpg = 0x86,
            STXAbs  = 0x8e,
            // 0x96 => {}, // STX_dy   

            STYZpg = 0x84,
            // 0x8c => {}, // STY_abs  
            // 0x94 => {}, // STY_dx   

            // Jumps
            JSRAbs  = 0x20,
            JMPAbs  = 0x4c,
            // 0x6c => {}, // JMP_ind  

            // 0x40 => {}, // RTI      
            RTS = 0x60,

            // Bit tests
            BITZpg = 0x24,
            // 0x2c => {}, // BIT_abs  

            // ALU operations
            // 0x01 => {}, // ORA_inx_x
            // 0x05 => {}, // ORA_z_pg 
            ORAImm  = 0x09,
            // 0x0d => {}, // ORA_abs  
            // 0x11 => {}, // ORA_ind_y
            // 0x15 => {}, // ORA_dx   
            // 0x19 => {}, // ORA_ax   
            // 0x1d => {}, // ORA_ay   

            // 0x21 => {}, // AND_inx_x
            // 0x25 => {}, // AND_z_pg 
            ANDImm  = 0x29,
            // 0x2d => {}, // AND_abs  
            // 0x31 => {}, // AND_ind_y
            // 0x35 => {}, // AND_dx   
            // 0x39 => {}, // AND_ax   
            // 0x3d => {}, // AND_ay   

            // 0x41 => {}, // EOR_inx_x
            // 0x45 => {}, // EOR_z_pg 
            EORImm = 0x49,
            // 0x4d => {}, // EOR_abs  
            // 0x51 => {}, // EOR_ind_y
            // 0x55 => {}, // EOR_dx   
            // 0x59 => {}, // EOR_ax   
            // 0x5d => {}, // EOR_ay   

            // 0x61 => {}, // ADC_inx_x
            // 0x65 => {}, // ADC_z_pg 
            ADCImm  = 0x69,
            // 0x6d => {}, // ADC_abs  
            // 0x71 => {}, // ADC_ind_y
            // 0x75 => {}, // ADC_dx   
            // 0x79 => {}, // ADC_ax   
            // 0x7d => {}, // ADC_ay   

            // 0xc1 => {}, // CMP_inx_x
            // 0xc5 => {}, // CMP_z_pg 
            CMPImm = 0xc9,
            // 0xcd => {}, // CMP_abs  
            // 0xd1 => {}, // CMP_ind_y
            // 0xd5 => {}, // CMP_dx   
            // 0xd9 => {}, // CMP_ax   
            // 0xdd => {}, // CMP_ay   

            // 0xe1 => {}, // SBC_inx_x
            // 0xe5 => {}, // SBC_z_pg 
            SBCImm = 0xe9,
            // 0xed => {}, // SBC_abs  
            // 0xf1 => {}, // SBC_ind_y
            // 0xf5 => {}, // SBC_dx   
            // 0xf9 => {}, // SBC_ax   
            // 0xfd => {}, // SBC_ay   

            // 0x06 => {}, // ASL_z_pg 
            // 0x0a => {}, // ASL      
            // 0x0e => {}, // ASL_abs  
            // 0x16 => {}, // ASL_dx   
            // 0x1e => {}, // ASL_ax   

            // 0x46 => {}, // LSR_z_pg 
            // 0x4a => {}, // LSR      
            // 0x4e => {}, // LSR_abs  
            // 0x56 => {}, // LSR_dx   
            // 0x5e => {}, // LSR_ax   

            // Rotates
            // 0x26 => {}, // ROL_z_pg 
            // 0x2a => {}, // ROL      
            // 0x2e => {}, // ROL_abs  
            // 0x36 => {}, // ROL_dx   
            // 0x3e => {}, // ROL_ax   

            // 0x66 => {}, // ROR_z_pg 
            // 0x6a => {}, // ROR      
            // 0x6e => {}, // ROR_abs  
            // 0x76 => {}, // ROR_dx   
            // 0x7e => {}, // ROR_ax   

            // Increments
            DECZpg = 0xc6,
            // 0xce => {}, // DEC_abs  
            // 0xd6 => {}, // DEC_dx   
            // 0xde => {}, // DEC_ax   

            // 0xe6 => {}, // INC_z_pg 
            // 0xee => {}, // INC_abs  
            // 0xf6 => {}, // INC_dx   
            // 0xfe => {, // INC_ax   

            // The ever important nop
            // Observe all its majesty
            NOP = 0xea,
    }
}
