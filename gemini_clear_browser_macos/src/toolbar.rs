use cacao::objc::{msg_send, sel, sel_impl};

use cacao::button::Button;
use cacao::input::{TextField, TextFieldDelegate};

use cacao::appkit::toolbar::{
    ItemIdentifier, Toolbar, ToolbarDelegate, ToolbarDisplayMode, ToolbarItem,
};

use super::Action;

const BACK_BUTTON: &str = "BackButton";
const FWDS_BUTTON: &str = "FwdsButton";
const URL_BAR: &str = "URLBar";
const CLEAR_BUTTON: &str = "ClearButton";

fn ensure_scheme(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}

#[derive(Debug)]
pub struct URLBar;

impl TextFieldDelegate for URLBar {
    const NAME: &'static str = "URLBar";

    fn text_did_end_editing(&self, value: &str) {
        let url = ensure_scheme(value);
        Action::Load(url).dispatch();
    }
}

#[derive(Debug)]
pub struct BrowserToolbar {
    back_item: ToolbarItem,
    forwards_item: ToolbarItem,
    url_bar: TextField<URLBar>,
    url_bar_item: ToolbarItem,
    clear_item: ToolbarItem,
}

impl BrowserToolbar {
    pub fn new() -> Self {
        let back_button = Button::new("\u{25C0}");
        let mut back_item = ToolbarItem::new(BACK_BUTTON);
        back_item.set_button(back_button);
        back_item.set_action(|| Action::Back.dispatch());

        let forwards_button = Button::new("\u{25B6}");
        let mut forwards_item = ToolbarItem::new(FWDS_BUTTON);
        forwards_item.set_button(forwards_button);
        forwards_item.set_action(|| Action::Forward.dispatch());

        let url_bar = TextField::with(URLBar);
        let url_bar_item = ToolbarItem::new(URL_BAR);

        url_bar.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*url_bar_item.objc, setView:&*obj];
        });

        let clear_button = Button::new("CLEAR");
        let mut clear_item = ToolbarItem::new(CLEAR_BUTTON);
        clear_item.set_button(clear_button);
        clear_item.set_action(|| Action::ClearData.dispatch());

        BrowserToolbar {
            back_item,
            forwards_item,
            url_bar,
            url_bar_item,
            clear_item,
        }
    }

    pub fn set_url(&self, url: &str) {
        self.url_bar.set_text(url);
    }

    fn item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![
            ItemIdentifier::Custom(BACK_BUTTON),
            ItemIdentifier::Custom(FWDS_BUTTON),
            ItemIdentifier::Space,
            ItemIdentifier::Custom(URL_BAR),
            ItemIdentifier::Space,
            ItemIdentifier::Custom(CLEAR_BUTTON),
        ]
    }
}

impl ToolbarDelegate for BrowserToolbar {
    const NAME: &'static str = "GeminiToolbar";

    fn did_load(&mut self, toolbar: Toolbar) {
        toolbar.set_display_mode(ToolbarDisplayMode::IconOnly);
    }

    fn allowed_item_identifiers(&self) -> Vec<ItemIdentifier> {
        self.item_identifiers()
    }

    fn default_item_identifiers(&self) -> Vec<ItemIdentifier> {
        self.item_identifiers()
    }

    fn item_for(&self, identifier: &str) -> &ToolbarItem {
        match identifier {
            BACK_BUTTON => &self.back_item,
            FWDS_BUTTON => &self.forwards_item,
            URL_BAR => &self.url_bar_item,
            CLEAR_BUTTON => &self.clear_item,
            _ => {
                std::unreachable!();
            }
        }
    }
}
