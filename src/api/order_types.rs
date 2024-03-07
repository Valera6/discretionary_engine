use crate::api::Symbol;
use v_utils::trades::Side;

//TODO!: Move order_types to v_utils when stable

//TODO!!: automatically derive the Protocol Order types (by substituting `size` with `percent_size`, then auto-implementation of the conversion. Looks like I'm making a `discretionary_engine_macros` crate specifically to for this.

/// Generics for defining order types and their whereabouts. Specific `size` and `market` are to be added in the api-specific part of the implementation.
#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
	Market(Market),
	Limit(Limit),
	StopMarket(StopMarket),
}

//ref: https://binance-docs.github.io/apidocs/futures/en/#new-order-trade
#[derive(Debug, Clone, PartialEq)]
pub struct Market {
	pub symbol: Symbol,
	pub side: Side,
	pub size_usd: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StopMarket {
	pub symbol: Symbol,
	pub side: Side,
	pub price: f64,
	pub size_usd: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Limit {
	pub symbol: Symbol,
	pub side: Side,
	pub price: f64,
	pub size_usd: f64,
}

//=============================================================================
// Apparently, this is how we're pushing orders up to later be chosen and assigned sizes
//=============================================================================

pub enum OrderTypeP {
	Market(MarketP),
	Limit(LimitP),
	StopMarket(StopMarketP),
}

impl OrderTypeP {
	pub fn to_exact(self, total_controled_size: f64) -> OrderType {
		match self {
			OrderTypeP::Market(m) => OrderType::Market(m.to_exact(total_controled_size)),
			OrderTypeP::Limit(l) => OrderType::Limit(l.to_exact(total_controled_size)),
			OrderTypeP::StopMarket(s) => OrderType::StopMarket(s.to_exact(total_controled_size)),
		}
	}
}

pub struct MarketP {
	pub symbol: Symbol,
	pub side: Side,
	pub percent_size: f64,
}

impl MarketP {
	pub fn to_exact(self, total_controled_size: f64) -> Market {
		Market {
			symbol: self.symbol,
			side: self.side,
			size_usd: total_controled_size * self.percent_size,
		}
	}
}

pub struct StopMarketP {
	pub symbol: Symbol,
	pub side: Side,
	pub price: f64,
	pub percent_size: f64,
}

impl StopMarketP {
	pub fn to_exact(self, total_controled_size: f64) -> StopMarket {
		StopMarket {
			symbol: self.symbol,
			side: self.side,
			price: self.price,
			size_usd: total_controled_size * self.percent_size,
		}
	}
}

pub struct LimitP {
	pub symbol: Symbol,
	pub side: Side,
	pub price: f64,
	pub percent_size: f64,
}

impl LimitP {
	pub fn to_exact(self, total_controled_size: f64) -> Limit {
		Limit {
			symbol: self.symbol,
			side: self.side,
			price: self.price,
			size_usd: total_controled_size * self.percent_size,
		}
	}
}