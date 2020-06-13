use std::ops::Deref;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct AssetSymbol {
    symbol: String,
}

impl Deref for AssetSymbol {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

impl fmt::Display for AssetSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl<S> From<S> for AssetSymbol
    where S: AsRef<str>
{
    fn from(symbol: S) -> AssetSymbol {
        AssetSymbol {
            symbol: symbol.as_ref().into()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MarketSymbol {
    symbol: String,
    base: AssetSymbol,
    quote: AssetSymbol,
}

impl MarketSymbol {
    pub fn get_base(&self) -> &AssetSymbol {
        &self.base
    }

    pub fn get_quote(&self) -> &AssetSymbol {
        &self.quote
    }
}

impl Deref for MarketSymbol {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

impl fmt::Display for MarketSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl<B, Q> From<(B, Q)> for MarketSymbol
    where
        B: AsRef<str>,
        Q: AsRef<str>,
{
    fn from((base, quote): (B, Q)) -> MarketSymbol {
        let base = base.as_ref();
        let quote = quote.as_ref();
        MarketSymbol {
            symbol: format!("{}{}", base, quote),
            base: base.into(),
            quote: quote.into()
        }
    }
}