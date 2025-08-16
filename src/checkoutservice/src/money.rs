use std::fmt::Display;

use payment_service::types::Money;

const MIN_NANO: i32 = -999_999_999;
const MAX_NANO: i32 = 999_999_999;
const NANOS_MOD: i32 = 1_000_000_000;

#[derive(Debug, Clone)]
pub enum MoneyErrors {
    InvalidValue,
    MismatchingCurrency,
}

impl Display for MoneyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyErrors::InvalidValue => {
                write!(f, "{}", "One of the specified money values is invalid")
            }
            MoneyErrors::MismatchingCurrency => write!(f, "{}", "Mismatching currency code"),
        }
    }
}

impl std::error::Error for MoneyErrors {}

pub fn sign_matches(m: &Money) -> bool {
    m.nanos == 0 || m.units == 0 || (m.nanos < 0) == (m.units < 0)
}

pub fn valid_nanos(nanos: i32) -> bool {
    MIN_NANO <= nanos && nanos <= MAX_NANO
}

pub fn is_valid(m: &Money) -> bool {
    valid_nanos(m.nanos) && sign_matches(m)
}

pub fn is_positive(m: Money) -> bool {
    is_valid(&m) && m.units > 0 || (m.units == 0 && m.nanos > 0)
}

pub fn is_negative(m: Money) -> bool {
    is_valid(&m) && m.units < 0 || (m.units == 0 && m.nanos < 0)
}

pub fn are_same_currency(l: &Money, r: &Money) -> bool {
    l.currency_code == r.currency_code && l.currency_code != ""
}

pub fn are_equal(l: &Money, r: &Money) -> bool {
    l.currency_code == r.currency_code && l.units == r.units && l.nanos == r.nanos
}

pub fn negate(m: Money) -> Money {
    Money {
        currency_code: m.currency_code,
        units: -m.units,
        nanos: -m.nanos,
    }
}

pub fn sum(l: &Money, r: &Money) -> Result<Money, MoneyErrors> {
    if let false = (is_valid(&l) || is_valid(&r)) {
        return Err(MoneyErrors::InvalidValue);
    }
    if let false = are_same_currency(&l, &r) {
        return Err(MoneyErrors::MismatchingCurrency);
    }

    let mut units = l.units + r.units;
    let mut nanos = l.nanos + r.nanos;

    if (units == 0 && nanos == 0) || (units > 0 && nanos >= 0) || (units < 0 && nanos <= 0) {
        //same sign <units, nanos>
        units += (nanos / NANOS_MOD) as i64;
        nanos = nanos % NANOS_MOD;
    } else {
        // different sign. nanos guaranteed to not to go over the limit
        if units > 0 {
            units -= 1;
            nanos += NANOS_MOD;
        } else {
            units += 1;
            nanos -= NANOS_MOD;
        }
    }
    Ok(Money {
        units,
        nanos,
        currency_code: l.currency_code.clone(),
    })
}

pub fn slow_multiply(m: &Money, n: i32) -> Money {
    let mut out = m.clone();
    for _ in 0..n {
        out = sum(&out, &m).expect("Couldn't sum the value to its aggregator");
    }
    out
}
