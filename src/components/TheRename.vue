<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import { ref } from "vue";
import BaseDrag from "../components/BaseDrag.vue";

const separator = ref("#");
const isDragOver = ref(false);
const selectedFiles = ref([]);

listen("files-selected", (event) => {
    selectedFiles.value = [event.payload.files];
});

const rename = () => {
    invoke("rename", { files: selectedFiles.value, separator: separator.value });
};

listen("tauri://file-drop-hover", (event) => {
    isDragOver.value = true;
});

listen("tauri://file-drop", (event) => {
    console.log(event);
    console.log(event.payload);
    isDragOver.value = false;
    selectedFiles.value = event.payload;
});
</script>

<template>
    <div class="px-4 py-5 flex flex-col h-full">
        <div class="flex justify-between">
            <div class="flex">
                <input class="border border-gray-300 outline-none px-2" type="text" placeholder="请输入混淆文字" :value="separator" />
            </div>
            <div class="flex">
                <div class="button" @click="log">还原</div>
                <div class="button ml-2" @click="rename">混淆</div>
            </div>
        </div>
        <ul class="text-sm my-2 list-inside list-square">
            <li v-for="item in selectedFiles">{{item}}</li>
        </ul>
        <base-drag :is-drag-over="isDragOver" class="flex-grow" />
    </div>
</template>

<style scoped>
.button {
    @apply bg-gray-300 px-2 cursor-pointer;
}
</style>
