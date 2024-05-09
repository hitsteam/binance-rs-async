use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::*;
use crate::rest_model::ServerTime;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get("/fapi/v1/ping", None).await?;
        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get_p("/fapi/v1/time", None).await
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
        /// Obtain exchange information (rate limits, symbol metadata etc)
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let conf = Config::default().set_rest_api_endpoint(DATA_REST_ENDPOINT);
    /// let general: General = Binance::new_with_env(&conf);
    /// let exchange_info = tokio_test::block_on(general.exchange_info());
    /// assert!(exchange_info.is_ok(), "{:?}", exchange_info);
    /// ```  
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("/fapi/v1/exchangeInfo", None).await
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let symbol_string = symbol.into();
        let upper_symbol = symbol_string.to_uppercase();
        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                Err(Error::UnknownSymbol(symbol_string.clone()))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use std::env::var;

    use crate::{api::Binance, config::*};

    use super::FuturesGeneral;

    async fn test_futures_general() -> FuturesGeneral {
        let api_key = Some(var("BINANCE_KEY").unwrap().to_string());
        let secret_key = Some(var("BINANCE_SECRET").unwrap().to_string());
        let conf = Config::default().set_rest_api_endpoint(DATA_REST_ENDPOINT);
        let general: FuturesGeneral = Binance::new_with_config(api_key, secret_key, &conf);
        general
    }

    #[tokio::test]
    async fn test_ping() {
        let general = test_futures_general().await;
        let ping = general.ping().await;
        assert!(ping.is_ok(), "{:?}", ping);
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let general = test_futures_general().await;
        let server_time = general.get_server_time().await;
        assert!(server_time.is_ok(), "{:?}", server_time);
    }

    #[tokio::test]
    async fn test_exchange_info() {
        let general = test_futures_general().await;
        let exchange_info = general.exchange_info().await;
        assert!(exchange_info.is_ok(), "{:?}", exchange_info);
    }

    #[tokio::test]
    async fn test_get_symbol_info() {
        let general = test_futures_general().await;
        let symbol_info = general.get_symbol_info("BTCUSDT").await;
        assert!(symbol_info.is_ok(), "{:?}", symbol_info);
    }
}