import { defineStore } from "pinia";
import { computed, reactive } from "vue";

type LoginRespStoreData = LoginResponse;

export const useAuthStore = defineStore("auth", () => {
  const _loginResp = reactive<LoginRespStoreData>({
    auth: { token: "", token_type: "" },
    data: { id: "", username: "" },
  });

  const loginResp = computed(() => {
    return _loginResp;
  });

  const getToken = () => {
    return _loginResp.auth.token;
  };

  const getUser = () => {
    return _loginResp.data;
  };

  const setLoginResp = (data: LoginRespStoreData) => {
    Object.assign(_loginResp, data);
  };

  const isLogined = () => {
    return _loginResp &&
      _loginResp.auth &&
      _loginResp.data &&
      _loginResp.auth.token &&
      _loginResp.data.id
      ? true
      : false;
  };

  return { loginResp, setLoginResp, isLogined, getToken, getUser };
});
