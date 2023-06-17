use adw::prelude::ApplicationExt;
use relm4::prelude::*;

use deck_editor::data::{app_id, dirs};
use deck_editor::gui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dirs::init()?;

    let db = rusqlite::Connection::open(dirs::DB_FILE.as_path())?;
    deck_editor::db::update(&db)?;

    let main_app = relm4::main_application();
    main_app.set_application_id(Some(app_id::DOT_SEPARATED.as_str()));
    main_app.set_resource_base_path(Some(app_id::SLASH_SEPARATED.as_str()));

    let relm_app = RelmApp::from_app(main_app);
    relm_app.run::<App>(db);

    Ok(())
}
