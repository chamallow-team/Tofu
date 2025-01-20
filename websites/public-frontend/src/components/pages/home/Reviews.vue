<template>
	<div class="flex flex-col justify-center items-center">
		<h2 class="text-center text-3xl md:text-4xl font-bold mb-10">{{ t("home.reviews.t1") }}</h2>

		<!-- Container -->
		<div class="py-10 px-16 overflow-hidden border-2 border-onyx w-3/4 rounded-xl bg-dark">
			<!-- Title + accent -->
			<div class="relative">
				<h2 class="text-center text-3xl md:text-4xl font-bold mb-10 z-0">{{ t("home.reviews.t2") }}</h2>
				<div
						class="bg-greenish w-64 h-64 rounded-full absolute left-1/2 -translate-x-1/2 z-10 -translate-y-20"
						style="filter: blur(128px)"></div>
			</div>

			<!-- Reviews -->
			<div class="w-full">
				<!-- Border gradients -->
				<div
						class="bg-gradient-to-r from-dark via-dark to-transparent  h-60 w-20 absolute z-30 -translate-y-5  left-64"></div>
				<div
						class="bg-gradient-to-l from-dark via-dark to-transparent  h-60 w-60 absolute z-30 -translate-y-5 right-64"></div>

				<!-- Container -->
				<div
						class="flex flex-row justify-center gap-10 overflow-x-hidden relative z-20 w-fit left-1/2 -translate-x-1/2">
					<HomeSingleReview v-for="(review, index) in reviews"
														:key="index"
														:author="review.author" :note="review.note" :review="review.review"
														:image-url="review.imageUrl">
					</HomeSingleReview>
				</div>

			</div>
		</div>

	</div>
</template>

<script setup>
import {useI18n} from "vue-i18n";
import HomeSingleReview from "@/components/pages/home/HomeSingleReview.vue";

const {t} = useI18n()

const reviews = [
	{
		author: "Aubin",
		note: 5,
		review: "Je trouve le bot très simple d'utilisation, pratique, et avec une belle interface ! GG NOTES=  ⭐⭐⭐⭐⭐",
		imageUrl: "https://cdn.discordapp.com/avatars/821061709292634152/5e7a6c427da8ccdc248db9ee5ae6385c.png?size=4096"
	},
	{
		author: ".Sambatali",
		note: 4,
		review: "Bot propre, facile à utiliser et comprendre, vraiment parfait je mettrais la note de ⭐⭐⭐⭐ car on peut toujours mieux faire. 👀",
		imageUrl: "https://cdn.discordapp.com/avatars/619991341937459200/b2b7afedf48de2376d1811421f39bdbc.png?size=4096"
	},
	{
		author: "Duskshall",
		note: 5,
		review: "Je trouve le bot facile à config qui est un grand atout des tres varie GG NOTES ⭐⭐⭐⭐⭐ rien a dire parfait",
		imageUrl: "https://cdn.discordapp.com/avatars/979356112635711538/5ac7b01d3c8099013cd9c9c505bebae4.png?size=4096"
	},
	{
		author: "Sushi",
		note: 5,
		review: "J'adore plus que tout le welcome sur kady je l'utilise d'ailleurs pour sa ! Une note de  5 étoiles supplément cheddar",
		imageUrl: "https://cdn.discordapp.com/avatars/816708235440029706/24ab9545949a6297642d9455bd508165.png?size=4096"
	}
]
</script>