use crate::color::ansi_24_bit::Ansi24Bit;
use crate::color::ansi_3_bit::Ansi3Bit;
use crate::color::ansi_4_bit::Ansi4Bit;
use crate::color::ansi_8_bit::Ansi8Bit;
use crate::color::colorless::Colorless;
use enum_iterator::Sequence;

#[test]
fn test_cardinalities() {
    assert_eq!(Colorless::CARDINALITY, 1);
    assert_eq!(Ansi3Bit::CARDINALITY, 8);
    assert_eq!(Ansi4Bit::CARDINALITY, 16);
    assert_eq!(Ansi8Bit::CARDINALITY, 256);
    assert_eq!(Ansi24Bit::CARDINALITY, 16_777_216);
}
