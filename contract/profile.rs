use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // 新しいユーザーのウォレットに接続した際に自動的に実行されるプロフィール作成関数
    // フロントでは最初はプロフィールの名前はunknown, imgUrlも指定されることになる。
    pub fn create_profile_fn(&mut self, account_id: AccountId) {
        // 既にアカウントが作成されていないか確認
        let is_already_connected = self.profile_map.contains(&account_id);
        if !is_already_connected {
            self.profile_map.insert(
                &(account_id),
                &Profile {
                    following_list: Vec::new(),
                    follower_list: Vec::new(),
                    friend_list: Vec::new(),
                    user_id: account_id,
                    name: None,
                    img_url: None,
                    message_list_id_list: Vec::new(),
                    post_id_list: Vec::new(),
                },
            );
        }
    }

    // プロフィールの名前と画像のURLを設定
    pub fn set_profile_info_fn(&mut self, account_id: AccountId, name: String, img_url: String) {
        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();
        profile.name = Some(name);
        profile.img_url = Some(img_url);
        self.profile_map.insert(&account_id, &profile);
    }

    // get profile info
    pub fn get_profile_info_fn(&self, account_id: AccountId) -> Profile {
        let profile: Profile = self.profile_map.get(&account_id).unwrap();
        profile
    }

    pub fn check_created_profile_fn(&self, account_id: AccountId) -> bool {
        let is_already_connected = self.profile_map.contains(&account_id);
        is_already_connected
    }
}