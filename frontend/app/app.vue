<template>
  <div
    class="bg-[#2D2A33] w-screen h-screen overflow-scroll overflow-y-auto flex flex-row gap-0 font-sans font-[_decide_later_]"
  >
    <Sidebar />
    <main class="flex flex-col w-full h-full">
      <div class="flex-1 overflow-y-auto">
        <NuxtPage />
      </div>
      <Player />
    </main>
  </div>
</template>
