use crate::settings::{Config, AuthConfig, CtfConfig, ProtocolConfig};


pub fn get_config() -> Config {
    Config::new(
        AuthConfig::new(
           "sibears1cool"
        ),
        CtfConfig::new(
            ProtocolConfig::new(
                "test_proto", 
                "test_token", 
                "test_checksys_host", 
                1337
            ),
            "\\w{31}=",
            5*60,
            2,
            100,
            vec!["team1", "team2"]
        )
    )
}