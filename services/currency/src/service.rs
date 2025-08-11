use crate::types::{CurrencyConversionRequest, GetSupperCurrenciesResponse, Money};

#[tarpc::service]
pub trait CurrencyService {
    async fn get_supported_currencies() -> GetSupperCurrenciesResponse;
    async fn convert(conversion_request: CurrencyConversionRequest) -> Money;
}
