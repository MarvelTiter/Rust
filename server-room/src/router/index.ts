import { createRouter, RouteRecordRaw, createWebHashHistory } from 'vue-router'
const routes: RouteRecordRaw[] = [
    {
        path: "/",
        component: () => import("../views/index.vue")
    },
    {
        path: '/room',
        component: () => import("../views/room/index.vue")
    }
]
export default createRouter({
    history: createWebHashHistory(),
    routes: routes
})
