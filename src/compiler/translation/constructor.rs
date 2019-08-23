macro_rules! instruction {
    ($direction: ident; $polarization: expr, $instruction: expr, $span: expr) => {
		Spanned::new(super::Element::Instruction(crate::compiler::translation::Instruction {
			direction: crate::interpreter::Direction::$direction,
			polarization: $polarization,
			instruction: $instruction,
		}), $span)
    };

    ($direction: ident, $polarization: ident, $instruction: expr, $span: expr) => {
        instruction!($direction ; Some(crate::interpreter::Direction::$polarization), $instruction, $span)
    };

    ($direction: ident, $instruction: expr, $span: expr) => {
        instruction!($direction ; None, $instruction, $span)
    };
}
