use crate::response_curves::{Linear, ResponseCurve};
use crate::utils;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::IntoSystemConfigs;
use std::any::TypeId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConsiderationType {
    Simple,
    Targeted,
}

pub struct Consideration {
    pub name: String,
    pub input: TypeId,
    pub response_curve: ResponseCurve,
    pub consideration_type: ConsiderationType,
    pub input_name: String,
    // This is Option to allow it to be Taken out later on as SystemAppConfig does not implement
    // clone.
    pub(crate) system_app_config: Option<SystemConfigs>,
}

impl Consideration {
    fn construct(
        input_name: String,
        input: TypeId,
        consideration_type: ConsiderationType,
        system_app_config: SystemConfigs,
    ) -> Self {
        let response_curve = ResponseCurve::Linear(Linear::new(1.0));
        Self {
            name: format!("{} - {}", input_name, response_curve),
            input,
            input_name,
            consideration_type,
            system_app_config: Some(system_app_config),
            response_curve: ResponseCurve::Linear(Linear::new(1.0)),
        }
    }

    pub fn simple<M>(input: impl IntoSystemConfigs<M> + 'static) -> Self {
        Consideration::construct(
            utils::trim_type_name(utils::type_name_of(&input)).into(),
            utils::type_id_of(&input),
            ConsiderationType::Simple,
            input.into_configs(),
        )
    }

    pub fn targeted<M>(input: impl IntoSystemConfigs<M> + 'static) -> Self {
        Consideration::construct(
            utils::trim_type_name(utils::type_name_of(&input)).into(),
            utils::type_id_of(&input),
            ConsiderationType::Targeted,
            input.into_configs(),
        )
    }

    pub fn with_response_curve(self, response_curve: impl Into<ResponseCurve>) -> Self {
        let response_curve = response_curve.into();
        Self {
            name: format!("{} - {}", self.input_name, response_curve),
            response_curve,
            ..self
        }
    }

    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }
}
