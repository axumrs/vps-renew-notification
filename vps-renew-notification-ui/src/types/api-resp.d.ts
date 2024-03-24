type ApiResponse<T> = {
  code: number;
  msg: string;
  data?: T | null;
};

type LoginResponse = {
  auth: Auth;
  data: UserClaimsData;
};
