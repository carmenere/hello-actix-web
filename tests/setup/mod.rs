use actix_web::{web};

use hello_actix as lib;
use lib::state::AppData;


pub async fn app_data() -> web::Data<AppData> {
    web::Data::new(AppData::new().await)
}
