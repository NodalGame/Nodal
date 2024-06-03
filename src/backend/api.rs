pub mod api {
    use std::error::Error;

    use bevy::ecs::system::Resource;
    use reqwest::blocking::Client as BlockingClient;
    use reqwest::Client as AsyncClient;
    use tracing::debug;

    #[derive(Resource, Debug)]
    pub struct NodalApi {
        async_client: AsyncClient,
        blocking_client: BlockingClient,
    }

    pub struct TestApiResponse {
        pub response: String,
    }

    impl NodalApi {
        pub fn new() -> Self {
            NodalApi {
                async_client: AsyncClient::new(),
                blocking_client: BlockingClient::new(),
            }
        }

        fn base_url() -> String {
            // dev stage url
            #[cfg(debug_assertions)]
            {
                "https://vqn63th004.execute-api.us-west-2.amazonaws.com/prod".to_string()
            }
            // prod stage url
            #[cfg(not(debug_assertions))]
            {
                PROD_URL
            }
            // beta stage url
            #[cfg(feature = "beta")]
            {
                BETA_URL
            }
        }

        pub fn redirect_uri(&self) -> String {
            Self::base_url() + "/auth/callback"
        }

        pub fn call_test_blocking(&self) -> Result<TestApiResponse, Box<dyn Error>> {
            let res = self
                .blocking_client
                .get(Self::base_url() + "/test")
                .send()?;
            let body = res.text()?;
            debug!("call_test_blocking returned {}", body);
            Ok(TestApiResponse { response: body })
        }

        pub async fn call_test_async(&self) -> Result<TestApiResponse, Box<dyn Error>> {
            let res = self
                .async_client
                .get(Self::base_url() + "/test")
                .send()
                .await?;
            let body = res.text().await?;
            debug!("call_test_async returned {}", body);
            Ok(TestApiResponse { response: body })
        }
    }
}
