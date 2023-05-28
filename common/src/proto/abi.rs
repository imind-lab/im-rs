#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRequest {
    #[prost(oneof = "message_request::Inner", tags = "1, 2, 3, 4, 99")]
    pub inner: ::core::option::Option<message_request::Inner>,
}
/// Nested message and enum types in `MessageRequest`.
pub mod message_request {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Connect(super::Connect),
        #[prost(message, tag = "2")]
        Ping(super::Ping),
        #[prost(message, tag = "3")]
        Text(super::Text),
        #[prost(message, tag = "4")]
        Textack(super::Textack),
        #[prost(message, tag = "99")]
        Disconnect(super::Disconnect),
    }
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageResponse {
    #[prost(oneof = "message_response::Inner", tags = "1, 2, 3, 4, 99")]
    pub inner: ::core::option::Option<message_response::Inner>,
}
/// Nested message and enum types in `MessageResponse`.
pub mod message_response {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Connack(super::Connack),
        #[prost(message, tag = "2")]
        Pong(super::Pong),
        #[prost(message, tag = "3")]
        Text(super::Text),
        #[prost(message, tag = "4")]
        Textack(super::Textack),
        #[prost(message, tag = "99")]
        Disconnect(super::Disconnect),
    }
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connect {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub keep_alive: u32,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connack {
    #[prost(int32, tag = "1")]
    pub code: i32,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disconnect {}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    #[prost(uint64, tag = "1")]
    pub message_id: u64,
    #[prost(uint32, tag = "2")]
    pub sender_id: u32,
    #[prost(uint32, tag = "3")]
    pub receiver_id: u32,
    #[prost(uint32, tag = "4")]
    pub message_type: u32,
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Textack {
    #[prost(uint64, tag = "1")]
    pub message_id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
    #[prost(uint32, tag = "3")]
    pub status: u32,
}
