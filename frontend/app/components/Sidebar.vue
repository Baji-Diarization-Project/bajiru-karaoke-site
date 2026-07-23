<script setup lang="ts">
import NavItem from './sidebar/NavItem.vue'
</script>

<template>
  <aside class="w-[320px] flex flex-col p-4 gap-6 h-full bg-[#3D313D] shadow-md shadow-black">
    <header class="flex gap-2 items-center">
      <span class="i-solar:music-notes-outline w-8 h-8 bg-linear-to-b from-[#FD67AB] via-[#A9D0F0] to-[#83FFB0]" />
      <h1 class="text-xl bg-linear-to-r from-[#FD67AB] via-[#A9D0F0] to-[#83FFB0] color-transparent bg-clip-text">
        Karaoke Player
      </h1>
    </header>
    <ul class="flex flex-col gap-2 text-[#DCDDDF] list-none p-0">
      <NavItem to="/" label="Home" iconClass="i-solar:box-minimalistic-outline" />
      <NavItem to="/songs" label="Search" />
      <NavItem to="/..." label="Random Song" />
      <NavItem to="/..." label="Explore" />
      <NavItem to="/..." label="Artists" />
      <NavItem to="/..." label="Karaoke Setlist" />
      <NavItem to="/..." label="About" />
      <NavItem to="/..." label="Dashboard" />
      <div class="separator flex flex-col gap-1 py-1 text-[#818181]">
        <span class="w-full h-[1px] bg-[#818181]"></span>
        <p class="uppercase">Your Library</p>
      </div>
      <NavItem to="/..." label="Your Favorites" />
      <NavItem to="/..." label="Your Playlists" />
    </ul>
  </aside>
</template>
