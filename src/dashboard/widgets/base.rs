use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::{debug, Mut, Resource, With, World};
use bevy::utils::{AHasher, HashMap};
use bevy_egui::egui::Context;
use bevy_egui::{egui, EguiContext};
use std::hash::{Hash, Hasher};

use crate::dashboard::plugin::UtilityAIDashboardWindow;

///////////////////////////////////
/// HOW TO CALL ROOT WIDGETS
///////////////////////////////////

// impl'd on `World`
pub trait WorldWidgetSystemExt {
    fn root_widget<S: RootWidgetSystem<Args = ()> + 'static>(
        &mut self,
        id: impl Hash,
    ) -> S::Output {
        self.root_widget_with::<S>(id, ())
    }

    fn root_widget_with<S: RootWidgetSystem + 'static>(
        &mut self,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output;

    fn egui_context_scope<R>(&mut self, f: impl FnOnce(&mut Self, Context) -> R) -> R;
}

impl WorldWidgetSystemExt for World {
    fn root_widget_with<S: RootWidgetSystem + 'static>(
        &mut self,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output {
        self.egui_context_scope(|world, mut ctx| {
            let id = WidgetId::new(id);

            if !world.contains_resource::<StateInstances<S>>() {
                debug!("Init system state {}", std::any::type_name::<S>());
                world.insert_resource(StateInstances::<S> {
                    instances: HashMap::new(),
                });
            }

            world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {
                let cached_state = states.instances.entry(id).or_insert_with(|| {
                    debug!(
                        "Registering system state for root widget {id:?} of type {}",
                        std::any::type_name::<S>()
                    );
                    SystemState::new(world)
                });
                let output = S::system(world, cached_state, &mut ctx, args);
                cached_state.apply(world);
                output
            })
        })
    }

    fn egui_context_scope<R>(&mut self, f: impl FnOnce(&mut Self, Context) -> R) -> R {
        let mut state =
            self.query_filtered::<&mut EguiContext, (With<EguiContext>, With<UtilityAIDashboardWindow>)>();
        let mut egui_ctx = state.single_mut(self);
        let ctx = egui_ctx.get_mut().clone();
        f(self, ctx)
    }
}

///////////////////////////////////
/// HOW TO CALL NON-ROOT WIDGETS
///////////////////////////////////

// impl'd on `egui::Ui`
pub trait UiWidgetSystemExt {
    fn add_system<S: WidgetSystem<Args = ()> + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
    ) -> S::Output {
        self.add_system_with::<S>(world, id, ())
    }

    fn add_system_with<S: WidgetSystem + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output;
}

impl UiWidgetSystemExt for egui::Ui {
    fn add_system_with<S: WidgetSystem + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output {
        let id = WidgetId::new(id);

        if !world.contains_resource::<StateInstances<S>>() {
            debug!("Init system state {}", std::any::type_name::<S>());
            world.insert_resource(StateInstances::<S> {
                instances: HashMap::new(),
            });
        }

        world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {
            let cached_state = states.instances.entry(id).or_insert_with(|| {
                debug!(
                    "Registering system state for widget {id:?} of type {}",
                    std::any::type_name::<S>()
                );
                SystemState::new(world)
            });
            let output = S::system(world, cached_state, self, args);
            cached_state.apply(world);
            output
        })
    }
}

///////////////////////////////////
/// WIDGETS IMPLEMENT THESE
///////////////////////////////////

pub trait RootWidgetSystem: SystemParam {
    type Args;
    type Output;

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ctx: &mut Context,
        args: Self::Args,
    ) -> Self::Output;
}

pub trait WidgetSystem: SystemParam {
    type Args;
    type Output;

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        args: Self::Args,
    ) -> Self::Output;
}

///////////////////////////////////
/// RUN-TIME MACHINERY
///////////////////////////////////

#[derive(Resource, Default)]
struct StateInstances<T: SystemParam + 'static> {
    instances: HashMap<WidgetId, SystemState<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);

impl WidgetId {
    pub fn new(id: impl Hash) -> Self {
        let mut hasher = AHasher::default();
        id.hash(&mut hasher);
        WidgetId(hasher.finish())
    }
}
