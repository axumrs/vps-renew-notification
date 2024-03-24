type VPS = {
  id: string;
  provider_id: string;
  name: string;
  expire: string;
  dateline: string;
};

type VPSWithProvider =
  | VPS
  | {
      provider_name: string;
    };
