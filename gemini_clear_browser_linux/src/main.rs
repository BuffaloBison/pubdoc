use gtk::prelude::*;
use webkit2gtk::{
    WebContext, WebView, WebViewExt, WebsiteDataManagerExtManual, WebsiteDataTypes,
};

const HOME_URL: &str = "https://www.startpage.com";
const APP_TITLE: &str = "Gemini Clear Browser";

fn ensure_scheme(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(APP_TITLE);
    window.set_default_size(1024, 768);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // --- toolbar ---
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    toolbar.set_margin_start(4);
    toolbar.set_margin_end(4);
    toolbar.set_margin_top(4);
    toolbar.set_margin_bottom(4);

    let back_btn = gtk::Button::with_label("\u{25C0}");
    back_btn.set_tooltip_text(Some("Back"));
    let forward_btn = gtk::Button::with_label("\u{25B6}");
    forward_btn.set_tooltip_text(Some("Forward"));

    let address_bar = gtk::Entry::new();
    address_bar.set_hexpand(true);
    address_bar.set_placeholder_text(Some("Enter URL\u{2026}"));

    let go_btn = gtk::Button::with_label("Go");
    let clear_btn = gtk::Button::with_label("CLEAR");

    toolbar.pack_start(&back_btn, false, false, 0);
    toolbar.pack_start(&forward_btn, false, false, 0);
    toolbar.pack_start(&address_bar, true, true, 0);
    toolbar.pack_start(&go_btn, false, false, 0);
    toolbar.pack_start(&clear_btn, false, false, 0);

    // --- webview ---
    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    webview.set_vexpand(true);
    webview.set_hexpand(true);
    webview.load_uri(HOME_URL);

    // Keep address bar in sync with page URL
    {
        let address_bar = address_bar.clone();
        webview.connect_uri_notify(move |wv| {
            if let Some(uri) = wv.uri() {
                address_bar.set_text(&uri);
            }
        });
    }

    // Update window title from page title
    {
        let window = window.clone();
        webview.connect_title_notify(move |wv| {
            let page_title = wv.title().map(|t| t.to_string()).unwrap_or_default();
            if page_title.is_empty() {
                window.set_title(APP_TITLE);
            } else {
                window.set_title(&format!("{page_title} \u{2014} {APP_TITLE}"));
            }
        });
    }

    // Back button
    {
        let webview = webview.clone();
        back_btn.connect_clicked(move |_| {
            if webview.can_go_back() {
                webview.go_back();
            }
        });
    }

    // Forward button
    {
        let webview = webview.clone();
        forward_btn.connect_clicked(move |_| {
            if webview.can_go_forward() {
                webview.go_forward();
            }
        });
    }

    // Navigate when Enter is pressed in the address bar
    {
        let webview = webview.clone();
        address_bar.connect_activate(move |entry| {
            let url = ensure_scheme(&entry.text());
            webview.load_uri(&url);
        });
    }

    // Go button
    {
        let webview = webview.clone();
        let address_bar = address_bar.clone();
        go_btn.connect_clicked(move |_| {
            let url = ensure_scheme(&address_bar.text());
            webview.load_uri(&url);
        });
    }

    // Clear button — wipe cookies, cache, local storage, etc.
    {
        let webview = webview.clone();
        clear_btn.connect_clicked(move |_| {
            if let Some(manager) = webview.website_data_manager() {
                let types = WebsiteDataTypes::COOKIES
                    | WebsiteDataTypes::DISK_CACHE
                    | WebsiteDataTypes::LOCAL_STORAGE
                    | WebsiteDataTypes::MEMORY_CACHE
                    | WebsiteDataTypes::OFFLINE_APPLICATION_CACHE;

                manager.clear(
                    types,
                    glib::TimeSpan::from_seconds(0),
                    None::<&gtk::gio::Cancellable>,
                    |_result| {},
                );
            }

            webview.load_html(
                "<html><body><center><h1>Data Cleared Successfully</h1>\
                 <p>History and Cookies have been deleted.</p></center></body></html>",
                None,
            );
        });
    }

    vbox.pack_start(&toolbar, false, false, 0);
    vbox.pack_start(&webview, true, true, 0);
    window.add(&vbox);
    window.show_all();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.gemini.clear-browser")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
