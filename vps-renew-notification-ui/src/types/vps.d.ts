type VPS = {
  id: string;
  provider_id: string;
  name: string;
  expire: string;
  dateline: string;
};

type VPSWithProvider = {
  id: string;
  provider_id: string;
  name: string;
  expire: string;
  dateline: string;
  provider_name: string;
  renew_days: number;
  notify_days: number;
};
