<template>
  <svg aria-hidden="true">
    <use :href="symbolId" :style="{ color: color }"></use>
  </svg>
</template>

<script setup lang="ts">

import {computed} from "vue";

const props = defineProps({
  prefix: {
    type: String,
    default: "icon"
  },
  name: String,
  color: String,
})

const symbolId = computed(() => `#${props.prefix}-${props.name}`)

</script>

<style scoped>

</style>