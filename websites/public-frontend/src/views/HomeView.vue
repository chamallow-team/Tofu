<template>
	<div>
		<!-- Main presentation -->
		<div
				class="px-5 sm:px-10 md:px-40 xl:px-32 pt-10 w-full h-screen flex flex-col md:flex-row items-center justify-center">
			<!-- Left -->
			<div class="flex flex-col justify-center items-center w-full h-3/4 lg:h-full">
				<!-- Icon + name + desc -->
				<div class="flex flex-col md:flex-row items-center w-full justify-center md:justify-start">
					<img alt="Kady icon" src="logo.jpg" class="w-1/2 min-w-52 xl:w-[22rem] rounded-full">
					<div class="mt-10 md:ml-16 md:mt-0">
						<h1 class="text-4xl md:text-7xl text-center md:text-left mx-auto md:mx-0 pb-5 md:pr-5 font-black border-b-[5px] border-b-cream w-full md:w-fit">
							Kady
						</h1>
						<p class="text-2xl xl:text-4xl mt-5 w-full md:w-7/12 font-semibold">{{ t('home.main_description') }}</p>
					</div>
				</div>

				<!-- Button -->
				<a target="_blank" :href="INVITE_LINK"
					 class="mt-auto mb-10 md:mt-10 group select-none">

					<div
							class="bg-opacity-50 rounded-full absolute z-0 blur-3xl -translate-y-1/2 md:translate-y-6 -translate-x-56 md:-translate-x-0 md:left-40"
							style="background-color: #5865f2; height: 75px; width: 75px;"></div>

					<div
							class="flex flex-row items-center absolute md:rela left-0 md:left-40 mx-5 z-10 bg-discord py-2 px-3 rounded -translate-y-full md:-translate-y-0">
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
				</a>
			</div>

			<!-- Last update on development -->
			<div class="absolute z-10 right-7 w-4/12 hidden 2xl:block">
				<div class="bg-opacity-50 rounded-full absolute z-0 blur-3xl translate-y-20 -translate-x-6"
						 style="background-color: #2a4c4a; height: 133px; width: 133px;"></div>

				<div class="bg-dark absolute z-10 -translate-y-1/2 border border-grayish rounded-xl">
					<!-- Head -->
					<p class="py-5 px-6 border-b border-b-grayish text-xl font-bold ">Update on the development</p>
					<!-- body -->
					<div class="p-6">
						<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et
							dolore magna aliqua. Tempor id eu nisl nunc mi ipsum. Morbi non arcu risus quis varius quam. Dolor purus
							non enim praesent elementum facilisis leo. Nisi porta lorem mollis aliquam ut porttitor leo a. At lectus
							urna duis convallis convallis. Cursus mattis molestie a iaculis at erat pellentesque. Vitae nunc sed velit
							dignissim sodales ut eu sem. Suscipit tellus mauris a diam. Aenean et tortor at risus viverra adipiscing.
						</p>

						<div class="flex flex-row justify-between items-center mt-3">
							<!-- TODO ajouter lien vers le blog + dynamisation de cette information -->
							<router-link to="/"
													 class="py-2 px-3 bg-cream bg-opacity-70 hover:bg-opacity-100 cursor-pointer rounded select-none">
								Read more
							</router-link>
							<p class="text-gray-400 font-light">{{ t('blog.by', {name: 'Lynn & Little'}) }}</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<p class="my-96">A</p>
	</div>
</template>

<script setup>
import {useI18n} from "vue-i18n";
import {INVITE_LINK} from "@/constants";

const {t} = useI18n()
</script>
