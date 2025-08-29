use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

const FRACTION_SIZE: f64 = (1 * 10i32.pow(9)) as f64;
const CONVERSION_PATH: &str = "data/currency_conversion.json";

use currency_service::types::MoneyOut;
use serde_json;

pub struct ConversionTable {
    pub(crate) table: HashMap<String, f64>,
}

impl ConversionTable {
    pub fn new() -> Self {
        let table = load_conversion_file().expect("Couldn't load the conversation table");
        Self { table }
    }

    pub fn convert(&self, money: MoneyOut, to_code: String) -> MoneyOut {
        let exchange_rate = self
            .table
            .get(&money.currency_code)
            .expect("Currency not supported");
        let units = money.units as f64 / exchange_rate;
        let nanos = money.nanos as f64 / exchange_rate;
        let (euro_units, euro_nanos) = carry_nanos(units, nanos);
        let euro_nanos = euro_nanos.round();


        let target_exchange = self.table.get(&to_code).expect("Currency not expected");
        let (dst_units, dst_nanos) =
            carry_nanos(euro_units * target_exchange, euro_nanos * target_exchange);

        MoneyOut {
            currency_code: to_code,
            units: dst_units.floor() as i64,
            nanos: dst_nanos.floor() as i32,
        }
    }

    // fn convert_to_eurs(&self, money: Money) ->
}

fn load_conversion_file() -> anyhow::Result<HashMap<String, f64>> {
    let file = File::open(CONVERSION_PATH)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

fn carry_nanos(units: f64, nanos: f64) -> (f64, f64) {
    let total_nanos = nanos + units.fract() * (FRACTION_SIZE);
    let units = units.floor() + total_nanos.div_euclid(FRACTION_SIZE);
    let nanos = total_nanos.rem_euclid(FRACTION_SIZE);
    (units, nanos)
}
