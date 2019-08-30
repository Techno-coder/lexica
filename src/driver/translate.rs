use crate::compiler::TranslationMap;
use crate::intrinsics::IntrinsicStore;
use crate::node::NodeConstruct;
use crate::source::{ErrorCollate, Spanned, TextMap};

pub fn translate(source_map: &TextMap, intrinsics: &IntrinsicStore) -> Option<TranslationMap> {
	let mut syntax_unit = emit_errors(source_map, crate::parser::parse(source_map))?;
	syntax_unit.accept(&mut crate::compiler::ReverseExposition::default());
	emit_errors(source_map, syntax_unit.accept(&mut crate::compiler::VariableExposition::default()))?;

	let mut inference_engine = crate::compiler::InferenceEngine::default();
	syntax_unit.accept(&mut crate::compiler::TypeLocaliser::new(intrinsics));
	emit_errors(source_map, syntax_unit.accept(&mut inference_engine))?;
	let mut type_annotator = crate::compiler::TypeAnnotator::new(inference_engine.context());
	emit_errors(source_map, syntax_unit.accept(&mut type_annotator))?;

	let mut lower_transform = crate::compiler::LowerTransform::default();
	syntax_unit.accept(&mut lower_transform);

	let mut translator = crate::compiler::Translator::new(intrinsics);
	let elements = translator.translate(lower_transform.functions());
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
			None
		}
	}
}
