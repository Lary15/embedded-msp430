//! Special Function

utils::periph! {
    /// Special Function
    SpecialFunction;
    /// Interrupt Enable 1
    rw IE1 @ 0x00: u8 = 0_0 {
        /// WDTIE
        WDTIE: 0 = struct WDTIE(bool);
        /// OFIE
        OFIE: 1 = struct OFIE(bool);
        /// NMIIE
        NMIIE: 4 = struct NMIIE(bool);
        /// ACCVIE
        ACCVIE: 5 = struct ACCVIE(bool);
    }
    /// Interrupt Flag 1
    rw IFG1 @ 0x02: u8 = 0_0 {
        /// Watchdog Interrupt Flag
        WDTIFG: 0 = struct WDTIFG(bool);
        /// Osc. Fault Interrupt Flag
        OFIFG: 1 = struct OFIFG(bool);
        /// Power On Interrupt Flag
        PORIFG: 2 = struct PORIFG(bool);
        /// Reset Interrupt Flag
        RSTIFG: 3 = struct RSTIFG(bool);
        /// NMI Interrupt Flag
        NMIIFG: 4 = struct NMIIFG(bool);
    }
    /// Interrupt Enable 2
    rw IE2 @ 0x01: u8 = 0_0 {
        /// BTIE
        BTIE: 7 = struct BTIE(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// BTIFG
        BTIFG: 7 = struct BTIFG(bool);
    }
}
