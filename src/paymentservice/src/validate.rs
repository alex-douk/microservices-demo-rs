use card_validate;
use chrono::Datelike;
use payment_service::types::PaymentCreditCardInfoOut;

pub(super) struct CreditCardDetails {
    pub end_numbers: String,
    pub card_type: String,
}
pub(super) fn validate_card(
    credit_card: PaymentCreditCardInfoOut,
) -> Result<CreditCardDetails, String> {
    let validate = card_validate::Validate::from(&credit_card.credit_card_number.as_str())
        .map_err(|_| String::from("Invalid credit card"))?;
    match validate.card_type.name().as_str() {
        card_type @ ("visa" | "mastercard") => Ok(CreditCardDetails {
            end_numbers: verify_expiration(credit_card)?,
            card_type: card_type.to_string(),
        }),
        card_type @ _ => {
            let err = format!("card_type {card_type} is not accepted");
            Err(err)
        }
    }
}

fn verify_expiration(credit_card: PaymentCreditCardInfoOut) -> Result<String, String> {
    let PaymentCreditCardInfoOut {
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
        false => Err(String::from("expired credit card"))//::ExpiredCreditCard(end_number, month, year)),
    }
}
