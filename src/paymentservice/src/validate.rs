use card_validate;
use chrono::Datelike;
use payment_service::types::{CreditCardError, CreditCardInfoOut};

pub(super) struct CreditCardDetails {
    pub end_numbers: String,
    pub card_type: String,
}
pub(super) fn validate_card(
    credit_card: CreditCardInfoOut,
) -> Result<CreditCardDetails, CreditCardError> {
    let validate = card_validate::Validate::from(&credit_card.credit_card_number.as_str())
        .map_err(|_| CreditCardError::InvalidCreditCard)?;
    match validate.card_type.name().as_str() {
        card_type @ ("visa" | "mastercard") => Ok(CreditCardDetails {
            end_numbers: verify_expiration(credit_card)?,
            card_type: card_type.to_string(),
        }),
        card_type @ _ => {
            let err = CreditCardError::UnnaceptedCreditCard(card_type.to_string());
            Err(err)
        }
    }
}

fn verify_expiration(credit_card: CreditCardInfoOut) -> Result<String, CreditCardError> {
    let CreditCardInfoOut {
        credit_card_number,
        credit_card_expiration_month: month,
        credit_card_expiration_year: year,
        ..
    } = credit_card;
    let current_time = chrono::Utc::now();

    let end_number = credit_card_number
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // Also validate expiration is > today.
    let (current_month, current_year) = (current_time.month() + 1, current_time.year());
    match year * 12 + month > current_year * 12 + current_month as i32 {
        true => Ok(end_number),
        false => Err(CreditCardError::ExpiredCreditCard(end_number, month, year)),
    }
}
