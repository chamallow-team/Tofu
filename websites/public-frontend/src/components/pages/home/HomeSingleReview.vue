<template>
	<div class="border-2 rounded-lg p-3 border-sheen bg-greenish w-80">
		<!-- Head -->
		<div class="flex flex-row items-center justify-between">
			<img :src="imageUrl" v-if="imageUrl" alt="icon" class="w-16 h-16 rounded-full"/>
			<div class="flex flex-col items-center justify-start ml-4">
				<h3 class="text-left mr-auto">{{ author }}</h3>

				<!-- Stars -->
				<div class="flex flex-row justify-start gap-2">
					<svg width="35" xmlns="http://www.w3.org/2000/svg" height="35" fill="none"
							 v-for="n in [0,1,2,3,4]" :key="n">
						<g class="fills">
							<rect rx="0" ry="0" width="35" height="35"
										transform="matrix(1.000000, 0.000000, 0.000000, 1.000000, 0.000000, 0.000000)"
										class="frame-background"/>
						</g>
						<g style="fill: rgb(0, 0, 0);" class="frame-children">
							<path
									d="M15.733,4.681C16.386,3.111,18.614,3.111,19.267,4.681L22.304,11.983L30.185,12.615C31.882,12.750,32.570,14.868,31.277,15.976L25.273,21.120L27.106,28.809C27.501,30.466,25.700,31.774,24.248,30.888L17.500,26.766L10.752,30.888C9.300,31.774,7.499,30.465,7.894,28.809L9.727,21.120L3.723,15.976C2.430,14.868,3.118,12.750,4.815,12.615L12.696,11.983L15.733,4.681ZZ"
									clip-rule="evenodd" fill-rule="evenodd"
									:class="n >= note ? 'fill-gray-300 opacity-20' : 'fill-chrome-yellow'"
									class="fills"/>
						</g>
					</svg>
				</div>
			</div>
		</div>
		<p class="mt-2">{{ review }}</p>
	</div>
</template>

<script setup>
import {defineProps} from 'vue';

defineProps({
	author: String,
	note: Number,
	review: String,
	imageUrl: String,
})
</script>