use crate::client::*;
use crate::errors::*;
use crate::delivery::rest_model::*;
use crate::rest_model::ServerTime;

#[derive(Clone)]
pub struct DeliveryGeneral {
    pub client: Client,
}

impl DeliveryGeneral {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get("/dapi/v1/ping", None).await?;
        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get_p("/dapi/v1/time", None).await
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("/dapi/v1/exchangeInfo", None).await
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
mod tests {
    use std::env::var;

    use crate::{api::Binance, config};

    use super::DeliveryGeneral;

    async fn test_delivery_general() -> DeliveryGeneral {
        let api_key = Some(var("BINANCE_KEY").unwrap().to_string());
        let secret_key = Some(var("BINANCE_SECRET").unwrap().to_string());
        let config = config::Config::testnet();
        Binance::new_with_config(api_key, secret_key, &config)
    }

    #[tokio::test]
    async fn test_delivery_get_server_time() {
        let general = test_delivery_general().await;
        let result = general.get_server_time().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delivery_exchange_info() {
        let general = test_delivery_general().await;
        let result = general.exchange_info().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delivery_get_symbol_info() {
        let general = test_delivery_general().await;
        let result = general.get_symbol_info("BTCUSD_PERP").await;
        assert!(result.is_ok());
    }
}