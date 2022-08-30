// For LoRaWAN 1.0: SNwkSIntKey = NwkSEncKey = FNwkSIntKey = NwkSKey
pub fn pv_a(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x01, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}