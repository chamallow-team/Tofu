<template>
	<div>
		<!-- Nav desktop -->
		<nav
				class="hidden sm:flex flex-row justify-between items-center py-8 px-10 fixed top-0 left-0 w-full"
				style="z-index: 9999">
			<router-link :to="{ name: 'home', params: { locale }}" :class="{'invisible': isInHome()}">
				<img src="/logo.jpg" alt="home" class="w-12 rounded-full mt-auto">
			</router-link>

			<div class="flex items-center justify-between gap-16">
				<div class="flex flex-row justify-between gap-10">
					<router-link :to="{name: 'blog_home'}" class="hover:underline">{{ t('nav.blog') }}</router-link>
					<router-link :to="{name: 'features'}" class="hover:underline">{{ t('nav.features') }}</router-link>
					<router-link :to="{name: 'team_page'}" class="hover:underline">{{ t('nav.team') }}</router-link>
					<router-link :to="{name: 'support'}" class="hover:underline">{{ t('nav.help') }}</router-link>
					<router-link :to="{name: 'tos'}" class="hover:underline">{{ t('nav.tos') }}</router-link>
				</div>

				<Contacts></Contacts>

				<TranslationChanger></TranslationChanger>
			</div>
		</nav>

		<!-- Nav mobile-->
		<nav
				class="sm:hidden flex flex-row justify-between items-center my-4 pl-10 pr-16 fixed top-0 left-0 w-full"
				style="z-index: 9999">

			<router-link :to="{ name: 'home', params: { locale }}" :class="{'hidden': isInHome()}"
									 @click="showNavigation = false">
				<img src="/logo.jpg" alt="home" class="w-12 rounded-full mt-auto">
			</router-link>

			<svg class="fill-white" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"
					 @click="showNavigation = !showNavigation">
				<path d="M4 6h16v2H4zm0 5h16v2H4zm0 5h16v2H4z"></path>
			</svg>

			<div v-if="showNavigation"
					 class="absolute left-1/2 -translate-x-1/2 translate-y-60 w-5/6 bg-dark p-6 border border-grayish rounded">
				<div class="flex flex-col text-center justify-between gap-10 text-xl mb-10">
					<router-link :to="{name: 'blog_home'}" class="hover:underline" @click="showNavigation = false">
						{{ t('nav.blog') }}
					</router-link>
					<router-link :to="{name: 'features'}" class="hover:underline" @click="showNavigation = false">
						{{ t('nav.features') }}
					</router-link>
					<router-link :to="{name: 'team_page'}" class="hover:underline" @click="showNavigation = false">
						{{ t('nav.team') }}
					</router-link>
					<router-link :to="{name: 'support'}" class="hover:underline" @click="showNavigation = false">{{
							t('nav.help')
						}}
					</router-link>
					<router-link :to="{name: 'tos'}" class="hover:underline" @click="showNavigation = false">{{
							t('nav.tos')
						}}
					</router-link>
				</div>
				<!-- Contact -->
				<Contacts></Contacts>
			</div>

			<TranslationChanger></TranslationChanger>
		</nav>
	</div>
</template>

<script setup>
import TranslationChanger from "@/components/navigation/TranslationChanger.vue";
import {useI18n} from "vue-i18n";
import {ref} from "vue";
import {useRouter} from "vue-router";
import {SUPPORTED_LOCALES} from "@/i18n";
import Contacts from "@/components/navigation/Contacts.vue";

const router = useRouter();

function isInHome() {
	return SUPPORTED_LOCALES.some((l) =>
			router.currentRoute.value.fullPath.replace(/^\//, "") === l)
}

const {t, locale} = useI18n();

const showNavigation = ref(false);
</script>