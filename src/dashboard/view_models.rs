use crate::AIDefinition;
use std::any::TypeId;

#[derive(Debug, Clone)]
pub(crate) struct ViewAIDefinition {
    pub(crate) id: TypeId,
    pub(crate) name: String,
}

impl ViewAIDefinition {
    pub(crate) fn from_ai_definition(ai_definition: &AIDefinition) -> Self {
        ViewAIDefinition {
            id: ai_definition.marker_type,
            name: ai_definition.name.clone(),
        }
    }
}
