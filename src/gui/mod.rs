use adw::prelude::*;
use relm4::prelude::*;

pub struct App {
    db: rusqlite::Connection,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = rusqlite::Connection;
    type Input = ();
    type Output = ();

    view! {
        adw::Window {
            set_width_request: 640,
            set_height_request: 480,
            add_css_class: "devel",
        }
    }

    fn init (
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { db: params };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
