import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useStatusStore = defineStore("status", () => {
  const _isLoading = ref(false);
  const _msg = ref("");

  const isLoading = computed(() => _isLoading);
  const msg = computed(() => _msg);

  const setLoading = (v: boolean) => {
    _isLoading.value = v;
  };

  const setMsg = (m: string) => {
    _msg.value = m;
  };

  return { isLoading, setLoading, msg, setMsg };
});
