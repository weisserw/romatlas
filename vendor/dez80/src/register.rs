//! Contains models and functions used to manage a Z80 processor's internal storage,
//! including single registers, register pairs, and status flags.

use std::fmt;
use strum_macros::Display;

macro_rules! impl_display_register {
    ($( $ident: ident ),*) => (
        $( impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // Write out the enum variant name, replacing underscores with straight quotes.
                write!(f, "{}", format!("{:?}", self).replace("_", "'"))
            }
        } )*
    );
}

/// Represents the order of a given register in a pair.
/// For example, `RegisterPairType::AF` includes the
/// registers `SingleRegisterType::A` and
/// `SingleRegisterType::F`, which are high and low
/// within the pair respectively.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterOrder {
    High,
    Low,
}

/// Used to identify a single register in an
/// implementation-independent manner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SingleRegisterType {
    A,   // Accumulator register
    F,   // Flag register
    B,   // B general purpose register
    C,   // C general purpose register
    D,   // D general purpose register
    E,   // E general purpose register
    H,   // H general purpose register
    L,   // L general purpose register
    IXH, // IX index register, high byte
    IXL, // IX index register, low byte
    IYH, // IY index register, high byte
    IYL, // IY index register, low byte
    PCH, // Program counter, high byte
    PCL, // Program counter, low byte
    SPH, // Stack pointer, high byte
    SPL, // Stack pointer, low byte
    I,   // Interrupt vector register
    R,   // Memory refresh register
    W,   // Temporary storage register, high byte
    Z,   // Temporary storage register, low byte
    A_,  // Alternate accumulator register
    F_,  // Alternate flag register
    B_,  // Alternate B general purpose register
    C_,  // Alternate C general purpose register
    D_,  // Alternate D general purpose register
    E_,  // Alternate E general purpose register
    H_,  // Alternate H general purpose register
    L_,  // Alternate L general purpose register
}

impl SingleRegisterType {
    #[inline(always)]
    pub fn to_register_pair_type(self) -> (RegisterPairType, RegisterOrder) {
        use RegisterOrder::*;
        use RegisterPairType::*;
        use SingleRegisterType::*;

        match self {
            A => (AF, High),
            F => (AF, Low),
            B => (BC, High),
            C => (BC, Low),
            D => (DE, High),
            E => (DE, Low),
            H => (HL, High),
            L => (HL, Low),
            IXH => (IX, High),
            IXL => (IX, Low),
            IYH => (IY, High),
            IYL => (IY, Low),
            PCH => (PC, High),
            PCL => (PC, Low),
            SPH => (SP, High),
            SPL => (SP, Low),
            I => (IR, High),
            R => (IR, Low),
            W => (WZ, High),
            Z => (WZ, Low),
            A_ => (AF_, High),
            F_ => (AF_, Low),
            B_ => (BC_, High),
            C_ => (BC_, Low),
            D_ => (DE_, High),
            E_ => (DE_, Low),
            H_ => (HL_, High),
            L_ => (HL_, Low),
        }
    }
}

/// Used to identify a register pair in an
/// implementation-independent manner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterPairType {
    AF,  // Accumulator and flag registers
    BC,  // B and C general purpose registers
    DE,  // D and E general purpose registers
    HL,  // H and L general purpose registers
    IX,  // IX index register
    IY,  // IY index register
    PC,  // Program counter
    SP,  // Stack pointer
    IR,  // Interrupt vector and memory refresh registers
    WZ,  // Temporary storage register
    AF_, // Alternate accumulator and flag registers
    BC_, // Alternate B and C general purpose registers
    DE_, // Alternate D and E general purpose registers
    HL_, // Alternate H and L general purpose registers
}

impl RegisterPairType {
    #[inline(always)]
    pub fn to_single_register_types(self) -> (SingleRegisterType, SingleRegisterType) {
        use RegisterPairType::*;
        use SingleRegisterType::*;

        match self {
            AF => (A, F),
            BC => (B, C),
            DE => (D, E),
            HL => (H, L),
            IX => (IXH, IXL),
            IY => (IYH, IYL),
            PC => (PCH, PCL),
            SP => (SPH, SPL),
            IR => (I, R),
            WZ => (W, Z),
            AF_ => (A_, F_),
            BC_ => (B_, C_),
            DE_ => (D_, E_),
            HL_ => (H_, L_),
        }
    }
}

impl_display_register!(SingleRegisterType, RegisterPairType);

/// Used to identify either a single register or a register pair.
pub enum RegisterType {
    SingleRegister(SingleRegisterType),
    RegisterPair(RegisterPairType),
}

/// Represents individual Z80 status flags, with their masks as their values.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
#[repr(u8)]
pub enum Flag {
    S = 0b1000_0000,
    Z = 0b0100_0000,
    F5 = 0b0010_0000,
    H = 0b0001_0000,
    F3 = 0b0000_1000,
    PV = 0b0000_0100,
    N = 0b0000_0010,
    C = 0b0000_0001,
}

impl Flag {
    pub const SIGN: Self = Flag::S;
    pub const ZERO: Self = Flag::Z;
    pub const BIT5: Self = Flag::F5;
    pub const HALF_CARRY: Self = Flag::H;
    pub const BIT3: Self = Flag::F3;
    pub const PARITY: Self = Flag::PV;
    pub const OVERFLOW: Self = Flag::PV;
    pub const SUBTRACT: Self = Flag::N;
    pub const CARRY: Self = Flag::C;

    /// Returns the zero-based bit index of a flag as stored
    /// in the flag register (`SingleRegisterType::F`).
    pub fn index(self) -> usize {
        use Flag::*;

        match self {
            S => 7,
            Z => 6,
            F5 => 5,
            H => 4,
            F3 => 3,
            PV => 2,
            N => 1,
            C => 0,
        }
    }
}
