<script lang="ts" setup>import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
const user = ref<string>('')
const dialog = ref<HTMLDialogElement>()
const router = useRouter()
const exit = () => {
    router.go(-1)
}
const close = () => {
    console.log(1)
}
onMounted(() => {
    user.value = localStorage.getItem('UserName') ?? ''
    if (user.value == '') {
        dialog.value?.showModal()
    }
})
</script>

<template>
    <label>{{ user }}</label>

    <dialog ref="dialog"
            class="border rounded">
        <form method="dialog">
            <h3>输入用户昵称</h3>
            <input class="border outline-0 p-2 rounded mb-2">
            <div>
                <button @click="close">确定</button>
                <button>关闭</button>
            </div>
        </form>
    </dialog>
    <el-popover placement="top"
                trigger="click">
        <template #reference>
            <div class="fixed border right-4 bottom-4 w-8 h-8 rounded-3xl flex justify-center items-center">
                <button>+</button>
            </div>
        </template>
        <div>
            <button>退出</button>
        </div>
    </el-popover>
</template>
