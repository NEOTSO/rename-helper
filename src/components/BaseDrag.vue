<script setup lang="ts">
defineProps({
    isDragOver: Boolean,
});
</script>

<template>
    <div class="border border-dashed flex justify-center items-center h-40 cursor-default select-none" :class="isDragOver ? 'border-blue-400 text-blue-400' : 'border-gray-400'" border-gray-400 @dragenter="drag">拖拽文件到这里</div>
</template>
