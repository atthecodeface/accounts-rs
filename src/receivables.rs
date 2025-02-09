use serde::{Deserialize, Serialize};

use crate::{Amount, Date, Entity};

//tp Receivable
/// A value that is due to be received from somebody or organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receivable {
    entity: Entity,
    amount: Amount,
    data: Date,
}
