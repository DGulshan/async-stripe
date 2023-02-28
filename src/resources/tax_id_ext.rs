use serde::Serialize;

use crate::client::{Client, Response};
use crate::generated::billing::tax_id::{TaxId, TaxIdType};
use crate::ids::{CustomerId, TaxIdId};

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateTaxId {
    #[serde(rename = "type")]
    pub type_: TaxIdType,
    pub value: String,
}

impl TaxId {
    pub fn create(
        client: &Client,
        customer_id: &CustomerId,
        params: CreateTaxId,
    ) -> Response<TaxId> {
        client.post_form(&format!("/customers/{}/tax_ids", customer_id), params)
    }

    pub fn delete(
        client: &Client,
        customer_id: &CustomerId,
        tax_id_id: &TaxIdId,
    ) -> Response<TaxId> {
        client.delete(&format!("/customers/{}/tax_ids/{}", customer_id, tax_id_id))
    }
}
