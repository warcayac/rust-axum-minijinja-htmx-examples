use std::{collections::HashMap, fs::File, io::Read};

use axum::Router;
use once_cell::sync::Lazy;

use super::{domain::entities::Routes, presentation::{api::*, views::*}, TAppStateRouter};


pub static APP_ROUTES: Lazy<HashMap<String, Routes>> = Lazy::new(|| {
    let mut file = File::open("data/routes.json").unwrap();
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).unwrap();

    let json_routes = serde_json::from_str::<Vec<Routes>>(&json_data).unwrap();
    let mut map = HashMap::new();

    for route in json_routes {
        let key = route.get_alias();
        map.insert(key, route);
    }

    map
});

pub fn routes() -> TAppStateRouter {
    Router::new()
        .merge(home_routes())
        .nest("/api", info_routes())
        .nest(&APP_ROUTES["click2edit"].get_base(), click_to_edit_routes())
        .nest(&APP_ROUTES["bulkupdate"].get_base(), bulk_update_routes())
        .nest(&APP_ROUTES["click2load"].get_base(), click_to_load_routes())
        .nest(&APP_ROUTES["deleterow"].get_base(), delete_row_routes())
        .nest(&APP_ROUTES["editrow"].get_base(), edit_row_routes())
        .nest(&APP_ROUTES["lazyload"].get_base(), lazy_load_routes())
        .nest(&APP_ROUTES["inlineval"].get_base(), inline_validation_routes())
        .nest(&APP_ROUTES["infinitescroll"].get_base(), infinite_scroll_routes())
        .nest(&APP_ROUTES["activesearch"].get_base(), active_search_routes())
        .nest(&APP_ROUTES["progressbar"].get_base(), progress_bar_routes())
        .nest(&APP_ROUTES["valueselect"].get_base(), value_select_routes())
        .nest(&APP_ROUTES["animations"].get_base(), animations_routes())
        .nest(&APP_ROUTES["fileupload"].get_base(), file_upload_routes())
        .nest(&APP_ROUTES["fileuploadinput"].get_base(), file_upload_input_routes())
        .nest(&APP_ROUTES["resetuserinput"].get_base(), reset_user_input_routes())
        .nest(&APP_ROUTES["dialogs"].get_base(), dialogs_routes())
        .nest(&APP_ROUTES["dialogsuikit"].get_base(), dialogs_uikit_routes())
        .nest(&APP_ROUTES["modalbootstrap"].get_base(), modal_boostrap_routes())
        .nest(&APP_ROUTES["modalcustom"].get_base(), modal_custom_routes())
        .nest(&APP_ROUTES["tabshateoas"].get_base(), tabs_hateoas_routes())
        .nest(&APP_ROUTES["tabsjavascript"].get_base(), tabs_javascript_routes())
        .nest(&APP_ROUTES["kbshortcuts"].get_base(), keyboard_shortcuts_routes())
        .nest(&APP_ROUTES["sortable"].get_base(), sortable_routes())
        .nest(&APP_ROUTES["updateother"].get_base(), update_other_content_routes())
        .nest(&APP_ROUTES["confirm"].get_base(), confirm_routes())
        .nest(&APP_ROUTES["asyncauth"].get_base(), async_auth_routes())
        .nest(&APP_ROUTES["webcomponents"].get_base(), web_components_routes())
        .nest(&APP_ROUTES["movebefore"].get_base(), move_before_routes())
}
