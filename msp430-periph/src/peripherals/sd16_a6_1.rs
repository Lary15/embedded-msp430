//! SD16_A6

utils::periph! {
    /// SD16_A6
    SD16_A6;
    /// SD16 Input Control Register Channel 0
    rw INCTL0 @ 0x00: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN0INCH: 0..2 = enum IN0INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN0GAIN0: 3 = struct IN0GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN0GAIN1: 4 = struct IN0GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN0GAIN2: 5 = struct IN0GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN0INTDLY: 6..7 = enum IN0INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 1
    rw INCTL1 @ 0x01: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN1INCH: 0..2 = enum IN1INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN1GAIN0: 3 = struct IN1GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN1GAIN1: 4 = struct IN1GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN1GAIN2: 5 = struct IN1GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN1INTDLY: 6..7 = enum IN1INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 2
    rw INCTL2 @ 0x02: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN2INCH: 0..2 = enum IN2INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN2GAIN0: 3 = struct IN2GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN2GAIN1: 4 = struct IN2GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN2GAIN2: 5 = struct IN2GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN2INTDLY: 6..7 = enum IN2INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 3
    rw INCTL3 @ 0x03: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN3INCH: 0..2 = enum IN3INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN3GAIN0: 3 = struct IN3GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN3GAIN1: 4 = struct IN3GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN3GAIN2: 5 = struct IN3GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN3INTDLY: 6..7 = enum IN3INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 4
    rw INCTL4 @ 0x04: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN4INCH: 0..2 = enum IN4INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN4GAIN0: 3 = struct IN4GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN4GAIN1: 4 = struct IN4GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN4GAIN2: 5 = struct IN4GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN4INTDLY: 6..7 = enum IN4INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 5
    rw INCTL5 @ 0x05: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN5INCH: 0..2 = enum IN5INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN5GAIN0: 3 = struct IN5GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN5GAIN1: 4 = struct IN5GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN5GAIN2: 5 = struct IN5GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN5INTDLY: 6..7 = enum IN5INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Preload Register Channel 0
    rw PRE0 @ 0x08: u8 = 0_0 {
        /// SD16 Preload Register Channel 0
        PRE0: 0..7 = struct PRE0Field(u8);
    }
    /// SD16 Preload Register Channel 1
    rw PRE1 @ 0x09: u8 = 0_0 {
        /// SD16 Preload Register Channel 1
        PRE1: 0..7 = struct PRE1Field(u8);
    }
    /// SD16 Preload Register Channel 2
    rw PRE2 @ 0x0a: u8 = 0_0 {
        /// SD16 Preload Register Channel 2
        PRE2: 0..7 = struct PRE2Field(u8);
    }
    /// SD16 Preload Register Channel 3
    rw PRE3 @ 0x0b: u8 = 0_0 {
        /// SD16 Preload Register Channel 3
        PRE3: 0..7 = struct PRE3Field(u8);
    }
    /// SD16 Preload Register Channel 4
    rw PRE4 @ 0x0c: u8 = 0_0 {
        /// SD16 Preload Register Channel 4
        PRE4: 0..7 = struct PRE4Field(u8);
    }
    /// SD16 Preload Register Channel 5
    rw PRE5 @ 0x0d: u8 = 0_0 {
        /// SD16 Preload Register Channel 5
        PRE5: 0..7 = struct PRE5Field(u8);
    }
    /// Sigma Delta ADC 16 Control Register
    rw CTL @ 0x50: u16 = 0_0 {
        /// SD16 Overflow Interupt Enable
        OVIE: 1 = struct OVIE(bool);
        /// SD16 Switch internal Reference on
        REFON: 2 = struct REFON(bool);
        /// SD16 Switch Vmid Buffer on
        VMIDON: 3 = struct VMIDON(bool);
        /// SD16 Clock Source Select 0
        SSEL: 4..5 = enum SSEL {
            /// SD16 Clock Source Select MCLK
            SSEL_0 = 0b00,
            /// SD16 Clock Source Select SMCLK
            SSEL_1 = 0b01,
            /// SD16 Clock Source Select ACLK
            SSEL_2 = 0b10,
            /// SD16 Clock Source Select TACLK
            SSEL_3 = 0b11,
        }
        /// SD16 Clock Divider Select 0
        DIV: 6..7 = enum DIV {
            /// SD16 Clock Divider Select /1
            DIV_0 = 0b00,
            /// SD16 Clock Divider Select /2
            DIV_1 = 0b01,
            /// SD16 Clock Divider Select /4
            DIV_2 = 0b10,
            /// SD16 Clock Divider Select /8
            DIV_3 = 0b11,
        }
        /// SD16 Low Power Mode Enable
        LP: 8 = struct LP(bool);
        /// SD16 2.Clock Divider Select 0
        XDIV: 9..11 = enum XDIV {
            /// SD16 2.Clock Divider Select /1
            XDIV_0 = 0b000,
            /// SD16 2.Clock Divider Select /3
            XDIV_1 = 0b001,
            /// SD16 2.Clock Divider Select /16
            XDIV_2 = 0b010,
            /// SD16 2.Clock Divider Select /48
            XDIV_3 = 0b011,
        }
    }
    /// SD16 Channel 0 Control Register
    rw CCTL0 @ 0x52: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C0GRP: 0 = struct C0GRP(bool);
        /// SD16 Start Conversion
        C0SC: 1 = struct C0SC(bool);
        /// SD16 Channel x Interrupt Flag
        C0IFG: 2 = struct C0IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C0IE: 3 = struct C0IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C0DF: 4 = struct C0DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C0OVIFG: 5 = struct C0OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C0LSBACC: 6 = struct C0LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C0LSBTOG: 7 = struct C0LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C0OSR0: 8 = struct C0OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C0OSR1: 9 = struct C0OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C0SNGL: 10 = struct C0SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C0XOSR: 11 = struct C0XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C0UNI: 12 = struct C0UNI(bool);
    }
    /// SD16 Channel 1 Control Register
    rw CCTL1 @ 0x54: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C1GRP: 0 = struct C1GRP(bool);
        /// SD16 Start Conversion
        C1SC: 1 = struct C1SC(bool);
        /// SD16 Channel x Interrupt Flag
        C1IFG: 2 = struct C1IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C1IE: 3 = struct C1IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C1DF: 4 = struct C1DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C1OVIFG: 5 = struct C1OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C1LSBACC: 6 = struct C1LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C1LSBTOG: 7 = struct C1LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C1OSR0: 8 = struct C1OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C1OSR1: 9 = struct C1OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C1SNGL: 10 = struct C1SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C1XOSR: 11 = struct C1XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C1UNI: 12 = struct C1UNI(bool);
    }
    /// SD16 Channel 2 Control Register
    rw CCTL2 @ 0x56: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C2GRP: 0 = struct C2GRP(bool);
        /// SD16 Start Conversion
        C2SC: 1 = struct C2SC(bool);
        /// SD16 Channel x Interrupt Flag
        C2IFG: 2 = struct C2IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C2IE: 3 = struct C2IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C2DF: 4 = struct C2DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C2OVIFG: 5 = struct C2OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C2LSBACC: 6 = struct C2LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C2LSBTOG: 7 = struct C2LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C2OSR0: 8 = struct C2OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C2OSR1: 9 = struct C2OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C2SNGL: 10 = struct C2SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C2XOSR: 11 = struct C2XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C2UNI: 12 = struct C2UNI(bool);
    }
    /// SD16 Channel 3 Control Register
    rw CCTL3 @ 0x58: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C3GRP: 0 = struct C3GRP(bool);
        /// SD16 Start Conversion
        C3SC: 1 = struct C3SC(bool);
        /// SD16 Channel x Interrupt Flag
        C3IFG: 2 = struct C3IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C3IE: 3 = struct C3IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C3DF: 4 = struct C3DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C3OVIFG: 5 = struct C3OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C3LSBACC: 6 = struct C3LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C3LSBTOG: 7 = struct C3LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C3OSR0: 8 = struct C3OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C3OSR1: 9 = struct C3OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C3SNGL: 10 = struct C3SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C3XOSR: 11 = struct C3XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C3UNI: 12 = struct C3UNI(bool);
    }
    /// SD16 Channel 4 Control Register
    rw CCTL4 @ 0x5a: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C4GRP: 0 = struct C4GRP(bool);
        /// SD16 Start Conversion
        C4SC: 1 = struct C4SC(bool);
        /// SD16 Channel x Interrupt Flag
        C4IFG: 2 = struct C4IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C4IE: 3 = struct C4IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C4DF: 4 = struct C4DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C4OVIFG: 5 = struct C4OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C4LSBACC: 6 = struct C4LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C4LSBTOG: 7 = struct C4LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C4OSR0: 8 = struct C4OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C4OSR1: 9 = struct C4OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C4SNGL: 10 = struct C4SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C4XOSR: 11 = struct C4XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C4UNI: 12 = struct C4UNI(bool);
    }
    /// SD16 Channel 5 Control Register
    rw CCTL5 @ 0x5c: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C5GRP: 0 = struct C5GRP(bool);
        /// SD16 Start Conversion
        C5SC: 1 = struct C5SC(bool);
        /// SD16 Channel x Interrupt Flag
        C5IFG: 2 = struct C5IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C5IE: 3 = struct C5IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C5DF: 4 = struct C5DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C5OVIFG: 5 = struct C5OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C5LSBACC: 6 = struct C5LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C5LSBTOG: 7 = struct C5LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C5OSR0: 8 = struct C5OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C5OSR1: 9 = struct C5OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C5SNGL: 10 = struct C5SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C5XOSR: 11 = struct C5XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C5UNI: 12 = struct C5UNI(bool);
    }
    /// SD16 Channel 0 Conversion Memory
    rw MEM0 @ 0x60: u16 = 0_0 {
        /// SD16 Channel 0 Conversion Memory
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// SD16 Channel 1 Conversion Memory
    rw MEM1 @ 0x62: u16 = 0_0 {
        /// SD16 Channel 1 Conversion Memory
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// SD16 Channel 2 Conversion Memory
    rw MEM2 @ 0x64: u16 = 0_0 {
        /// SD16 Channel 2 Conversion Memory
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// SD16 Channel 3 Conversion Memory
    rw MEM3 @ 0x66: u16 = 0_0 {
        /// SD16 Channel 3 Conversion Memory
        MEM3: 0..15 = struct MEM3Field(u16);
    }
    /// SD16 Channel 4 Conversion Memory
    rw MEM4 @ 0x68: u16 = 0_0 {
        /// SD16 Channel 4 Conversion Memory
        MEM4: 0..15 = struct MEM4Field(u16);
    }
    /// SD16 Channel 5 Conversion Memory
    rw MEM5 @ 0x6a: u16 = 0_0 {
        /// SD16 Channel 5 Conversion Memory
        MEM5: 0..15 = struct MEM5Field(u16);
    }
    /// SD16 Interrupt Vector Register
    rw IV @ 0xfe: u16 = 0_0 {
        /// SD16 Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
