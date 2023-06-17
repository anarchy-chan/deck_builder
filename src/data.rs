pub const APP_ID_PARTS: (&str, &str, &str) = ("com", "anarchy_chan", "DeckEditor");

pub mod app_id {
    use once_cell::sync::Lazy;

    use super::APP_ID_PARTS as ID;

    pub static DOT_SEPARATED: Lazy<String> = Lazy::new(|| format!("{}.{}.{}", ID.0, ID.1, ID.2));
    pub static SLASH_SEPARATED: Lazy<String> =
        Lazy::new(|| format!("/{}/{}/{}/", ID.0, ID.1, ID.2));
}

pub mod dirs {
    use std::fs;
    use std::path::PathBuf;

    use directories::ProjectDirs;
    use once_cell::sync::Lazy;

    use super::APP_ID_PARTS as ID;

    pub static ROOT: Lazy<PathBuf> = Lazy::new(|| {
        ProjectDirs::from(ID.0, ID.1, ID.2)
            .expect("Failed to initialise project directories.")
            .data_dir()
            .to_path_buf()
    });

    pub static USER: Lazy<PathBuf> = Lazy::new(|| ROOT.join("user"));

    pub static DB_FILE: Lazy<PathBuf> = Lazy::new(|| ROOT.join("data.db"));

    macro_rules! create_lazy_dirs {
        ( $( $i:ident ),* ) => {
            $(
                fs::create_dir_all($i.as_path())?;
            )*
        }
    }

    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        create_lazy_dirs!(ROOT, USER);

        Ok(())
    }
}
