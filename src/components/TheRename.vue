<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { ref, Ref } from "vue";
import BaseDrag from "../components/BaseDrag.vue";

const mode: Ref<string> = ref("字符分隔");
const keywords: Ref<string> = ref("❤");
const isDragOver: Ref<boolean> = ref(false);
const selectedFiles: Ref<string[]> = ref([]);
const message: Ref<string> = ref("");

const changeMode = (e: Event) => {
    console.log((e.target as HTMLInputElement).value);
    mode.value = (e.target as HTMLInputElement).value;
};

const restore = () => {
    invoke("restore", { files: selectedFiles.value, keywords: keywords.value });
};

const rename = () => {
    invoke("rename", { files: selectedFiles.value, keywords: keywords.value });
};

const deleteStr = () => {
    invoke("delete_str", { files: selectedFiles.value, keywords: keywords.value });
}

interface IPayloadFiles {
    files: string[];
}

interface IPayloadMessage {
    message: string;
}

listen("files-selected", (event: TauriEvent<IPayloadFiles>) => {
    selectedFiles.value = event.payload.files;
});

listen("done", (event: TauriEvent<IPayloadMessage>) => {
    message.value = event.payload.message;
});

listen("tauri://file-drop-hover", (event: TauriEvent<null>) => {
    isDragOver.value = true;
});

listen("tauri://file-drop", (event: TauriEvent<string[]>) => {
    message.value = "";
    isDragOver.value = false;
    selectedFiles.value = event.payload;
});
</script>

<template>
    <div class="px-4 py-5 flex flex-col h-full">
        <div class="flex justify-between">
            <div class="flex">
                <select class="outline-none" @change="changeMode">
                    <option value="字符分隔">字符分隔</option>
                    <option value="删除文字">删除文字</option>
                </select>
            </div>
            <div class="flex">
                <input class="border border-gray-300 outline-none px-2" type="text" placeholder="请输入混淆文字" v-model="keywords" />
            </div>
            <div v-if="mode === '字符分隔'" class="flex">
                <div class="button" @click="restore">还原</div>
                <div class="button ml-2" @click="rename">混淆</div>
            </div>
            <div v-if="mode === '删除文字'" class="flex">
                <div class="button" @click="deleteStr">确认删除</div>
            </div>
        </div>
        <p v-if="message">{{ message }}</p>
        <ul v-else class="text-sm my-2 list-inside list-square">
            <li v-for="item in selectedFiles">{{ item }}</li>
        </ul>
        <base-drag :is-drag-over="isDragOver" class="flex-grow" />
    </div>
</template>

<style scoped>
.button {
    @apply bg-gray-300 px-2 cursor-pointer;
}
</style>
