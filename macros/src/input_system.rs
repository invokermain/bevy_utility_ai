use crate::common::{parse_input, ParsedInput, SigType};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemFn};

pub(crate) fn input_system(
    _: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;
    let quoted_name = format!("{}", name);

    let parsed_sig: Vec<ParsedInput> = item_fn
        .sig
        .inputs
        .iter()
        .map(parse_input)
        .collect::<Result<Vec<ParsedInput>, Error>>()?;

    let component_arg_types: Vec<proc_macro2::TokenStream> = parsed_sig
        .iter()
        .filter_map(handle_component_arg_types)
        .collect();
    let component_arg_idents = parsed_sig
        .iter()
        .filter(|sig| {
            sig.sig_type == SigType::Component
                || sig.sig_type == SigType::WrappedComponent
        })
        .map(|sig| sig.ident.clone());
    let extra_args: Vec<proc_macro2::TokenStream> = parsed_sig
        .iter()
        .filter(|sig| sig.sig_type == SigType::Extra)
        .map(|ParsedInput { ident, tokens, .. }| quote! { #ident: #tokens })
        .collect();

    let vis = item_fn.vis;
    let body = item_fn.block;

    let output = quote! {
        #vis fn #name(
            mut query_input_system: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta #(, #component_arg_types)*)>,
            res_ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>,
            #[cfg(debug_assertions)]
            mut event_writer: bevy::prelude::EventWriter<bevy_utility_ai::events::InputCalculatedEvent>
            #(, #extra_args)*
        ) {
            let _span = bevy::prelude::debug_span!(target: "bevy_utility_ai", "Calculating Input", input = #quoted_name).entered();
            let key = bevy_utility_ai::utils::type_id_of(&#name);

            for (entity, mut ai_meta #(, #component_arg_idents)*) in query_input_system.iter_mut() {
                let _span = bevy::prelude::debug_span!(target: "bevy_utility_ai", "", entity = entity.index()).entered();

                let ai_definition = &res_ai_definitions.map[&ai_meta.ai_definition];

                if !ai_definition.requires_simple_input(&key) {
                    bevy::prelude::debug!(target: "bevy_utility_ai", "skipped calculating inputs for this entity");
                    continue;
                };

                let score = #body;
                let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                *entry = score;
                bevy::prelude::debug!(target: "bevy_utility_ai", "score {:.2}", score);

                #[cfg(debug_assertions)]
                event_writer.send(bevy_utility_ai::events::InputCalculatedEvent {
                    entity,
                    target: None,
                    input: #quoted_name.to_string(),
                    score
                });
            }
        }
    };

    Ok(output.into())
}

fn handle_component_arg_types(input: &ParsedInput) -> Option<proc_macro2::TokenStream> {
    match input.sig_type {
        SigType::Component => {
            let sig = input.tokens.clone();
            Some(quote! { &#sig })
        }
        SigType::WrappedComponent => Some(input.tokens.clone()),
        _ => None,
    }
}
