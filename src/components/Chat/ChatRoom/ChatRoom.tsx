import React, {ChangeEvent, EventHandler, FormEvent, useEffect, useState} from 'react';
import Header from './Resources/Header';
import {ChannelInfo, Chat, ChatChannel, UserInfo, ChannelMetaStruct, ChannelMetaType, FeedType} from 'node-kakao/dist';
import Chats from '../Chats';
import ChatInput from '../Item/ChatInput';
import Background from './Resources/Background';
import { extractRoomName } from '../Utils/RoomInfoExtracter';

export interface ChatRoomProps {
    channel: ChatChannel
    chatList: Chat[]
    onInputChange: EventHandler<ChangeEvent<HTMLInputElement>>
    onSubmit: EventHandler<FormEvent>
    inputValue: string
}

const ChatRoom: React.FC<ChatRoomProps> = ({channel, chatList, onInputChange, onSubmit, inputValue}) => {
    const [title, setTitle] = useState('')
    useEffect(() => {
        channel.getChannelInfo().then((ch: ChannelInfo) => {
            const userInfoList = ch.UserIdList.map((id) => ch.getUserInfoId(id)).filter((v, i) => i < 5 && v != null) as UserInfo[];
            
            const name = extractRoomName(ch, userInfoList)

            setTitle(name)
        });
    }, [channel])
    return (
        <Background>
            <Header title={title}/>
            <Chats channel={channel} chatList={chatList}/>
            <ChatInput onChange={onInputChange} onSubmit={onSubmit} value={inputValue}/>
        </Background>
    )
};

export default ChatRoom;