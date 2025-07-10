mod client;
mod command;
mod deserialize;
mod device;
mod event;
mod event_sender;
mod execute;
mod link;
mod media_class;
mod metadata;
mod node;
mod object_id;
mod property_store;
mod proxy_registry;
mod session;
pub mod state;
mod stream;
mod stream_registry;
mod sync_registry;

pub use command::CommandSender;
pub use event::{Event, StateEvent};
pub use event_sender::EventHandler;
pub use object_id::ObjectId;
pub use property_store::PropertyStore;
pub use session::Session;

#[cfg(test)]
mod mock {
    use crate::{CommandSender, ObjectId};

    #[derive(Default)]
    pub struct WirehoseHandle {}

    impl CommandSender for WirehoseHandle {
        fn node_capture_start(
            &self,
            _obj_id: ObjectId,
            _object_serial: u64,
            _capture_sink: bool,
        ) {
        }
        fn node_capture_stop(&self, _obj_id: ObjectId) {}
        fn node_mute(&self, _obj_id: ObjectId, _mute: bool) {}
        fn node_volumes(&self, _obj_id: ObjectId, _volumes: Vec<f32>) {}
        fn device_mute(
            &self,
            _obj_id: ObjectId,
            _route_index: i32,
            _route_device: i32,
            _mute: bool,
        ) {
        }
        fn device_set_profile(&self, _obj_id: ObjectId, _profile_index: i32) {}
        fn device_set_route(
            &self,
            _obj_id: ObjectId,
            _route_index: i32,
            _route_device: i32,
        ) {
        }
        fn device_volumes(
            &self,
            _obj_id: ObjectId,
            _route_index: i32,
            _route_device: i32,
            _volumes: Vec<f32>,
        ) {
        }
        fn metadata_set_property(
            &self,
            _obj_id: ObjectId,
            _subject: u32,
            _key: String,
            _type_: Option<String>,
            _value: Option<String>,
        ) {
        }
    }
}
