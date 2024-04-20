<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { IPackageIndex } from '../interfaces/IPackageIndex';

const emit = defineEmits(["loadIndex"])

async function openPackage() {
    const selected = await open({
        multiple: false,
        filters: [{
            name: 'Package',
            extensions: ['zip']
        }]
    });
    const dpdata: IPackageIndex = await invoke("index_package", { zipPath: selected})
    console.log(dpdata);
    emit("loadIndex", dpdata, selected)
}
</script>

<template>
    <div class="btns-menu">
        <button @click="openPackage()">Open Package</button>
        <button style="float: right;">Exit</button>
    </div>  
</template>
