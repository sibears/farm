use crate::settings::{Config, AuthConfig, CtfConfig, ProtocolConfig};


// TODO: Зарефакторить конфиги в удобную структуру для редактирования, мб добавить макросы для этого
pub fn get_config() -> Config {
    Config::new(
        AuthConfig::new(
           "sibears1cool"
        ),
        CtfConfig::new(
            ProtocolConfig::new(
                "test_proto", 
                "23d81685fb77150f", 
                "forkad.docker.localhost", 
                80
            ),
            "\\w{31}=",
            5*60,
            5,
            100,
            vec!["team1", "team2"]
        )
    )
}