use std::fmt::Display;
use alohomora::bbox::BBox;
use alohomora::fold::fold;
use alohomora::policy::{AnyPolicyDyn, NoPolicy, Specializable};
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use crate::{Money, MoneyOut};

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

fn sign_matches(units: i64, nanos: i32) -> bool {
    nanos == 0 || units == 0 || (nanos < 0) == (units < 0)
}

fn valid_nanos(nanos: i32) -> bool {
    MIN_NANO <= nanos && nanos <= MAX_NANO
}

fn is_valid(units: i64, nanos: i32) -> bool {
    valid_nanos(nanos) && sign_matches(units, nanos)
}

fn is_positive(units: i64, nanos: i32) -> bool {
    is_valid(units, nanos) && units > 0 || (units == 0 && nanos > 0)
}

fn is_negative(units: i64, nanos: i32) -> bool {
    is_valid(units, nanos) && units < 0 || (units == 0 && nanos < 0)
}

fn are_same_currency(l: &str, r: &str) -> bool {
    l == r && l != ""
}

fn negate(m: Money) -> Money {
    Money {
        currency_code: m.currency_code,
        units: m.units.into_ppr(PrivacyPureRegion::new(|units: i64| -units)),
        nanos: m.nanos.into_ppr(PrivacyPureRegion::new(|nanos: i32| -nanos)),
    }
}

pub fn sum(l: &Money, r: &Money) -> Result<Money, MoneyErrors> {
    let result = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
        (l.clone(), r.clone()),
        PrivacyPureRegion::new(|(l, r): (MoneyOut, MoneyOut)| {
            if let false = (is_valid(l.units, l.nanos) || is_valid(r.units, r.nanos)) {
                return Err(MoneyErrors::InvalidValue);
            }
            if let false = are_same_currency(&l.currency_code, &r.currency_code) {
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
            Ok(MoneyOut {
                units,
                nanos,
                currency_code: l.currency_code,
            })
        })
    );

    let result = result.unwrap();
    let result = result.specialize_policy::<NoPolicy>().unwrap();
    let result = result.fold_in()?;
    Ok(Money::from(result))
}

pub fn slow_multiply(m: &Money, n: i32) -> Money {
    let mut out = m.clone();
    for _ in 0..n {
        out = sum(&out, &m).expect("Couldn't sum the value to its aggregator");
    }
    out
}
