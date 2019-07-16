use super::*;

use std::fmt;

use serde::{Deserialize, Serialize, Serializer};

use specs::{
    error::NoError,
    saveload::{ConvertSaveload, DeserializeComponents, SerializeComponents},
};

use components::*;
use resources::*;

macro_rules! serde_resources {
    (
        $ser_struct_name:ident, $deser_struct_name:ident,
        resources: [ $( $name:ident : $kind:ident ),* $(,)* ]
    ) => {
        #[derive(SystemData)]
        pub struct $ser_struct_name<'a> {
            $(
                $name: Read<'a, $kind>,
            )*
        }

        // name will be mangled, but it's only used inside the macro, so it's fine
        #[derive(Serialize)]
        struct ActuallySerializedStruct<'a> {
            $(
                $name: &'a $kind,
            )*
        }

        impl<'a> Serialize for $ser_struct_name<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let to_serialize = ActuallySerializedStruct {
                    $(
                        $name: &self.$name,
                    )*
                };

                to_serialize.serialize(serializer)
            }
        }

        #[derive(SystemData)]
        pub struct $deser_struct_name<'a> {
            $(
                $name: Write<'a, $kind>,
            )*
        }

        #[derive(Deserialize)]
        struct ActuallyDeserializedStruct {
            $(
                $name: $kind,
            )*
        }

        impl <'a> $deser_struct_name<'a> {
            fn load_from(&mut self, deser: ActuallyDeserializedStruct) {
                $(
                    *self.$name = deser.$name;
                )*
            }
        }
    };
}

macro_rules! serde_components {
    (
        $ser_struct_name:ident, $deser_struct_name:ident, $marker_name:tt,
        components: [ $( $name:ident : $kind:ident ),* $(,)* ]
    ) => {
        // Define the serialization components struct
        #[derive(SystemData)]
        pub struct $ser_struct_name<'a> (
            $(
                ReadStorage<'a, $kind>,
            )*
        );

        impl<'a> SerializeComponents<NoError, $marker_name> for $ser_struct_name<'a> {
            type Data = (
                $(
                    Option<<$kind as ConvertSaveload<$marker_name>>::Data>,
                )*
            );

            fn serialize_entity<F>(&self, entity: Entity, mut ids: F) -> Result<Self::Data, NoError>
            where
                F: FnMut(Entity) -> Option<$marker_name>,
            {
                let $ser_struct_name( $( ref $name , )* ) = *self;

                Ok((
                    $(
                        $name.get(entity).map(|c| c.convert_into(&mut ids).map(Some)).unwrap_or(Ok(None))?,
                    )*
                ))
            }
        }

        // Define the deserialization components struct
        #[derive(SystemData)]
        pub struct $deser_struct_name<'a> {
            $(
                $name: WriteStorage<'a, $kind>,
            )*
        }

        impl<'a> DeserializeComponents<CombinedError, $marker_name> for $deser_struct_name<'a> {
            type Data = (
                $(
                    Option<<$kind as ConvertSaveload<$marker_name>>::Data>,
                )*
            );

            fn deserialize_entity<F>(
                &mut self,
                entity: Entity,
                components: Self::Data,
                mut ids: F
            ) -> Result<(), CombinedError>
            where
                F: FnMut($marker_name) -> Option<Entity>
            {
                let ( $($name,)* ) = components;

                $(
                    if let Some(component) = $name {
                        self.$name.insert(entity, ConvertSaveload::<$marker_name>::convert_from(component, &mut ids)?)?;
                    } else {
                        self.$name.remove(entity);
                    }
                )*

                Ok(())
            }
        }
    };
}

#[derive(Debug)]
enum CombinedError {
    Cbor(serde_cbor::error::Error),
    Specs(specs::error::Error),
}

impl fmt::Display for CombinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CombinedError::Cbor(ref e) => write!(f, "{}", e),
            CombinedError::Specs(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<specs::error::Error> for CombinedError {
    fn from(x: specs::error::Error) -> Self {
        CombinedError::Specs(x)
    }
}

impl From<serde_cbor::error::Error> for CombinedError {
    fn from(x: serde_cbor::error::Error) -> Self {
        CombinedError::Cbor(x)
    }
}

impl From<NoError> for CombinedError {
    fn from(x: NoError) -> Self {
        // NoError is an enum with no variants, so cannot be constructed
        // this is used to make certain other things compile (specs uses NoError
        // for things which can't actually fail, but which need to be Result for
        // some other reason)
        match x {}
    }
}

pub struct SerializeSystem;

#[derive(SystemData)]
pub struct SerializeSystemData<'a> {
    entities: Entities<'a>,
    components: SerializeSystemComponentsM<'a>,
    resources: SerializeResourcesM<'a>,
    marker: ReadStorage<'a, SaveComponent>,
    saves: Write<'a, resources::SavedStates>,
}

serde_components! (
    SerializeSystemComponentsM, DeserializeSystemComponentsM, SaveComponent,
    // TOOD: use structs more carefully so we don't need two names
    components: [
        has_position: HasPosition,
        ivt: ImaginaryVisibleTile,
        bm: BlocksMovement,
        bv: BlocksVisibility,
        visible: Visible,
        hackable: Hackable,
        char_render: CharRender,
        player: Player,
        oc: OxygenContainer,
        breathes: Breathes,
        cs: CanSuffocate,
        vacuum: Vacuum,
        door: Door,
        od: OpensDoors,
        camera: Camera,
        npc: NPC,
    ]
    // TODO: have a resources aspect to this, too
);

serde_resources! (
    SerializeResourcesM, DeserializeResourcesM,
    resources: [
        game_clock: GameClock,
        npc_moves: NpcMoves,
    ]
);

impl<'a> System<'a> for SerializeSystem {
    type SystemData = SerializeSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.saves.save_requested {
            // TODO: this is too pretty, lots of wasted space, that costs RAM
            // TODO: might need compression at some point
            let mut ser = ron::ser::Serializer::new(Some(Default::default()), true);

            SerializeComponents::<NoError, SaveComponent>::serialize(&data.components, &data.entities, &data.marker, &mut ser)
                .unwrap_or_else(|e| eprintln!("Serializing worldstate error: {}", e));

            let saved_components = ser.into_output_string();

            let resource_bytes = ron::ser::to_string(&data.resources)
                .expect("Should be able to serialize resources")
                .into_bytes();

            data.saves.saves.push(resources::SaveGameData {
                world_state: saved_components.into_bytes(),
                resources: resource_bytes,
            });

            data.saves.save_requested = false;
        }
    }
}

pub struct DeserializeSystem;

#[derive(SystemData)]
pub struct DeserializeSystemData<'a> {
    entities: Entities<'a>,
    components: DeserializeSystemComponentsM<'a>,
    marker: WriteStorage<'a, SaveComponent>,
    resources: DeserializeResourcesM<'a>,
    allocator: Write<'a, SaveComponentAllocator>,

    saves: Write<'a, resources::SavedStates>,
    render_stale: Write<'a, resources::RenderStale>,
}

impl<'a> System<'a> for DeserializeSystem {
    type SystemData = DeserializeSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use ron::de::Deserializer;

        if data.saves.load_requested {
            let num_saves = data.saves.saves.len();

            if num_saves > 0 {
                let top_save = data.saves.saves.get(num_saves - 1).unwrap();

                // TODO: handle "load" failures better
                let mut de = Deserializer::from_bytes(&top_save.world_state).expect("Deserializer should be able to be instantiated");
                DeserializeComponents::<CombinedError, _>::deserialize(
                    &mut data.components,
                    &data.entities,
                    &mut data.marker,
                    &mut data.allocator,
                    &mut de,
                )
                .unwrap_or_else(|e| eprintln!("Error: {}", e));

                data.resources
                    .load_from(ron::de::from_bytes(&top_save.resources).expect("Deserialization should succeed"));
            } else {
                eprintln!("Load requested, but no save game data is present");
            }

            data.saves.load_requested = false;

            data.render_stale.0 = true;
        }
    }
}
