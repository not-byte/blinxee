use candid::{CandidType, Deserialize, Principal};
use ic_cdk::trap;
use std::{cell::RefCell, collections::HashMap};

// TODO: Add Image type
// NOTE: Id can be changed to uuid
#[derive(CandidType, Deserialize, Clone)]
struct Message {
    id: u64,
    sender: Principal,
    receiver: Principal,
    content: String,
    timestamp: u64,
}

type Conversation = Vec<Message>;

#[derive(Default, CandidType, Deserialize, Clone)]
struct State {
    conversations: HashMap<(Principal, Principal), Conversation>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[ic_cdk::update]
fn send_message(receiver: Principal, content: String) {
    let sender = ic_cdk::caller();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    // TODO: Add proper id selection
    let message = Message {
        id: 0,
        sender,
        receiver,
        content,
        timestamp,
    };

    STATE.with_borrow_mut(|state| {
        state
            .conversations
            .entry((sender, receiver))
            .or_default()
            .push(message);
    })
}

#[ic_cdk::query]
fn get_messages_with(user: Principal) -> Conversation {
    let caller = ic_cdk::caller();
    STATE.with_borrow(|state| {
        state.conversations.get(&(caller, user)).cloned().unwrap_or(
            state
                .conversations
                .get(&(user, caller))
                .cloned()
                .unwrap_or_default(),
        )
    })
}

#[ic_cdk::update]
fn remove_message(receiver: Principal, id: u64) {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        if let Some(conversation) = state.conversations.get_mut(&(caller, receiver)) {
            let Some(index) = conversation.iter().position(|v| v.id == id) else {
                trap(r#"{"message": "Message not found"}"#);
            };
            conversation.remove(index);
        }
    })
}

#[ic_cdk::update]
fn update_message(receiver: Principal, id: u64, new_message: String) {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        if let Some(conversation) = state.conversations.get_mut(&(caller, receiver)) {
            let Some(index) = conversation.iter().position(|v| v.id == id) else {
                trap(r#"{"message": "Message not found"}"#);
            };
            if let Some(v) = conversation.get_mut(index) {
                v.content = new_message;
            }
        }
    })
}

// NOTE: Store conversations on stable storage
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with_borrow(|state| {
        ic_cdk::storage::stable_save((state.clone(),)).unwrap();
    });
}

// NOTE: Get conversations from stable storage
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with_borrow_mut(|state| {
        if let Ok((new_state,)) = ic_cdk::storage::stable_restore::<(State,)>() {
            *state = new_state;
        }
    });
}

// NOTE: Remove in prod
#[ic_cdk::query]
fn get_all() -> HashMap<(Principal, Principal), Vec<Message>> {
    let caller = ic_cdk::caller();
    ic_cdk::println!("{:#?}", caller);
    STATE.with_borrow(|state| {
        return state.conversations.clone();
    })
}
