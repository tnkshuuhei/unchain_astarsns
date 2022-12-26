import { useRouter } from "next/router";
import { FC, useState, useEffect } from "react";

import { ApiPromise } from "@polkadot/api";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import type { PostType } from "../hooks/postFunction";

import { balenceOf } from "../../hooks/FT";
import { connectToContract } from "../../hooks/connect";
import { getIndividualPost } from "../../hooks/postFunction";
import {
  checkCreatedInfo,
  createProfile,
  getFollowerList,
  getFollowingList,
  getProfileForProfile,
} from "../../hooks/profileFunction";

const DisconnectButton: FC = () => {
  const [imgUrl, setImgUrl] = useState("");
  const [isCreatedProfile, setIsCreatedProfile] = useState(true);
  const [isCreatedFnRun, setIsCreatedFnRun] = useState(false);
  const [name, setName] = useState("");
  const [individualPostList, setIndividualPostList] = useState<PostType[]>([]);

  const [showSettingModal, setShowSettingModal] = useState(false);
  const [isSetup, setIsSetup] = useState(false);
  const [api, setApi] = useState<ApiPromise>();
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAccount, setActingAccount] = useState<InjectedAccountWithMeta>();
  const [followingList, setFollowingList] = useState<Array<string>>([]);
  const [followerList, setFollowerList] = useState<Array<string>>([]);
  const [balance, setBalance] = useState<string>("0");
  const router = useRouter();

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
    if (!isSetup) return;
    getProfileForProfile({
      api: api,
      userId: actingAccount?.address,
      setImgUrl: setImgUrl,
      setName: setName,
    });
    getIndividualPost({
      api: api,
      actingAccount: actingAccount,
      setIndividualPostList: setIndividualPostList,
    });
    getFollowingList({
      api: api,
      userId: actingAccount?.address,
      setFollowingList: setFollowingList,
    });
    getFollowerList({
      api: api,
      userId: actingAccount?.address,
      setFollowerList: setFollowerList,
    });
    balenceOf({
      api: api,
      actingAccount: actingAccount!,
      setBalance: setBalance,
    });
    if (isCreatedFnRun) return;
    checkCreatedInfo({
      api: api,
      userId: actingAccount?.address!,
      setIsCreatedProfile: setIsCreatedProfile,
    });
    if (isCreatedProfile) return;
    createProfile({ api: api, actingAccount: actingAccount! });
    setIsCreatedFnRun(true);
  }, []);

  return (
    <button
      onClick={() => {
        console.log("clicked Disconnect.")
        setIsSetup(false);
        setShowSettingModal(false);
        setAccountList([]);
        router.push("/");
      }}
      className="z-10 text-xl text-white items-center flex justify-center h-14 bg-[#003AD0] hover:bg-blue-700  py-2 px-4 rounded-full mr-4"
    >
      Disconnect Wallet
    </button>
  );
};

export default DisconnectButton;