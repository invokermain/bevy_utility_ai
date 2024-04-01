use crate::response_curves::{InputTransform, Linear, ResponseCurve};
use crate::utils;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::IntoSystemConfigs;
use bevy::utils::Uuid;
use std::any::TypeId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConsiderationType {
    Simple,
    Targeted,
}

/// Considerations are a transformed view on an Input which can be used by to calculate a
/// score for a specific entity, and potentially a target entity. These scores make up
/// the weightings for a Decision.
pub struct Consideration {
    pub id: Uuid,
    pub name: String,
    /// The TypeId of the Consideration's Input system.
    pub input: TypeId,
    pub response_curve: ResponseCurve,
    pub consideration_type: ConsiderationType,
    /// The name of the Consideration's Input.
    pub input_name: String,
    /// The lower bound for the calculated score, defaults to 0.0. Must be >= 0.0.
    pub lower_bound: f32,
    /// The upper bound for the calculated score, defaults to 1.0. Must be >= 0.0 &
    /// greater than the lower_bound.
    pub upper_bound: f32,
    pub(crate) system_app_config: Option<SystemConfigs>,
}

impl Consideration {
    pub fn calculate_score(&self, input_score: f32) -> f32 {
        self.response_curve
            .transform(input_score)
            .clamp(self.lower_bound, self.upper_bound)
    }

    fn construct(
        input_name: String,
        input: TypeId,
        consideration_type: ConsiderationType,
        system_app_config: SystemConfigs,
    ) -> Self {
        let response_curve = ResponseCurve::LinearCurve(Linear::new(1.0));
        Self {
            id: Uuid::new_v4(),
            name: format!("{} - {}", input_name, response_curve),
            input,
            input_name,
            consideration_type,
            system_app_config: Some(system_app_config),
            response_curve: ResponseCurve::LinearCurve(Linear::new(1.0)),
            lower_bound: 0.0,
            upper_bound: 1.0,
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

    /// Sets the lower & upper bounds, by default these are 0.0 and 1.0.
    pub fn with_bounds(self, lower: f32, upper: f32) -> Self {
        if lower < 0.0 {
            panic!("Consideration's lower bound must be >= 0.0");
        }
        if lower >= upper {
            panic!("The lower bound must be less than the upper bound");
        }
        Self {
            lower_bound: lower,
            upper_bound: upper,
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
