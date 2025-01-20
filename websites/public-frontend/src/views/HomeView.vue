<template>
	<div>
		<!-- Main presentation -->
		<MainStart></MainStart>

		<p class="text-center mb-48">FEATURES TODO</p>

		<Reviews></Reviews>

		<!-- Last, but not least -->
		<h2 class="mt-48 font-black text-7xl text-center">{{ t('home.no_regret') }}</h2>

		<div class="flex flex-col items-center justify-center gap-10 mb-48">
			<p class="mt-32 font-bold text-4xl text-center">{{ t('home.start') }}</p>
			<div class="bg-discord py-2 px-3 rounded flex flex-row items-center justify-start w-fit">
				<svg width="55" xmlns="http://www.w3.org/2000/svg" height="55" fill="none"
						 class="group-hover:-rotate-12 transition-all duration-100">
					<g style="fill: rgb(0, 0, 0);">
						<path
								d="M33.962,9.763C33.507,10.578,33.102,11.419,32.748,12.283C29.271,11.733,25.729,11.733,22.252,12.283C21.898,11.419,21.493,10.578,21.038,9.763C17.762,10.322,14.578,11.324,11.573,12.742C6.199,20.497,3.762,29.913,4.698,39.302C8.200,41.935,12.131,43.943,16.317,45.238C17.270,43.978,18.120,42.645,18.860,41.250C17.495,40.746,16.182,40.109,14.942,39.348C15.282,39.122,15.604,38.869,15.904,38.592C23.232,42.121,31.768,42.121,39.096,38.592C39.417,38.867,39.737,39.119,40.058,39.348C38.811,40.103,37.500,40.748,36.140,41.273C36.849,42.700,37.677,44.064,38.615,45.352C42.795,44.062,46.719,42.053,50.210,39.417C51.169,30.026,48.730,20.603,43.335,12.856C40.364,11.410,37.211,10.369,33.962,9.763ZZM19.892,33.940C17.508,33.768,15.687,31.744,15.767,29.356C15.675,26.964,17.503,24.932,19.892,24.773C22.281,24.932,24.109,26.964,24.017,29.356C24.109,31.749,22.281,33.780,19.892,33.940ZZM35.108,33.940C32.725,33.768,30.903,31.744,30.983,29.356C30.891,26.964,32.719,24.932,35.108,24.773C37.503,24.921,39.338,26.959,39.233,29.356C39.338,31.753,37.503,33.792,35.108,33.940ZZ"
								style="fill: rgb(255, 255, 255);" class="fills"/>
					</g>
				</svg>
				<h2 class="text-xl font-semibold ml-3 px-5 w-fit">{{ t('home.add_kady') }}</h2>
			</div>
		</div>


		<Footer></Footer>
	</div>
</template>

<script setup>
import MainStart from "@/components/pages/home/MainStart.vue";
import Reviews from "@/components/pages/home/Reviews.vue";
import Footer from "@/components/navigation/Footer.vue";
import {useI18n} from "vue-i18n";

const {t} = useI18n();
</script>
