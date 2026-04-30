use cacao::notification_center::Dispatcher;
use cacao::webview::{WebView, WebViewConfig, WebViewDelegate};

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::toolbar::Toolbar;
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate, WindowToolbarStyle};
use cacao::appkit::{App, AppDelegate};

mod toolbar;
use toolbar::BrowserToolbar;

pub const HOME_URL: &str = "https://www.startpage.com";
pub const APP_TITLE: &str = "Gemini Clear Browser";

#[derive(Debug)]
pub enum Action {
    Back,
    Forward,
    Load(String),
    ClearData,
}

impl Action {
    pub fn dispatch(self) {
        App::<BasicApp, Self>::dispatch_main(self);
    }
}

struct BasicApp {
    window: Window<AppWindow>,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new("", vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit,
            ]),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new("Edit", vec![
                MenuItem::Undo,
                MenuItem::Redo,
                MenuItem::Separator,
                MenuItem::Cut,
                MenuItem::Copy,
                MenuItem::Paste,
                MenuItem::Separator,
                MenuItem::SelectAll,
            ]),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front"),
            ]),
            Menu::new("Help", vec![]),
        ]);

        App::activate();
        self.window.show();
    }
}

impl Dispatcher for BasicApp {
    type Message = Action;

    fn on_ui_message(&self, message: Self::Message) {
        let window = self.window.delegate.as_ref().unwrap();

        match message {
            Action::Back => {
                window.content.go_back();
            }
            Action::Forward => {
                window.content.go_forward();
            }
            Action::Load(url) => {
                window.load_url(&url);
            }
            Action::ClearData => {
                clear_website_data();
                window.content.load_html(
                    "<html><body style=\"font-family:-apple-system,Helvetica,Arial,sans-serif;\
                     text-align:center;padding-top:80px;\">\
                     <h1>Data Cleared Successfully</h1>\
                     <p>Cookies, cache, and local storage have been deleted.</p>\
                     </body></html>",
                );
                window.toolbar.delegate.as_ref().unwrap().set_url("");
            }
        }
    }
}

/// Clears all website data (cookies, cache, local storage) via the default
/// WKWebsiteDataStore, using Objective-C runtime messaging.
fn clear_website_data() {
    use cacao::objc::runtime::Class;
    use cacao::objc::{msg_send, sel, sel_impl};
    use block::ConcreteBlock;

    unsafe {
        let wk_cls = Class::get("WKWebsiteDataStore").unwrap();
        let store: *mut cacao::objc::runtime::Object = msg_send![wk_cls, defaultDataStore];
        let all_types: *mut cacao::objc::runtime::Object =
            msg_send![wk_cls, allWebsiteDataTypes];

        let ns_date = Class::get("NSDate").unwrap();
        let epoch: *mut cacao::objc::runtime::Object = msg_send![ns_date, distantPast];

        let handler = ConcreteBlock::new(|| {});
        let handler = handler.copy();

        let _: () = msg_send![
            store,
            removeDataOfTypes: all_types
            modifiedSince: epoch
            completionHandler: &*handler
        ];
    }
}

#[derive(Default)]
pub struct WebViewInstance;

impl WebViewDelegate for WebViewInstance {}

pub struct AppWindow {
    toolbar: Toolbar<BrowserToolbar>,
    content: WebView<WebViewInstance>,
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            toolbar: Toolbar::new(
                "com.gemini.clear-browser.toolbar",
                BrowserToolbar::new(),
            ),
            content: WebView::with(WebViewConfig::default(), WebViewInstance::default()),
        }
    }

    pub fn load_url(&self, url: &str) {
        self.toolbar.delegate.as_ref().unwrap().set_url(url);
        self.content.load_url(url);
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "GeminiWindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title(APP_TITLE);
        window.set_autosave_name("GeminiClearBrowser");
        window.set_minimum_content_size(800., 600.);

        window.set_toolbar(&self.toolbar);
        window.set_content_view(&self.content);

        self.load_url(HOME_URL);
    }
}

fn main() {
    App::new(
        "com.gemini.clear-browser",
        BasicApp {
            window: Window::with(
                {
                    let mut config = WindowConfig::default();
                    config.toolbar_style = WindowToolbarStyle::Expanded;
                    config
                },
                AppWindow::new(),
            ),
        },
    )
    .run();
}
