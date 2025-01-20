<template>
	<footer class="mx-10 pr-10 pt-10 border border-onyx border-b-0 rounded rounded-b-none">
		<div class="flex flex-row justify-between w-full">
			<!-- img + name -->
			<router-link :to="{ name: 'home', params: { locale }}"
									 class="flex flex-row items-center justify-start p-10 group w-1/6">
				<img src="/logo.jpg" alt="icon" class="w-28 rounded-full mr-5">
				<h2 class="font-bold text-4xl my-auto group-hover:underline">Kady</h2>
			</router-link>

			<div class="flex flex-row justify-center items-center gap-10 text-lg">
				<router-link :to="{name: 'blog_home'}" class="hover:underline">{{ t('nav.blog') }}</router-link>
				<router-link :to="{name: 'features'}" class="hover:underline">{{ t('nav.features') }}</router-link>
				<router-link :to="{name: 'team_page'}" class="hover:underline">{{ t('nav.team') }}</router-link>
				<router-link :to="{name: 'support'}" class="hover:underline">{{ t('nav.help') }}</router-link>
				<router-link :to="{name: 'tos'}" class="hover:underline">{{ t('nav.tos') }}</router-link>
			</div>

			<div class="my-auto w-1/6">
				<Contacts :size="40"></Contacts>
			</div>
		</div>
		<p class="text-gray-400 text-center pb-6">© Kady Team 2025 - All rights reserved</p>
	</footer>
</template>

<script setup>
import {useI18n} from "vue-i18n";
import Contacts from "@/components/navigation/Contacts.vue";

const {t, locale} = useI18n();
</script>