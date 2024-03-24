import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useStatusStore = defineStore("status", () => {
  const _isLoading = ref(false);
  const _msg = ref("");
  const _ok_msg = ref("");

  const isLoading = computed(() => _isLoading);
  const msg = computed(() => _msg);
  const okMsg = computed(() => _ok_msg);

  const setLoading = (v: boolean) => {
    _isLoading.value = v;
  };

  const setMsg = (m: string) => {
    _msg.value = m;
  };

  const setOkMsg = (m: string) => {
    _ok_msg.value = m;
  };

  return { isLoading, setLoading, msg, setMsg, okMsg, setOkMsg };
});
