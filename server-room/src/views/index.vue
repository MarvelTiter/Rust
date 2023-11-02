<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter()
const createRoom = () => {
    router.push('room')
}

</script>

<template>
    <div class="container h-full w-full flex flex-col lg:flex-row">
        <div class="flex-1 flex flex-col justify-center items-center">
            <input type="text"
                   class="border outline-0 p-2 rounded mb-2"
                   placeholder="Ip Address">
            <button class="border rounded p-2 hover:bg-gray-200">搜索房间</button>
        </div>
        <div class="border"></div>
        <div class="flex-1 flex justify-center items-center">
            <button class="border rounded p-2 hover:bg-gray-200"
                    @click="createRoom">创建房间</button>
        </div>
    </div>
</template>
