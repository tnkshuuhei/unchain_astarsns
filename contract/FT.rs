use crate::astar_sns_contract::AstarSnsContract;
use ink_env::AccountId;

impl AstarSnsContract {
    // トークン残高を確認する関数
    pub fn balance_of_fn(&self, account_id: AccountId) -> u128 {
        let asset = self.asset_mapping.get(&account_id).unwrap_or(0 as u128);
        asset
    }

    // 送信者と受信者を指定してトークンを送信
    pub fn transfer_fn(&mut self, from_id: AccountId, to_id: AccountId, amount: u128) {
        let mut to_asset = self.asset_mapping.get(&to_id).unwrap_or(0 as u128);
        let mut from_asset = self.asset_mapping.get(&from_id).unwrap_or(0 as u128);
        if from_asset < amount {
            return;
        }
        to_asset = to_asset + amount;
        from_asset = from_asset - amount;
        self.asset_mapping.insert(&to_id, &to_asset);
        self.asset_mapping.insert(&from_id, &from_asset);
    }

    // コントラクトから指定されたアカウントへトークンを送信
    pub fn distribute_fn(&mut self, to_id: AccountId, amount: u128) {
        let mut to_asset = self.asset_mapping.get(&to_id).unwrap_or(0 as u128);
        to_asset = to_asset + amount;
        self.asset_mapping.insert(&to_id, &to_asset);
    }
}