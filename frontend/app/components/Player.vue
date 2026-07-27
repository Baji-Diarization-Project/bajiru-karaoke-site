<template>
  <div class="flex flex-row p-6 py-2 gap-4 h-16 bg-[#2A242A] shadow-sm text-white">
    <div data="song-info" class="flex flex-row justify-between flex-1">
      <div class="flex flex-row gap-4 items-center flex-1">
        <div class="flex flex-row gap-2 p-1">
          <img src="" alt="" class="w-10 h-10" />
          <div class="flex flex-col gap-1">
            <p class="text-sm">Song Title</p>
            <span class="text-[#AAAAAA] text-xs">Song Title</span>
          </div>
        </div>

        <div class="flex flex-row gap-2">
          <button class="w-6 h-6 flex items-center justify-center"><span class="i-solar:stopwatch-bold"></span></button>
          <button class="w-6 h-6 flex items-center justify-center">
            <span class="i-solar:info-circle-bold"></span>
          </button>
          <button class="w-6 h-6 flex items-center justify-center">
            <span class="i-solar:playlist-minimalistic-3-bold"></span>
          </button>
        </div>
      </div>

      <div class="flex flex-row gap-2 items-center">
        <button class="w-6 h-6 flex items-center justify-center">
          <span class="i-solar:volume-cross-bold"></span>
        </button>
        <input type="range" class="w-24" />
      </div>
    </div>

    <div class="flex flex-row items-center gap-2">
      <button class="w-8 h-8 flex items-center justify-center"><span class="i-solar:shuffle-linear"></span></button>
      <button class="w-8 h-8 flex items-center justify-center">
        <span class="i-solar:skip-previous-linear"></span>
      </button>
      <button class="w-8 h-8 flex items-center justify-center bg-[#83FFB010] rounded-full text-[#83FFB0]">
        <span class="i-solar:play-bold"></span>
      </button>
      <button class="w-8 h-8 flex items-center justify-center"><span class="i-solar:skip-next-linear"></span></button>
      <button class="w-8 h-8 flex items-center justify-center"><span class="i-solar:repeat-linear"></span></button>
    </div>

    <div class="flex flex-row items-center flex-1 gap-2">
      <span>0:12</span>
      <input class="flex-1" type="range" />
      <span>5:57</span>
    </div>

    <audio ref="audio" />
  </div>
</template>

<script setup>
import { useMediaControls } from '@vueuse/core'
const props = defineProps({
  src: String,
})

const audio = useTemplateRef('audio')
const player = useMediaControls(audio, { src: props.src })
</script>
