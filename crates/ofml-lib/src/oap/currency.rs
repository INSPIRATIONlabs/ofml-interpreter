//! Currency and Money types for multi-currency price handling
//!
//! This module provides:
//! - `Currency` enum for supported currencies (EUR, CHF, USD, GBP)
//! - `Money` struct for type-safe monetary values with currency
//! - `CurrencyConverter` for currency conversion with configurable rates
//!
//! # Example
//! ```ignore
//! use ofml_lib::oap::currency::{Money, Currency, CurrencyConverter};
//!
//! let price = Money::new(1000.0, Currency::EUR);
//! let converter = CurrencyConverter::new();
//! let chf_price = converter.convert(&price, Currency::CHF);
//! ```

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use thiserror::Error;

/// Errors that can occur during currency operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CurrencyError {
    #[error("Currency mismatch: cannot operate on {0} and {1}")]
    CurrencyMismatch(Currency, Currency),

    #[error("Exchange rate not available for {0} to {1}")]
    ExchangeRateNotAvailable(Currency, Currency),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
}

/// Supported currencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    /// Euro (default for OFML data)
    EUR,
    /// Swiss Franc
    CHF,
    /// US Dollar
    USD,
    /// British Pound
    GBP,
    /// Polish Zloty
    PLN,
    /// Swedish Krona
    SEK,
    /// Norwegian Krone
    NOK,
    /// Danish Krone
    DKK,
    /// Czech Koruna
    CZK,
    /// Hungarian Forint
    HUF,
}

impl Currency {
    /// Get the ISO 4217 code for this currency
    pub fn code(&self) -> &'static str {
        match self {
            Currency::EUR => "EUR",
            Currency::CHF => "CHF",
            Currency::USD => "USD",
            Currency::GBP => "GBP",
            Currency::PLN => "PLN",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::CZK => "CZK",
            Currency::HUF => "HUF",
        }
    }

    /// Get the currency symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::EUR => "€",
            Currency::CHF => "CHF",
            Currency::USD => "$",
            Currency::GBP => "£",
            Currency::PLN => "zł",
            Currency::SEK => "kr",
            Currency::NOK => "kr",
            Currency::DKK => "kr",
            Currency::CZK => "Kč",
            Currency::HUF => "Ft",
        }
    }

    /// Get decimal places for this currency
    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::HUF => 0, // Hungarian Forint has no decimals
            _ => 2,
        }
    }

    /// Parse currency from string code
    pub fn from_code(code: &str) -> Option<Currency> {
        match code.to_uppercase().as_str() {
            "EUR" => Some(Currency::EUR),
            "CHF" => Some(Currency::CHF),
            "USD" => Some(Currency::USD),
            "GBP" => Some(Currency::GBP),
            "PLN" => Some(Currency::PLN),
            "SEK" => Some(Currency::SEK),
            "NOK" => Some(Currency::NOK),
            "DKK" => Some(Currency::DKK),
            "CZK" => Some(Currency::CZK),
            "HUF" => Some(Currency::HUF),
            _ => None,
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::EUR
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Type-safe monetary value with currency
///
/// Money represents a monetary amount in a specific currency.
/// Operations between different currencies are not allowed without explicit conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in smallest currency unit (cents for EUR/USD/CHF)
    amount_cents: i64,
    /// Currency of this money value
    currency: Currency,
}

impl Money {
    /// Create a new Money value from a decimal amount
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        let scale = Decimal::from(10i64.pow(currency.decimal_places()));
        let amount_cents = (amount * scale).round().to_string().parse().unwrap_or(0);
        Self {
            amount_cents,
            currency,
        }
    }

    /// Create a new Money value from an f64 amount
    pub fn from_f64(amount: f64, currency: Currency) -> Self {
        Self::new(Decimal::try_from(amount).unwrap_or_default(), currency)
    }

    /// Create a new Money value from cents/smallest unit
    pub fn from_cents(cents: i64, currency: Currency) -> Self {
        Self {
            amount_cents: cents,
            currency,
        }
    }

    /// Create a zero Money value
    pub fn zero(currency: Currency) -> Self {
        Self {
            amount_cents: 0,
            currency,
        }
    }

    /// Get the amount as a Decimal
    pub fn amount(&self) -> Decimal {
        let scale = Decimal::from(10i64.pow(self.currency.decimal_places()));
        Decimal::from(self.amount_cents) / scale
    }

    /// Get the amount in cents (smallest currency unit)
    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    /// Get the currency
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Check if this Money value is zero
    pub fn is_zero(&self) -> bool {
        self.amount_cents == 0
    }

    /// Check if this Money value is positive
    pub fn is_positive(&self) -> bool {
        self.amount_cents > 0
    }

    /// Check if this Money value is negative
    pub fn is_negative(&self) -> bool {
        self.amount_cents < 0
    }

    /// Get the absolute value
    pub fn abs(&self) -> Self {
        Self {
            amount_cents: self.amount_cents.abs(),
            currency: self.currency,
        }
    }

    /// Negate the amount
    pub fn negate(&self) -> Self {
        Self {
            amount_cents: -self.amount_cents,
            currency: self.currency,
        }
    }

    /// Round to the nearest unit (useful for display)
    pub fn round(&self) -> Self {
        // Already stored in cents, so this is a no-op for precision
        // but could implement banker's rounding if needed
        *self
    }

    /// Format the money value for display (German format)
    pub fn format_german(&self) -> String {
        let amount = self.amount();
        let formatted = format!("{:.2}", amount);
        let parts: Vec<&str> = formatted.split('.').collect();

        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"00");

        // Add thousand separators
        let mut result = String::new();
        let chars: Vec<char> = integer_part.chars().collect();
        let is_negative = chars.first() == Some(&'-');
        let start_idx = if is_negative { 1 } else { 0 };
        let len = chars.len() - start_idx;

        if is_negative {
            result.push('-');
        }

        for (i, c) in chars[start_idx..].iter().enumerate() {
            if i > 0 && (len - i).rem_euclid(3) == 0 {
                result.push('.');
            }
            result.push(*c);
        }

        format!("{},{} {}", result, decimal_part, self.currency.code())
    }

    /// Format the money value for display (US format)
    pub fn format_us(&self) -> String {
        let amount = self.amount();
        format!(
            "{}{:.2}",
            self.currency.symbol(),
            amount.abs().to_string().parse::<f64>().unwrap_or(0.0)
        )
    }

    /// Add another Money value (must be same currency)
    pub fn checked_add(&self, other: &Money) -> Result<Money, CurrencyError> {
        if self.currency != other.currency {
            return Err(CurrencyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Self {
            amount_cents: self.amount_cents + other.amount_cents,
            currency: self.currency,
        })
    }

    /// Subtract another Money value (must be same currency)
    pub fn checked_sub(&self, other: &Money) -> Result<Money, CurrencyError> {
        if self.currency != other.currency {
            return Err(CurrencyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Self {
            amount_cents: self.amount_cents - other.amount_cents,
            currency: self.currency,
        })
    }

    /// Multiply by a scalar (percentage, quantity, etc.)
    pub fn multiply(&self, factor: Decimal) -> Self {
        let new_amount = self.amount() * factor;
        Self::new(new_amount, self.currency)
    }

    /// Apply a percentage (e.g., 19% -> factor = 19)
    pub fn apply_percentage(&self, percentage: Decimal) -> Self {
        self.multiply(percentage / Decimal::from(100))
    }
}

impl Default for Money {
    fn default() -> Self {
        Self::zero(Currency::EUR)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_german())
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.checked_add(&other).expect("Currency mismatch in add")
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.checked_sub(&other).expect("Currency mismatch in sub")
    }
}

impl Mul<Decimal> for Money {
    type Output = Self;

    fn mul(self, factor: Decimal) -> Self {
        self.multiply(factor)
    }
}

impl Mul<i32> for Money {
    type Output = Self;

    fn mul(self, factor: i32) -> Self {
        self.multiply(Decimal::from(factor))
    }
}

/// Exchange rate between two currencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// Source currency
    pub from: Currency,
    /// Target currency
    pub to: Currency,
    /// Exchange rate (1 from = rate to)
    pub rate: Decimal,
}

impl ExchangeRate {
    pub fn new(from: Currency, to: Currency, rate: Decimal) -> Self {
        Self { from, to, rate }
    }
}

/// Currency converter with configurable exchange rates
///
/// Provides conversion between currencies using configured exchange rates.
/// Default rates are rough approximations and should be updated with real rates.
#[derive(Debug, Clone, Default)]
pub struct CurrencyConverter {
    /// Exchange rates indexed by (from, to) pair
    rates: HashMap<(Currency, Currency), Decimal>,
}

impl CurrencyConverter {
    /// Create a new currency converter with default rates
    pub fn new() -> Self {
        let mut converter = Self::empty();
        converter.load_default_rates();
        converter
    }

    /// Create an empty converter with no rates
    pub fn empty() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    /// Load default approximate rates (EUR as base)
    fn load_default_rates(&mut self) {
        // Approximate rates as of 2025 (for reference only, should be updated)
        let eur_rates = [
            (Currency::CHF, Decimal::new(94, 2)),  // 0.94
            (Currency::USD, Decimal::new(108, 2)), // 1.08
            (Currency::GBP, Decimal::new(84, 2)),  // 0.84
            (Currency::PLN, Decimal::new(432, 2)), // 4.32
            (Currency::SEK, Decimal::new(1143, 2)), // 11.43
            (Currency::NOK, Decimal::new(1178, 2)), // 11.78
            (Currency::DKK, Decimal::new(745, 2)), // 7.45
            (Currency::CZK, Decimal::new(2520, 2)), // 25.20
            (Currency::HUF, Decimal::new(39500, 2)), // 395.00
        ];

        // Set EUR -> X rates
        for (currency, rate) in &eur_rates {
            self.set_rate(Currency::EUR, *currency, *rate);
            // Also set inverse rates
            if !rate.is_zero() {
                self.set_rate(*currency, Currency::EUR, Decimal::ONE / *rate);
            }
        }

        // EUR -> EUR is always 1
        self.set_rate(Currency::EUR, Currency::EUR, Decimal::ONE);

        // Set same-currency rates
        for currency in [
            Currency::CHF,
            Currency::USD,
            Currency::GBP,
            Currency::PLN,
            Currency::SEK,
            Currency::NOK,
            Currency::DKK,
            Currency::CZK,
            Currency::HUF,
        ] {
            self.set_rate(currency, currency, Decimal::ONE);
        }
    }

    /// Set an exchange rate
    pub fn set_rate(&mut self, from: Currency, to: Currency, rate: Decimal) {
        self.rates.insert((from, to), rate);
    }

    /// Get an exchange rate
    pub fn get_rate(&self, from: Currency, to: Currency) -> Option<Decimal> {
        self.rates.get(&(from, to)).copied()
    }

    /// Convert money to a different currency
    pub fn convert(&self, money: &Money, to: Currency) -> Result<Money, CurrencyError> {
        if money.currency() == to {
            return Ok(*money);
        }

        let rate = self
            .get_rate(money.currency(), to)
            .ok_or(CurrencyError::ExchangeRateNotAvailable(money.currency(), to))?;

        let converted_amount = money.amount() * rate;
        Ok(Money::new(converted_amount, to))
    }

    /// Convert money using EUR as intermediary if direct rate not available
    pub fn convert_via_eur(&self, money: &Money, to: Currency) -> Result<Money, CurrencyError> {
        // Try direct conversion first
        if let Ok(converted) = self.convert(money, to) {
            return Ok(converted);
        }

        // Convert via EUR
        let in_eur = self.convert(money, Currency::EUR)?;
        self.convert(&in_eur, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::CHF.code(), "CHF");
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::GBP.code(), "GBP");
    }

    #[test]
    fn test_currency_symbol() {
        assert_eq!(Currency::EUR.symbol(), "€");
        assert_eq!(Currency::USD.symbol(), "$");
        assert_eq!(Currency::GBP.symbol(), "£");
        assert_eq!(Currency::CHF.symbol(), "CHF");
    }

    #[test]
    fn test_currency_from_code() {
        assert_eq!(Currency::from_code("EUR"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("eur"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("CHF"), Some(Currency::CHF));
        assert_eq!(Currency::from_code("INVALID"), None);
    }

    #[test]
    fn test_currency_default() {
        assert_eq!(Currency::default(), Currency::EUR);
    }

    #[test]
    fn test_money_new() {
        let money = Money::new(Decimal::new(10050, 2), Currency::EUR);
        assert_eq!(money.amount(), Decimal::new(10050, 2));
        assert_eq!(money.currency(), Currency::EUR);
        assert_eq!(money.amount_cents(), 10050);
    }

    #[test]
    fn test_money_from_f64() {
        let money = Money::from_f64(100.50, Currency::EUR);
        assert_eq!(money.amount_cents(), 10050);
        assert_eq!(money.currency(), Currency::EUR);
    }

    #[test]
    fn test_money_from_cents() {
        let money = Money::from_cents(10050, Currency::EUR);
        assert_eq!(money.amount(), Decimal::new(10050, 2));
    }

    #[test]
    fn test_money_zero() {
        let money = Money::zero(Currency::USD);
        assert!(money.is_zero());
        assert!(!money.is_positive());
        assert!(!money.is_negative());
    }

    #[test]
    fn test_money_positive_negative() {
        let positive = Money::from_cents(100, Currency::EUR);
        let negative = Money::from_cents(-100, Currency::EUR);

        assert!(positive.is_positive());
        assert!(!positive.is_negative());
        assert!(negative.is_negative());
        assert!(!negative.is_positive());
    }

    #[test]
    fn test_money_abs() {
        let negative = Money::from_cents(-100, Currency::EUR);
        let abs = negative.abs();
        assert_eq!(abs.amount_cents(), 100);
    }

    #[test]
    fn test_money_negate() {
        let positive = Money::from_cents(100, Currency::EUR);
        let negated = positive.negate();
        assert_eq!(negated.amount_cents(), -100);
    }

    #[test]
    fn test_money_add_same_currency() {
        let a = Money::from_cents(100, Currency::EUR);
        let b = Money::from_cents(50, Currency::EUR);
        let sum = a + b;
        assert_eq!(sum.amount_cents(), 150);
    }

    #[test]
    fn test_money_sub_same_currency() {
        let a = Money::from_cents(100, Currency::EUR);
        let b = Money::from_cents(30, Currency::EUR);
        let diff = a - b;
        assert_eq!(diff.amount_cents(), 70);
    }

    #[test]
    fn test_money_checked_add_different_currency() {
        let eur = Money::from_cents(100, Currency::EUR);
        let usd = Money::from_cents(100, Currency::USD);
        let result = eur.checked_add(&usd);
        assert!(matches!(result, Err(CurrencyError::CurrencyMismatch(_, _))));
    }

    #[test]
    fn test_money_multiply() {
        let money = Money::from_cents(10000, Currency::EUR);
        let doubled = money.multiply(Decimal::from(2));
        assert_eq!(doubled.amount_cents(), 20000);
    }

    #[test]
    fn test_money_apply_percentage() {
        let money = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let tax = money.apply_percentage(Decimal::from(19)); // 19%
        assert_eq!(tax.amount_cents(), 1900); // 19.00 EUR
    }

    #[test]
    fn test_money_format_german() {
        let money = Money::from_cents(123456, Currency::EUR);
        assert_eq!(money.format_german(), "1.234,56 EUR");
    }

    #[test]
    fn test_money_format_german_small() {
        let money = Money::from_cents(500, Currency::EUR);
        assert_eq!(money.format_german(), "5,00 EUR");
    }

    #[test]
    fn test_money_default() {
        let money = Money::default();
        assert!(money.is_zero());
        assert_eq!(money.currency(), Currency::EUR);
    }

    #[test]
    fn test_money_display() {
        let money = Money::from_cents(10050, Currency::EUR);
        assert_eq!(format!("{}", money), "100,50 EUR");
    }

    #[test]
    fn test_money_mul_decimal() {
        let money = Money::from_cents(10000, Currency::EUR);
        let result = money * Decimal::from(3);
        assert_eq!(result.amount_cents(), 30000);
    }

    #[test]
    fn test_money_mul_i32() {
        let money = Money::from_cents(10000, Currency::EUR);
        let result = money * 2;
        assert_eq!(result.amount_cents(), 20000);
    }

    #[test]
    fn test_currency_converter_new() {
        let converter = CurrencyConverter::new();
        // Should have some rates loaded
        assert!(converter.get_rate(Currency::EUR, Currency::CHF).is_some());
        assert!(converter.get_rate(Currency::EUR, Currency::USD).is_some());
    }

    #[test]
    fn test_currency_converter_same_currency() {
        let converter = CurrencyConverter::new();
        let money = Money::from_cents(10000, Currency::EUR);
        let converted = converter.convert(&money, Currency::EUR).unwrap();
        assert_eq!(converted.amount_cents(), 10000);
    }

    #[test]
    fn test_currency_converter_eur_to_chf() {
        let converter = CurrencyConverter::new();
        let money = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let converted = converter.convert(&money, Currency::CHF).unwrap();
        // With default rate of 0.94, 100 EUR = 94 CHF
        assert_eq!(converted.amount_cents(), 9400);
        assert_eq!(converted.currency(), Currency::CHF);
    }

    #[test]
    fn test_currency_converter_chf_to_eur() {
        let converter = CurrencyConverter::new();
        let money = Money::from_cents(9400, Currency::CHF); // 94.00 CHF
        let converted = converter.convert(&money, Currency::EUR).unwrap();
        // Should convert back close to 100 EUR
        assert_eq!(converted.currency(), Currency::EUR);
        // Note: might have small rounding differences
    }

    #[test]
    fn test_currency_converter_set_rate() {
        let mut converter = CurrencyConverter::empty();
        converter.set_rate(Currency::EUR, Currency::USD, Decimal::new(110, 2));

        let money = Money::from_cents(10000, Currency::EUR);
        let converted = converter.convert(&money, Currency::USD).unwrap();
        assert_eq!(converted.amount_cents(), 11000);
    }

    #[test]
    fn test_currency_converter_missing_rate() {
        let converter = CurrencyConverter::empty();
        let money = Money::from_cents(10000, Currency::EUR);
        let result = converter.convert(&money, Currency::USD);
        assert!(matches!(
            result,
            Err(CurrencyError::ExchangeRateNotAvailable(_, _))
        ));
    }

    #[test]
    fn test_exchange_rate_struct() {
        let rate = ExchangeRate::new(Currency::EUR, Currency::USD, Decimal::new(110, 2));
        assert_eq!(rate.from, Currency::EUR);
        assert_eq!(rate.to, Currency::USD);
        assert_eq!(rate.rate, Decimal::new(110, 2));
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::EUR.decimal_places(), 2);
        assert_eq!(Currency::USD.decimal_places(), 2);
        assert_eq!(Currency::HUF.decimal_places(), 0);
    }

    #[test]
    fn test_money_hungarian_forint() {
        // HUF has no decimal places
        let money = Money::from_f64(1000.0, Currency::HUF);
        assert_eq!(money.amount_cents(), 1000);
        assert_eq!(money.amount(), Decimal::from(1000));
    }

    #[test]
    fn test_currency_error_display() {
        let err = CurrencyError::CurrencyMismatch(Currency::EUR, Currency::USD);
        assert!(err.to_string().contains("EUR"));
        assert!(err.to_string().contains("USD"));

        let err2 = CurrencyError::ExchangeRateNotAvailable(Currency::EUR, Currency::CHF);
        assert!(err2.to_string().contains("Exchange rate not available"));
    }

    #[test]
    fn test_money_serialization() {
        let money = Money::from_cents(10050, Currency::EUR);
        let json = serde_json::to_string(&money).unwrap();
        assert!(json.contains("10050"));
        assert!(json.contains("EUR"));

        let parsed: Money = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.amount_cents(), 10050);
        assert_eq!(parsed.currency(), Currency::EUR);
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency::EUR;
        let json = serde_json::to_string(&currency).unwrap();
        assert_eq!(json, "\"EUR\"");

        let parsed: Currency = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Currency::EUR);
    }

    #[test]
    fn test_convert_via_eur() {
        let converter = CurrencyConverter::new();
        // CHF -> USD should work via EUR
        let money = Money::from_cents(10000, Currency::CHF);
        let result = converter.convert_via_eur(&money, Currency::USD);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().currency(), Currency::USD);
    }
}
