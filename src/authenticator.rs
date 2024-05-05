pub mod authenticator {

    use cognite::AuthenticatorConfig;
    use cognite::CogniteClient;

    pub fn get_client() -> CogniteClient {
        let env_vars = std::fs::read_to_string(".\\src\\env_vars.txt").unwrap();
        let credentials = env_vars.lines().collect::<Vec<&str>>();
        let mut builder = CogniteClient::builder();
        builder
            .set_oidc_credentials(AuthenticatorConfig {
                client_id: credentials[0].to_string(),
                token_url:
                    "https://login.microsoftonline.com/celanese.onmicrosoft.com/oauth2/v2.0/token"
                        .to_string(),
                secret: credentials[1].to_string(),
                resource: None,
                audience: None,
                scopes: Some("https://az-eastus-1.cognitedata.com/.default".to_string()),
            })
            .set_project("celanese")
            .set_app_name("SDK")
            .set_base_url("https://az-eastus-1.cognitedata.com");
        let cognite_client = match builder.build() {
            Ok(client) => client,
            Err(e) => {
                panic!("Error creating client: {:?}", e);
            }
        };
        cognite_client
    }
}
