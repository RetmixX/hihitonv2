<template>
  <div class="container">
    <div class="wrapper">
      <AppHeader />
      <slot></slot>
      <AppFooter />
    </div>
  </div>
</template>

<script>
import AppHeader from "../layouts/AppHeader.vue";
import AppFooter from "../layouts/AppFooter.vue";
export default {
  name: "DefaultLayout",
  components: {
    AppHeader,
    AppFooter,
  },
};
</script>

<style  scoped>
.wrapper {
  width: 80%;
}
.container {
  display: flex;
  justify-content: center;
  width: 100%;
  align-items: center;
}
</style>
