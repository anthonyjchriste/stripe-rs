use crate::ids::TransferId;
use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{Account, BalanceTransaction, Charge, Currency, TransferReversal};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Transfer".
///
/// For more details see [https://stripe.com/docs/api/transfers/object](https://stripe.com/docs/api/transfers/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transfer {
    /// Unique identifier for the object.
    pub id: TransferId,

    /// Amount in %s to be transferred.
    pub amount: i64,

    /// Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,

    /// Balance transaction that describes the impact of this transfer on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time that this record of the transfer was first created.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the Stripe account the transfer was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Expandable<Account>>,

    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<Expandable<Charge>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A set of key-value pairs that you can attach to a transfer object.
    ///
    /// It can be useful for storing additional information about the transfer in a structured format.
    pub metadata: Metadata,

    /// A list of reversals that have been applied to the transfer.
    pub reversals: List<TransferReversal>,

    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,

    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<Expandable<Charge>>,

    /// The source balance this transfer came from.
    ///
    /// One of `card` or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#grouping-transactions) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Transfer {
    /// Returns a list of existing transfers sent to connected accounts.
    ///
    /// The transfers are returned in sorted order, with the most recently created transfers appearing first.
    pub fn list(client: &Client, params: TransferListParams<'_>) -> Response<List<Transfer>> {
        client.get_query("/transfers", &params)
    }
}

impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}

/// The parameters for `Transfer::list`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TransferListParams<'a> {
    #[serde(skip_deserializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_deserializing_if = "Option::is_none")]
    ending_before: Option<&'a TransferId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_deserializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_deserializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_deserializing_if = "Option::is_none")]
    starting_after: Option<&'a TransferId>,
}
