<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue"
import TabSwitcher from "./components/TabSwitcher.vue"

import DPVProfile from "./components/DPVProfile.vue";
import DPVMessageBrowser from "./components/DPVMessageBrowser.vue";

import { ref } from 'vue';
import { IPackageIndex } from "./interfaces/IPackageIndex";
import { IChannel } from "./interfaces/IChannel";

interface ITabPayload {
        name: String,
        payload: any,
        props: any
    }


let packageIndexRef = ref<IPackageIndex>({
    "account": {"id": "...", "username": "", "discriminator": 0, "global_name": "", "email": "", "verified": false, "has_mobile": false, "phone": "", "ip": ""},
    "channal_list": new Array<IChannel>()
})

//TODO: make this more graceful
let packagePath = ref();

function updatePackageIndex(dpIndex: IPackageIndex, dpPath: String) {
    packageIndexRef.value = dpIndex
    packagePath.value = dpPath
}

const pageArray: Array<ITabPayload> = [{"name": "User Profile", "payload": DPVProfile, "props": {"userData": packageIndexRef}}, {"name": "Message Browser", "payload": DPVMessageBrowser, "props": {"channelList": packageIndexRef, "packagePath": packagePath}}]

</script>

<template>
    <div class="container">
        <div class="appMenuBar">
            <MenuBar @load-index="updatePackageIndex"/>
        </div>
        <div class="appDPViewer">
            <TabSwitcher :tab-payload="pageArray"/>
        </div>
        <div class="appStatus">
            <p>...</p>
        </div>
    </div>
</template>

<style scoped>
.container {  display: grid;
    padding: 0.2em;
    box-sizing: border-box;
    height: inherit;
    grid-template-columns: 1fr;
    grid-template-rows: 28px 1fr 24px;
    grid-auto-flow: row;
    grid-template-areas:
        "appMenuBar"
        "appDPViewer"
        "appStatus";
}

.appMenuBar { grid-area: appMenuBar; }

.appDPViewer { grid-area: appDPViewer; 
    overflow: hidden;
}

.appStatus { grid-area: appStatus; 
    p {
        margin: 0;
    }
}

</style>
