use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // メッセージ送信関数
    pub fn send_message_fn(
        &mut self,
        message: String,
        message_list_id: u128,
        sender_id: AccountId,
        created_time: String,
    ) {
        // 指定したメッセージリストのidに紐づいたメッセージリストを取得
        let mut message_list: Vec<Message> = self
            .message_list_map
            .get(&message_list_id)
            .unwrap_or(Vec::default());

        // メッセージの内容を↑に追加
        message_list.push(Message {
            message,
            sender_id,
            created_time,
        });

        // ↑で追加した新しいメッセージリストでコントラクトの状態変数を上書き
        self.message_list_map
            .insert(&message_list_id, &message_list);
    }

    // メッセージリストを取得`
    pub fn get_message_list_fn(&self, message_list_id: u128, num: usize) -> Vec<Message> {
        // 指定したメッセージリストのidに紐づいたメッセージリストを取得
        let self_message_list: Vec<Message> = self.message_list_map.get(&message_list_id).unwrap();

        // 返り値用のメッセージリストを生成
        let mut message_list: Vec<Message> = Vec::new();

        // どれくらいの量の投稿を取得するかを指定
        let amount_index: usize = 5;

        // メッセージリストの長さを取得
        let list_length: usize = self_message_list.len();

        // コントラクトに格納された投稿の量が指定された量の投稿より多いか判定。
        // それによって取得方法が異なるため
        if list_length < amount_index + 1 {
            for m in 0..(list_length) {
                // 取得した内容を返り値用のメッセージリストに追加
                message_list.push(self_message_list[m].clone());
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // 取得した内容を返り値用のメッセージリストに追加
                message_list.push(self_message_list[n].clone());
            }
        }
        message_list
    }

    // 指定したメッセージリストのIDに紐づいたメッセージリストの最後のメッセージを取得
    pub fn get_last_message_fn(&self, message_list_id: u128) -> Message {
        let last_message: Message = self
            .message_list_map
            .get(&message_list_id)
            .unwrap()
            .last()
            .unwrap()
            .clone();
        last_message
    }
}