import {createRouter, createWebHistory} from 'vue-router'
import i18nActions from '@/translation'

const routes = [
    {
        path: '/',
        redirect: '/fr',
    },
    {
        path: '/:locale?',
        beforeEnter: i18nActions.routeMiddleware,
        children: [
            {
                path: '',
                name: 'home',
                component: () => import('../views/HomeView.vue'),
            },
            {
                path: 'about',
                name: 'about',
                component: () => import('../views/AboutView.vue')
            },
            {
                path: "blog",
                name: "blog_home",
            },
            {
                path: "team",
                name: "team_page",
            },
            {
                path: "support",
                name: "support",
            },
            {
                path: "terms",
                name: "tos"
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

export default router
