use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // フォロー関数
    pub fn follow_fn(&mut self, following_account_id: AccountId, followed_account_id: AccountId) {
        // 関数を読んだユーザーがフォローしているユーザーのプロフィール情報を取得
        let mut following_user_profile: Profile =
            self.profile_map.get(&following_account_id).unwrap();

        // 関数を読んだユーザーがフォローされているユーザーのプロフィール情報を取得
        let mut followed_user_profile: Profile =
            self.profile_map.get(&followed_account_id).unwrap();

        // 自分のフォローするユーザーのウォレットアドレスを自分のフォローリストに追加
        if !following_user_profile
            .following_list
            .contains(&followed_account_id)
        {
            following_user_profile
                .following_list
                .push(followed_account_id);
        }

        // 自分のウォレットアドレスを、自分のフォローするユーザーのフォロワーリストに追加
        if !followed_user_profile
            .follower_list
            .contains(&followed_account_id)
        {
            followed_user_profile
                .follower_list
                .push(following_account_id);
        }

        // 自分のメッセージリストのIDのリストの長さを取得
        let length: usize = following_user_profile.message_list_id_list.len();

        // 自分かフォローするアカウントのどちらかがメッセージリストを持っていないか確認
        if followed_user_profile.message_list_id_list.len() == 0
            && following_user_profile.message_list_id_list.len() == 0
        {
            // 二人のメッセージ用のメッセージリストのIDをそれぞれに追加
            followed_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            following_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            following_user_profile.friend_list.push(followed_account_id);
            followed_user_profile.friend_list.push(following_account_id);

            // メッセージリストのIDカウンターを１大きくする
            self.message_list_map_counter = &self.message_list_map_counter + 1;
        }

        for n in 0..length {
            // 自分の持っているメッセージリストのIDを、フォローする相手が持っていない
            // 既に二人用のメッセージリストが作成されていないかを確認
            let is_contained = followed_user_profile
                .message_list_id_list
                .contains(&following_user_profile.message_list_id_list[n]);

            // もし含まれていなければ(メッセージリストが作成されていなければ)メッセージリストのIDを追加
            if !is_contained {
                followed_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                following_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                following_user_profile.friend_list.push(followed_account_id);
                followed_user_profile.friend_list.push(following_account_id);
                self.message_list_map_counter = &self.message_list_map_counter + 1;
            }
        }

        // それぞれのプロフィール情報を上書き
        self.profile_map
            .insert(&following_account_id, &following_user_profile);
        self.profile_map
            .insert(&followed_account_id, &followed_user_profile);
    }

    // get following list of specified account
    pub fn get_following_list_fn(&self, account_id: AccountId) -> Vec<AccountId> {
        let following_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().following_list;
        following_list
    }

    // 指定したユーザーがフォローしているアカウントのフォローワーリストを取得
    pub fn get_follower_list_fn(&self, account_id: AccountId) -> Vec<AccountId> {
        let follower_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().follower_list;
        follower_list
    }
}