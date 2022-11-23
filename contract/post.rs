use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::string::ToString;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // 投稿するための関数
    pub fn release_post_fn(
        &mut self,
        account_id: AccountId,
        description: String,
        created_time: String,
        post_img_url: String,
    ) {
        // 指定されたウォレットアドレスに紐づいたプロフィール
        let profile_info: Profile = self.profile_map.get(&account_id).unwrap();

        // 投稿IDに紐づいた投稿情報を追加
        self.post_map.insert(
            &(self.post_map_counter),
            &Post {
                name: profile_info.name.unwrap_or("unknown".to_string()),
                user_id: profile_info.user_id,
                created_time,
                img_url: post_img_url,
                user_img_url: profile_info.img_url.unwrap(),
                description: description,
                num_of_likes: 0,
                post_id: self.post_map_counter,
            },
        );

        // 指定されたウォレットアドレスに紐づいたプロフィール
        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();

        // 上で追加した時に使用した投稿IDをプロフィールに追加
        profile.post_id_list.push(self.post_map_counter);

        // プロフィールのマッピングを上書き
        self.profile_map.insert(&account_id, &profile);

        // 投稿IDを１大きくする
        self.post_map_counter = &self.post_map_counter + 1;
    }

    // 全ての投稿から指定の最新度の投稿を取得
    pub fn get_general_post_fn(&self, num: u128) -> Vec<Post> {
        // 返す投稿のリスト用のリストを生成
        let mut post_list: Vec<Post> = Vec::new();

        // 投稿マッピングの大きさを取得
        let length: u128 = self.post_map_counter;

        // どれくらいの量の投稿を取得するかを指定
        let amount_index: u128 = 5;

        // コントラクトに格納された投稿の量が指定された量の投稿より多いか判定。
        // それによって取得方法が異なるため
        if length < amount_index + 1 {
            for m in 0..(length + 1) {
                // get specified post and add to returning list
                let post: Option<Post> = self.post_map.get(&m);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // get specified post and add to returning list
                // this is done from latest posts
                let post: Option<Post> = self.post_map.get(&(length - n - 1));
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        // 返り値用のリストを返す。returnは省略可能
        post_list
    }

    // 個人の投稿を取得
    pub fn get_individual_post_fn(&self, num: u128, account_id: AccountId) -> Vec<Post> {
        // 返す投稿のリスト用のリストを生成
        let mut post_list: Vec<Post> = Vec::new();

        // 指定したユーザーのウォレットアドレスに紐づいた投稿IDを取得する
        let post_id_list: Vec<u128> = self.profile_map.get(&account_id).unwrap().post_id_list;

        // どれくらいの量の投稿を取得するかを指定
        let amount_index: u128 = 5;

        // 投稿IDのリストの長さを取得
        let length: u128 = post_id_list.len() as u128;

        // コントラクトに格納された投稿の量が指定された量の投稿より多いか判定。
        // それによって取得方法が異なるため
        if length < amount_index + 1 {
            for m in 0..(length) {
                // 指定された投稿を取得して、最初に生成されたリストに格納
                let post: Option<Post> = self.post_map.get(&post_id_list[m as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // 指定された投稿を取得して、最初に生成されたリストに格納
                let post: Option<Post> =
                    self.post_map.get(&post_id_list[(length - n - 1) as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        // 返り値用のリストを返す。returnは省略可能
        post_list
    }

    // 指定された投稿にいいねを追加
    pub fn add_likes_fn(&mut self, post_id: u128) {
        // 指定された投稿の情を取得
        let mut post: Post = self.post_map.get(&post_id).unwrap();

        // 投稿のいいね数を１大きくする
        post.num_of_likes = &post.num_of_likes + 1;

        // 指定した投稿の情報を上書き
        self.post_map.insert(&post_id, &post);
    }

    // 指定されたアカウントの投稿に含まれているいいねの総数を取得する関数
    pub fn get_total_likes_fn(&self, account_id: AccountId) -> u128 {
        // 指定したアカウントに紐づく投稿idのリストを取得
        let post_id_list = self.profile_map.get(&account_id).unwrap().post_id_list;

        // 返り値となるいいね数の総数を格納する変数
        let mut num_of_likes = 0;

        // 取得した投稿idリストの各要素となるidに紐づく投稿が獲得したいいね数を合計する
        for id in post_id_list {
            let likes_of_post = self.post_map.get(&id).unwrap().num_of_likes;
            num_of_likes = num_of_likes + likes_of_post;
        }

        // いいねの総数を返す
        num_of_likes
    }

}