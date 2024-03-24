type UrlParameter = { [k: string]: any };

class ApiError extends Error {}

import { useStatusStore } from "@/store/status";
import { useAuthStore } from "@/store/auth";

export default function useFetch() {
  const { setMsg, setLoading } = useStatusStore();
  const { getToken } = useAuthStore();
  const _fetch = ({
    url,
    method,
    body,
    params,
  }: {
    url: string;
    method: string;
    body?: any;
    params?: UrlParameter;
  }) => {
    let urlParams = "";
    if (params) {
      let urlParamsArray = [];
      for (let k in params) {
        urlParamsArray.push(`${k}=${params[k]}`);
      }
      urlParams = `?${urlParamsArray.join("&")}`;
    }

    setLoading(true);

    return fetch(`${import.meta.env.VITE_API_URL}${url}${urlParams}`, {
      headers: {
        "Content-Type": "application/json;charset=utf-8",
        Authorization: `Bearer ${getToken()}`,
      },
      method,
      body: JSON.stringify(body),
    })
      .then((resp) => resp.json())
      .then((resp: ApiResponse<any>) => {
        if (resp.code !== 0) {
          //   setMsg(resp.msg);
          return Promise.reject(new ApiError(resp.msg));
        }
        return Promise.resolve(resp.data);
      })
      .catch((e) => {
        let msg = "请检查网络";
        if (e instanceof ApiError) {
          msg = e.message;
        }
        setMsg(msg);
        console.log(e);
      })
      .finally(() => {
        setLoading(false);
      });
  };

  const get = (url: string, params?: UrlParameter) => {
    return _fetch({ url, params, method: "GET" });
  };

  const post = (url: string, body?: any, params?: UrlParameter) => {
    return _fetch({ url, body, params, method: "POST" });
  };

  const put = (url: string, body?: any, params?: UrlParameter) => {
    return _fetch({ url, body, params, method: "PUT" });
  };

  const patch = (url: string, body?: any, params?: UrlParameter) => {
    return _fetch({ url, body, params, method: "PATCH" });
  };

  const del = (url: string, params?: UrlParameter) => {
    return _fetch({ url, params, method: "DELETE" });
  };

  return { get, post, put, patch, del };
}
