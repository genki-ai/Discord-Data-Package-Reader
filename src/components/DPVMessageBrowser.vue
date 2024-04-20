<script setup lang="ts">
    import { ref, watch } from 'vue';
    import { IMessage } from '../interfaces/IMessage';
    import { loadedChannel } from '../modules/dataStore';
    import { invoke } from '@tauri-apps/api';
    import TabSwitcher from './TabSwitcher.vue';
    import DPChannelList from "./DPChannelList.vue"

    const props = defineProps(["channelList", "packagePath"])

    interface ITabPayload {
        name: String,
        payload: any,
        props: any
    }

    const loadedMessages = ref<Array<IMessage>>(new Array<IMessage>());

    watch(loadedChannel, async () => {
        const result: Array<IMessage> = await invoke("load_message", {zipPath: props.packagePath.value, channelId: loadedChannel.value.channelID})
        loadedMessages.value = result;
    }, {deep: true})

    const pageArray: Array<ITabPayload> = [{"name": "DM", "payload": DPChannelList, "props": {"channelList": props.channelList, "filterType": 1}}, {"name": "Group", "payload": DPChannelList, "props": {"channelList": props.channelList, "filterType": 3}}, {"name": "Server", "payload": DPChannelList, "props": {"channelList": props.channelList, "filterType": 0}}, {"name": "VC", "payload": DPChannelList, "props": {"channelList": props.channelList, "filterType": 2}}]
</script>

<template>
    <div class="container">
        <div class="channelList">
            <TabSwitcher :tab-payload="pageArray"/>
        </div>
        <div class="messageList">
            <div v-for="message in loadedMessages" class="message">
                {{ message.Contents }}
            </div>
        </div>
        <div class="statusPane"></div>
    </div>
</template>

<style scoped>
.container {  display: grid;
    height: 100%;
    grid-template-columns: 0.5fr 1.5fr;
    grid-template-rows: 1.6fr 0.4fr;
    gap: 1em 1em;
    grid-auto-flow: row;
    grid-template-areas:
        "channelList messageList"
        "channelList statusPane";
}

.channelList { grid-area: channelList; 
    overflow: hidden;
}

.messageList { grid-area: messageList; 
    border: white 2px solid;
    overflow-y: scroll;
}

.statusPane { grid-area: statusPane; 
    border: white 2px solid;
}

.channel {
    border: white 1px solid;
    padding: 1em 0 1em 0;
    text-align: center;
}
.channel:hover, .message:hover {
    border: yellow 2px solid;
    cursor: pointer;
}

.message {
    overflow-wrap: break-word;
    padding: 0.2em;
}

</style>