<template>
  <li>
    <NuxtLink :to="to" tag="li" class="flex flex-row gap-2 items-center group">
      <div class="w-8 h-8 flex items-center justify-center">
        <span :class="['w-6 h-6', iconClass]" />
      </div>
      <span :class="[{ 'text-[#83FFB0]': route.path == to }, 'group-hover:text-[#C7FFDB] transition duration-[144ms]']">
        {{ label }}</span
      >
    </NuxtLink>
  </li>
</template>

<script setup>
defineProps({
  label: { type: String, required: true },
  to: { type: String, required: true },
  iconClass: { type: String, default: 'i-solar:box-minimalistic-outline' },
})
const route = useRoute()
</script>
