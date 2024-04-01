use crate::considerations::{Consideration, ConsiderationType};
use crate::utils::trim_type_name;
use bevy::prelude::Component;
use bevy::reflect::{GetTypeRegistration, TypeRegistration};
use bevy::utils::Uuid;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::any::{type_name, TypeId};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Filter {
    /// Entiy must contain Component with this TypeId
    Inclusive(TypeId),
    /// Entity must not contain Component with this TypeId
    Exclusive(TypeId),
}

impl Filter {
    pub fn component_type_id(&self) -> TypeId {
        match self {
            Filter::Inclusive(t) => *t,
            Filter::Exclusive(t) => *t,
        }
    }
}

pub struct Decision {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) action_name: String,
    pub(crate) action: TypeId,
    pub(crate) type_registration: TypeRegistration,
    pub(crate) is_targeted: bool,
    pub(crate) considerations: Vec<Consideration>,
    pub(crate) base_score: f32,
    pub(crate) subject_filters: Vec<Filter>,
    pub(crate) target_filters: Vec<Filter>,
    pub(crate) intertia: Option<f32>,
}

fn gen_random_tag() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

impl Decision {
    fn construct<C: Component + GetTypeRegistration>(is_targeted: bool) -> Self {
        let action_name: String = trim_type_name(type_name::<C>()).into();
        let tag: String = gen_random_tag().to_ascii_lowercase();
        let name = action_name.clone() + "_" + &tag;
        Self {
            id: Uuid::new_v4(),
            name,
            action_name,
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted,
            base_score: 1.0,
            considerations: Vec::new(),
            subject_filters: Vec::new(),
            target_filters: Vec::new(),
            intertia: None,
        }
    }

    pub fn simple<C: Component + GetTypeRegistration>() -> Self {
        Decision::construct::<C>(false)
    }

    pub fn targeted<C: Component + GetTypeRegistration>() -> Self {
        Decision::construct::<C>(true)
    }

    pub fn add_consideration(mut self, consideration: Consideration) -> Self {
        if !self.is_targeted
            && consideration.consideration_type == ConsiderationType::Targeted
        {
            panic!(
                "Cannot add targeted consideration '{}' to simple decision '{}'!",
                consideration.name, self.name
            )
        }
        self.considerations.push(consideration);
        self
    }

    pub fn subject_filter_include<C: Component>(mut self) -> Self {
        self.subject_filters
            .push(Filter::Inclusive(TypeId::of::<C>()));
        self
    }

    pub fn subject_filter_exclude<C: Component>(mut self) -> Self {
        self.subject_filters
            .push(Filter::Exclusive(TypeId::of::<C>()));
        self
    }

    pub fn target_filter_include<C: Component>(mut self) -> Self {
        if !self.is_targeted {
            panic!("Only targeted Decisions may have target filters")
        }

        self.target_filters
            .push(Filter::Inclusive(TypeId::of::<C>()));
        self
    }

    pub fn target_filter_exclude<C: Component>(mut self) -> Self {
        if !self.is_targeted {
            panic!("Only targeted Decisions may have target filters")
        }

        self.target_filters
            .push(Filter::Exclusive(TypeId::of::<C>()));
        self
    }

    /// Set the base score for this decision. The base score is the initial value that
    /// gets multiplied cumulatively by each consideration. The default base score is 1.0.
    /// This can be used to either create a fallback decision with no considerations, so
    /// that the AI does something appropriate when there is no good decision to make.
    /// This can also be used to weight decisions at the decision level.
    pub fn set_base_score(mut self, score: f32) -> Self {
        if score <= 0.0 || score >= 10.0 {
            panic!("base_score must be between 0.0 and 10.0");
        }
        self.base_score = score;
        self
    }

    /// Sets the inertia for this decision which is by default 0. Intertia is 'added' to
    /// the decision's current utility when it is active to prevent an agent from
    /// oscillating between two similarly weighted choices.
    pub fn set_intertia(mut self, intertia: f32) -> Self {
        if !(0.0..1.0).contains(&intertia) {
            panic!("intertia must be between 0.0 and 1.0");
        }
        self.intertia = Some(intertia);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}
