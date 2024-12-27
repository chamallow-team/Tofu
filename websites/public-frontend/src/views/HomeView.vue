<template>
	<div>
		<!--		<img alt="Vue logo" src="logo.jpg">-->
		<p>{{ t('home') }}</p>
	</div>
</template>

<script setup>
import {useI18n} from "vue-i18n";

const {t} = useI18n()
</script>
