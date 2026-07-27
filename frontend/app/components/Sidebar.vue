<script setup>
import NavItem from './sidebar/NavItem.vue'
import Separator from './sidebar/Separator.vue'
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
      <NavItem to="/todo" label="Random Song" />
      <NavItem to="/todo" label="Explore" />
      <NavItem to="/todo" label="Artists" />
      <NavItem to="/todo" label="Karaoke Setlist" />
      <NavItem to="/todo" label="About" />
      <NavItem to="/todo" label="Dashboard" />
      <Separator label="Your Library" />
      <NavItem to="/todo" label="Your Favorites" />
      <NavItem to="/todo" label="Your Playlists" />
    </ul>
  </aside>
</template>
