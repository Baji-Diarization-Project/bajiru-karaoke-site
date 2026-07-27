<template>
  <div class="separator flex flex-col gap-1 py-1 text-[#818181]">
    <span class="w-full h-[1px] bg-[#818181]"></span>
    <p class="uppercase">{{ label }}</p>
  </div>
</template>

<script setup>
defineProps({
  label: { type: String, required: true },
})
</script>
