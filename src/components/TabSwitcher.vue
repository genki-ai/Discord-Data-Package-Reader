<script setup lang="ts">
import { computed, ref } from 'vue';

    const props = defineProps(["tabPayload"])
    const pageIndex = ref(0)

    function switchTab(tabIndex: number) {
        pageIndex.value = tabIndex
    }
    const pageSwitch = computed(() => {
        return {currentPage: props.tabPayload[pageIndex.value]}
    })
</script>

<template>
    <div class="tabcontainer">
        <div class="tabs">
            <button v-for="(tabpage, i) in props.tabPayload" @click="switchTab(i)">{{ tabpage.name }}</button>
        </div>
        <component class="tabcontent" :is="pageSwitch.currentPage.payload" v-bind="pageSwitch.currentPage.props"/>
    </div>
</template>

<style scoped>
    .tabcontainer {
        display: flex;
        flex-direction: column;
        border: 2px solid white;
        height: 100%;
        box-sizing: border-box;
    }
    .tabs {
        border-bottom: 2px solid white;
    }
    .tabcontent {
        height: 100%;
        box-sizing: border-box;

        overflow-y: scroll;
    }
</style>
 

