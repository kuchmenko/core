use typeshare::typeshare;
use serde::{Serialize, Deserialize};

use crate::chain::Chain;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[typeshare(swift = "Equatable, Codable, Hashable")]
pub struct Subscription {
    pub chain: Chain,
    pub address: String,
}