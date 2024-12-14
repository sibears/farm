use crate::domain::config::entities::Config;
use crate::domain::flags::entities::{Flag, NewFlag};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::flags::controllers::get_flags,
        crate::presentation::flags::controllers::post_flag,
        crate::presentation::flags::controllers::post_flags,
        crate::presentation::sending::controllers::get_flags_for_senders,
        crate::presentation::sending::controllers::force_update_waiting_flags,
        crate::presentation::sending::controllers::update_flags_from_sending,
        crate::presentation::config::controllers::get_config,
    ),
    components(schemas(Flag, NewFlag, Config)),
    tags(
        (name = "flags", description = "Flag management endpoints"),
        (name = "sending", description = "Flag sending endpoints"),
        (name = "config", description = "Configuration endpoints")
    ),
    info(
        title = "Sibears Farm API",
        version = "1.0.3",
        description = "API for managing and sending flags"
    )
)]
pub struct ApiDoc;
