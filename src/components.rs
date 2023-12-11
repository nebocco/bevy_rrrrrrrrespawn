use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;
#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player {
    pub level: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Artificial;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Star;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct UiData;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct UiButton(pub String);

impl From<&EntityInstance> for UiButton {
    fn from(entity_instance: &EntityInstance) -> Self {
        let button = entity_instance
            .get_string_field("button")
            .cloned()
            .unwrap_or_else(|_| String::new());
        UiButton(button)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct UiText(pub String);

impl From<&EntityInstance> for UiText {
    fn from(entity_instance: &EntityInstance) -> Self {
        let text = entity_instance
            .get_string_field("text")
            .cloned()
            .unwrap_or_else(|_| String::new());
        UiText(text)
    }
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct RelatedDoor(pub String);

impl LdtkEntity for RelatedDoor {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> RelatedDoor {
        let entity_ref = entity_instance
            .get_entity_ref_field("Door")
            .expect("Door field not found");
        RelatedDoor(entity_ref.entity_iid.to_string())
    }
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Door(pub String);

impl LdtkEntity for Door {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Door {
        Door(entity_instance.iid.to_string())
    }
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Switch;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(16., 16.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            "Horizontal_Door" => ColliderBundle {
                collider: Collider::cuboid(20., 6.), // a bit longer to prevent tunneling
                rigid_body: RigidBody::Fixed,
                friction: Friction {
                    coefficient: 1.0,
                    ..Default::default()
                },
                rotation_constraints,
                ..Default::default()
            },
            "Vertical_Door" => ColliderBundle {
                collider: Collider::cuboid(6., 20.), // a bit longer to prevent tunneling
                rigid_body: RigidBody::Fixed,
                friction: Friction {
                    coefficient: 1.0,
                    ..Default::default()
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Star" => SensorBundle {
                collider: Collider::cuboid(4., 4.),
                sensor: Sensor,
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints,
            },
            "Switch" => SensorBundle {
                collider: Collider::cuboid(4., 4.),
                sensor: Sensor,
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints,
            },
            _ => unreachable!(),
        }
    }
}

// TODO: set collision groups, collision filters
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub ground_detection: GroundDetection,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct StarBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    sensor_bundle: SensorBundle,
    star: Star,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SwitchBundle {
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    sensor_bundle: SensorBundle,
    switch: Switch,
    related_door: RelatedDoor,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    collider_bundle: ColliderBundle,
    door: Door,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct UiDataBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    ui_data: UiData,
    #[from_entity_instance]
    buttons: UiButton,
    #[from_entity_instance]
    text: UiText,
}
