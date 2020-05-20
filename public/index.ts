import localforage from 'localforage';
import {v4} from 'uuid';

// @ts-ignore
global.getClientName = async (): Promise<string> => {
  try {
    const uuid = await localforage.getItem('client_name') as string | null
    if (uuid) return uuid
    else throw new Error()
  } catch (e) {
    let clientName = require('os').hostname();
    localforage.setItem('client_name', clientName)
        .catch((error) => {
          console.log(error);
        });
    return clientName;
  }
}
// @ts-ignore
global.createNewUUID = (): string => {
  return Buffer.from(v4()).toString('base64');
}
// @ts-ignore
global.getUUID = async (): Promise<string> => {
  try {
    const uuid = await localforage.getItem('uuid') as string | null
    if (uuid) return uuid
    else throw new Error()
  } catch (e) {
    // @ts-ignore
    let uuid = global.createNewUUID();
    localforage.setItem('uuid', uuid)
        .catch((error) => {
          console.log(error);
        });
    return uuid;
  }
}

// @ts-ignore
global.talkClient = new (require('node-kakao/dist').TalkClient)(global.getClientName());

nw.Window.open('localhost:3000', { frame: false, width: 800, height: 600, show_in_taskbar: true }, (win) => {});