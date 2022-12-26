import { ApiPromise } from "@polkadot/api";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import React, { useEffect, useState } from "react";

import BottomNavigation from "../components/bottomNavigation";
import MessageMember from "../components/message_member";
import MessageRoom from "../components/messageRoom";
import TopBar from "../components/topBar";
import { connectToContract } from "../hooks/connect";
import { balenceOf } from "../hooks/FT";
import { getLastMessage, getMessageList } from "../hooks/messageFunction";
import {
  checkCreatedInfo,
  createProfile,
  getProfileForMessage,
  getSimpleProfileForMessage,
} from "../hooks/profileFunction";
import type { ProfileType } from "../hooks/profileFunction";

const setMsec = 500;
const sleep = (waitMsec) => {
  var startMsec = new Date();
  while (new Date() - startMsec < waitMsec);
};

export default function message() {
  // variable related to contract
  const [isTrigger, setTrigger] = useState<Number>(0);
  const [api, setApi] = useState<ApiPromise>();
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAccount, setActingAccount] = useState<InjectedAccountWithMeta>();

  const [imgUrl, setImgUrl] = useState("");
  const [isCreatedProfile, setIsCreatedProfile] = useState(true);
  const [isCreatedFnRun, setIsCreatedFnRun] = useState(false);
  const [friendList, setFriendList] = useState([]);
  const [messageList, setMessageList] = useState([]);
  const [showMessageModal, setShowMessageModal] = useState(false);
  const [userName, setUserName] = useState("");
  const [userImgUrl, setUserImgUrl] = useState("");
  const [myImgUrl, setMyImgUrl] = useState("");
  const [messageListId, setMessageListId] = useState<string>("");
  const [messageMemberList, setMessageMemberList] = useState([]);
  const [myUserId, setMyUserId] = useState("");
  const [isSetup, setIsSetup] = useState(false);
  const [profile, setProfile] = useState<ProfileType>();
  const [balance, setBalance] = useState<string>("0");

  useEffect(() => {
    connectToContract({
      api: api,
      accountList: accountList,
      actingAccount: actingAccount!,
      isSetup: isSetup,
      setApi: setApi,
      setAccountList: setAccountList,
      setActingAccount: setActingAccount!,
      setIsSetup: setIsSetup,
    });
    sleep(500);
  }, [])

  useEffect(() => {
    if (!isSetup) return;
    console.log("isSetup");

    // get profile
    getProfileForMessage({
      api: api,
      userId: actingAccount?.address,
      setImgUrl: setImgUrl,
      setMyImgUrl: setMyImgUrl,
      setFriendList: setFriendList,
      setProfile: setProfile,
    });
    sleep(setMsec);

    // create message member list UI
    createMessageMemberList();
    sleep(setMsec);

    console.log("createMessageMemberList");

    balenceOf({
      api: api,
      actingAccount: actingAccount!,
      setBalance: setBalance,
    });
    sleep(setMsec);
    console.log("balenceOf");

    // check if already created profile in frontend
    if (isCreatedFnRun) return;
    console.log("isCreatedFnRun");

    // check if already created profile in contract
    checkCreatedInfo({
      api: api,
      userId: actingAccount?.address,
      setIsCreatedProfile: setIsCreatedProfile,
    });
    console.log("checkCreatedInfo");

    if (isCreatedProfile) return;
    console.log("isCreatedProfile");

    // create profile
    createProfile({ api: api, actingAccount: actingAccount! });
    console.log("createProfile");

    setIsCreatedFnRun(true);
    console.log("setIsCreatedFnRun");

  }, [actingAccount]);

  // create message member list UI
  const createMessageMemberList = async () => {
    let memberList: Array<any> = new Array();
    console.log("friendList: " + friendList);
    for (var i = 0; i < friendList.length; i++) {
      let friendProfile = await getSimpleProfileForMessage({
        api: api,
        userId: friendList[i],
      });
      console.log("friendProfile: " + JSON.stringify(friendProfile));
      let idList = profile?.messageListIdList;
      let lastMessage: string;
      let messageList = await getMessageList({
        api: api,
        id: idList![i],
      });
      console.log("messageList: " + messageList);
      if (idList !== null) {
        lastMessage = await getLastMessage({ api: api, id: idList![i] });
      }
      let memberListFactor = (
        <MessageMember
          name={friendProfile?.name}
          img_url={friendProfile?.imgUrl}
          last_message={lastMessage}
          setShowMessageModal={setShowMessageModal}
          setUserName={setUserName}
          setUserImgUrl={setUserImgUrl}
          setMyImgUrl={setMyImgUrl}
          messageListId={idList[i]}
          setMessageListId={setMessageListId}
          setMessageList={setMessageList}
          messageList={messageList}
          getMessageList={getMessageList}
          setMyUserId={setMyUserId}
          myUserId={profile?.userId}
          api={api}
        />
      );
      memberList.push(memberListFactor);
    }
    setMessageMemberList(memberList);
  };

  return !showMessageModal ? (
    <div className="flex justify-center items-center bg-gray-200 w-screen h-screen relative">
      <main className="items-center h-screen w-1/3 flex bg-white flex-col">
        <TopBar
          idList={accountList}
          imgUrl={imgUrl}
          setActingAccount={setActingAccount}
          balance={balance}
        />
        <div className="flex-1 overflow-scroll w-full mt-1">
          {messageMemberList}
        </div>
        <div className="w-full">
          <BottomNavigation />
        </div>
      </main>
    </div>
  ) : (
    <MessageRoom
      setShowMessageModal={setShowMessageModal}
      userName={userName}
      userImgUrl={userImgUrl}
      myImgUrl={myImgUrl}
      myUserId={myUserId}
      api={api!}
      actingAccount={actingAccount!}
      messageListId={messageListId!}
      messageList={messageList!}
    />
  );
}