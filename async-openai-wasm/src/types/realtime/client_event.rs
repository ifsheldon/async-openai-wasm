use serde::{Deserialize, Serialize};
use super::{item::Item, session_resource::SessionResource};

///
/// A trait same as Into<String>
///
/// For converting event structs into text part of WS message
///
pub trait ToText {
    fn to_text(self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionUpdateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Session configuration to update.
    pub session: SessionResource,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferAppendEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Base64-encoded audio bytes.
    pub audio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferCommitEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferClearEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemCreateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the preceding item after which the new item will be inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,

    /// The item to add to the conversation.
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemTruncateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the assistant message item to truncate.
    pub item_id: String,

    /// The index of the content part to truncate.
    pub content_index: u32,

    /// Inclusive duration up to which audio is truncated, in milliseconds.        
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemDeleteEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the item to delete.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCreateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// Configuration for the response.
    pub response: Option<SessionResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCancelEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// These are events that the OpenAI Realtime WebSocket server will accept from the client.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientEvent {
    /// Send this event to update the session’s default configuration.
    #[serde(rename = "session.update")]
    SessionUpdate(SessionUpdateEvent),

    /// Send this event to append audio bytes to the input audio buffer.
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(InputAudioBufferAppendEvent),

    /// Send this event to commit audio bytes to a user message.
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(InputAudioBufferCommitEvent),

    /// Send this event to clear the audio bytes in the buffer.
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(InputAudioBufferClearEvent),

    /// Send this event when adding an item to the conversation.
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(ConversationItemCreateEvent),

    /// Send this event when you want to truncate a previous assistant message’s audio.
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(ConversationItemTruncateEvent),

    /// Send this event when you want to remove any item from the conversation history.
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(ConversationItemDeleteEvent),

    /// Send this event to trigger a response generation.
    #[serde(rename = "response.create")]
    ResponseCreate(ResponseCreateEvent),

    /// Send this event to cancel an in-progress response.
    #[serde(rename = "response.cancel")]
    ResponseCancel(ResponseCancelEvent),
}

impl Into<String> for &ClientEvent {
    fn into(self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

macro_rules! event_struct_to_variant {
    ($from_typ:ty, $evt_typ:ty, $variant:ident) => {
        impl From<$from_typ> for $evt_typ {
            fn from(value: $from_typ) -> Self {
                <$evt_typ>::$variant(value)
            }
        }
    };
}

event_struct_to_variant!(SessionUpdateEvent, ClientEvent, SessionUpdate);
event_struct_to_variant!(
    InputAudioBufferAppendEvent,
    ClientEvent,
    InputAudioBufferAppend
);
event_struct_to_variant!(
    InputAudioBufferCommitEvent,
    ClientEvent,
    InputAudioBufferCommit
);
event_struct_to_variant!(
    InputAudioBufferClearEvent,
    ClientEvent,
    InputAudioBufferClear
);
event_struct_to_variant!(
    ConversationItemCreateEvent,
    ClientEvent,
    ConversationItemCreate
);
event_struct_to_variant!(
    ConversationItemTruncateEvent,
    ClientEvent,
    ConversationItemTruncate
);
event_struct_to_variant!(
    ConversationItemDeleteEvent,
    ClientEvent,
    ConversationItemDelete
);
event_struct_to_variant!(ResponseCreateEvent, ClientEvent, ResponseCreate);
event_struct_to_variant!(ResponseCancelEvent, ClientEvent, ResponseCancel);

impl<T: Into<ClientEvent>> ToText for T {
    // blanket impl for all client event structs
    fn to_text(self) -> String {
        (&self.into()).into()
    }
}

impl From<Item> for ConversationItemCreateEvent {
    fn from(value: Item) -> Self {
        Self {
            event_id: None,
            previous_item_id: None,
            item: value,
        }
    }
}
