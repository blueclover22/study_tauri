export interface ApiResponse<T> {
  status: {
    code: string;
    message?: string;
  };
  info: T;
}

export interface LoginRequest {
  userId: string;
  userPwd: string;
}

export interface LoginResponse {
  insEmpNo?: string;
  insDate?: string;
  updEmpNo?: string;
  updDate?: string;
  procEmpNo?: string;
  procDate?: string;
  userId?: string;        // POS 사용자 ID
  userNm?: string;        // POS 사용자 명
  facilityCd?: string;    // 사업장 코드
  posVersion?: string;    // 포스 버전
  lastLoginDate?: string; // 마지막 로그인 일시
  startDt?: string;
  endDt?: string;
  facilityNm?: string;    // 사업장 명
  posNo?: string;
  posNm?: string;
  posGb?: string;
  posKind?: string;
  uuid?: string;
  location?: string;
  posIp?: string;
  autoCheckYn?: string;
  capTerminalNo?: string;
  wechatTerminalNo?: string;
  aliTerminalNo?: string;
  lastSaleDt?: string;
  useYn?: string;
  saleDt?: string;
  subSeq?: string;
  rdyAmt: number;         // POS 시재금
  key?: string;
  tokenValue?: string;    // token 값
  loginType?: string;
  loginDeviceId?: string;
  addr1?: string;
  addr2?: string;
  ownerNm?: string;
  saupjaNo?: string;
  telNo?: string;
  telNumber?: string;
  saleViewType?: string;  // 판매상태 타입
}
