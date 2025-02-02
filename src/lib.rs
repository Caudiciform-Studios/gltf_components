use std::collections::HashMap;

use bevy::{
    prelude::*,
    asset::ron::Deserializer,
    reflect::serde::TypedReflectDeserializer,
    gltf::GltfExtras,
};
use serde::de::DeserializeSeed as _;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, check_for_gltf_extras);
}

fn check_for_gltf_extras(
    world: &mut World,
) {
    let mut reflect_components = vec![];

    for (entity, extras) in world.query_filtered::<(Entity, &GltfExtras), Added<GltfExtras>>().iter(world) {
        if let Ok(values) = ron::from_str::<HashMap<String, String>>(&extras.value) {
            for (component_path, component) in values {
                let type_registry = world.resource::<AppTypeRegistry>().0.read();
                if let Some(component_type) = type_registry.get_with_type_path(&component_path) {
                    match Deserializer::from_str(&component) {
                        Ok(mut deser) =>
                            match TypedReflectDeserializer::new(component_type, &type_registry)
                                    .deserialize(&mut deser) {
                                Ok(reflected) => reflect_components.push((entity, component_path, reflected)),
                                Err(err) => warn!("{component_path} is invalid: {err}")
                            }
                        Err(err) => warn!("Failed to make deserializer for {component_type:?}: {err}")
                    }

                }
            }
        }
    }

    if !reflect_components.is_empty() {
        for (entity, component_path, reflected) in reflect_components {
            let app_type_registry = world.resource::<AppTypeRegistry>().clone();
            let type_registry = app_type_registry.0.read();
            if let Ok(mut entity) = world.get_entity_mut(entity) {
                match type_registry.get_with_type_path(&component_path) {
                    Some(component_registration) => {
                        match component_registration
                            .data::<ReflectComponent>() {
                                Some(reflect_component) => {
                                    reflect_component.insert(&mut entity, &*reflected, &type_registry)
                                },
                                None => warn!("Component `{}` isn't reflectable", component_path)
                        };
                    },
                    None => warn!("Unknown component type: `{}`", component_path)
                }
            }
        }
    }
}
