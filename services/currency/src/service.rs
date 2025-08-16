use crate::types::{CurrencyConversionRequest, GetSupportedCurrenciesResponse, Money};

#[tarpc::service]
pub trait CurrencyService {
    async fn get_supported_currencies() -> GetSupportedCurrenciesResponse;
    async fn convert(conversion_request: CurrencyConversionRequest) -> Money;
}
