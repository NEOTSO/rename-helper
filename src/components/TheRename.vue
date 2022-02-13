<script setup lang="ts">
import BaseFileList from "./BaseFileList.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { emit, listen } from "@tauri-apps/api/event";

// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
// const invoke = window.__TAURI__.invoke

// Invoke the command

const selectFolders = async () => {
    const result = await open({ directory: true });
    console.log(result);
    if (result) {
        console.log("xxxx");
        invoke("rename_files", { files: result });
    }
};

const selectFiles = async () => {
    const result = await open({ directory: false, multiple: true });
    console.log(result);
    if (result) {
        console.log("xxxx");
        invoke("rename_files", { files: result, separator: '#' });
    }
};
</script>

<template>
    <div>
        <div class="flex justify-between px-4 py-5">
            <div class="flex">
                <div class="button" @click="selectFolders">打开文件夹</div>
                <div class="button ml-2" @click="selectFiles">打开文件</div>
                <input class="border border-gray-300 outline-none px-2 ml-2" type="text" placeholder="请输入混淆文字" />
            </div>
            <div class="flex">
                <div class="button">还原</div>
                <div class="button ml-2">混淆</div>
            </div>
        </div>
        <BaseFileList />
    </div>
</template>

<style scoped>
.button {
    @apply bg-gray-300 px-2 cursor-pointer;
}
</style>
