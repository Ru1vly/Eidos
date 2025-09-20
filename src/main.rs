use lib_bridge::{Bridge, Request};
use lib_chat::Chat;
use lib_core::Core;
use lib_translate::Translate;

fn main() {
    let mut bridge = Bridge::new();

    bridge.register(Request::Chat, Box::new(|| Chat::new().run()));
    bridge.register(Request::Core, Box::new(|| Core::new().run()));
    bridge.register(Request::Translate, Box::new(|| Translate::new().run()));

    bridge.route(Request::Chat);
    bridge.route(Request::Core);
    bridge.route(Request::Translate);
}