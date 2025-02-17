use std::collections::HashMap;

use bevy::{
    asset::ron::Deserializer,
    gltf::{GltfMeshExtras, GltfSceneExtras},
    prelude::*,
    reflect::serde::TypedReflectDeserializer,
};
use serde::de::DeserializeSeed as _;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, check_for_gltf_extras);
}

fn check_for_gltf_extras(world: &mut World) {
    let mut reflect_components = vec![];

    for (entity, mesh_extras, scene_extras, extras) in world
        .query_filtered::<(
            Entity,
            Option<&GltfMeshExtras>,
            Option<&GltfSceneExtras>,
            Option<&GltfExtras>,
        ), Or<(
            Added<GltfMeshExtras>,
            Added<GltfSceneExtras>,
            Added<GltfExtras>,
        )>>()
        .iter(world)
    {
        for value in [
            mesh_extras.map(|e| &e.value),
            scene_extras.map(|e| &e.value),
            extras.map(|e| &e.value),
        ]
        .into_iter()
        .filter_map(|v| v)
        {
            if let Ok(values) = ron::from_str::<HashMap<String, String>>(value) {
                for (component_path, component) in values {
                    let type_registry = world.resource::<AppTypeRegistry>().0.read();
                    if let Some(component_type) = type_registry.get_with_type_path(&component_path)
                    {
                        match Deserializer::from_str(&component) {
                            Ok(mut deser) => {
                                match TypedReflectDeserializer::new(component_type, &type_registry)
                                    .deserialize(&mut deser)
                                {
                                    Ok(reflected) => {
                                        reflect_components.push((entity, component_path, reflected))
                                    }
                                    Err(err) => warn!("{component_path} is invalid: {err}"),
                                }
                            }
                            Err(err) => {
                                warn!("Failed to make deserializer for {component_type:?}: {err}")
                            }
                        }
                    } else {
                        warn!("component not found: {component_path}");
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
                        match component_registration.data::<ReflectComponent>() {
                            Some(reflect_component) => {
                                reflect_component.insert(&mut entity, &*reflected, &type_registry)
                            }
                            None => warn!("Component `{}` isn't reflectable", component_path),
                        };
                    }
                    None => warn!("Unknown component type: `{}`", component_path),
                }
            }
        }
    }
}
