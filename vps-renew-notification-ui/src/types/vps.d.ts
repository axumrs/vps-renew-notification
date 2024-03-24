type VPS = {
  id: string;
  provider_id: string;
  name: string;
  expire: Date;
  dateline: Date;
};

type VPSWithProvider =
  | VPS
  | {
      provider_name: string;
    };
