use crate::compiler::TranslationMap;
use crate::intrinsics::IntrinsicStore;
use crate::node::NodeConstruct;
use crate::source::{ErrorCollate, Spanned, TextMap};

pub fn translate(source_map: &TextMap, intrinsics: &IntrinsicStore) -> Option<TranslationMap> {
	let mut syntax_unit = emit_errors(source_map, crate::parser::parse(source_map))?;
	emit_errors(source_map, syntax_unit.accept(&mut crate::compiler::VariableExposition::default()))?;

	let mut inference_engine = crate::compiler::InferenceEngine::default();
	syntax_unit.accept(&mut crate::compiler::TypeLocaliser::new(intrinsics));
	emit_errors(source_map, syntax_unit.accept(&mut inference_engine))?;
	let mut type_annotator = crate::compiler::TypeAnnotator::new(inference_engine.context());
	emit_errors(source_map, syntax_unit.accept(&mut type_annotator))?;

	// TODO: Replace translation
	syntax_unit.accept(&mut crate::compiler::LowerTransform::default());

	let elements = syntax_unit.accept(&mut crate::compiler::Translator::new(intrinsics));
	Some(crate::compiler::TranslationMap::new(elements))
}

pub fn emit_errors<T, E>(source_map: &TextMap, result: Result<T, ErrorCollate<Spanned<E>>>)
                         -> Option<T> where E: std::fmt::Display {
	match result {
		Ok(value) => Some(value),
		Err(errors) => {
			for error in errors {
				crate::source::emit(&source_map, &error);
				println!()
			}
			return None;
		}
	}
}
