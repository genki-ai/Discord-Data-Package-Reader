<script setup lang="ts">
    import { loadedChannel } from "../modules/dataStore"
    const props = defineProps(["channelList", "filterType"])
    function loadMessage(channelID: string){
        console.log("loading channel " + channelID)
        loadedChannel.value.channelID = channelID
    }
</script>

<template>
    <div class="container">
        <div class="channel" v-for="channel in props.channelList.value.channel_list">
            <div @click="loadMessage(channel.id)" v-if="channel.type == $props.filterType"> {{ channel.name }}</div>
        </div>
    </div>
</template>

<style scoped>
.container {
    overflow-y: scroll;
}
/* .channel {
    border: white 1px solid;
    padding: .1em 0 .1em 0;
    text-align: center;
} */
.channel:hover, .message:hover {
    border: yellow 2px solid;
    cursor: pointer;
}
</style>