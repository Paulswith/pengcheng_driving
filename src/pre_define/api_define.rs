/*
  create at 2019/12/4 by 'itachy'
*/

/// 每次操作都需要一个最新的signature 从这接口来的.
pub const GET_API_CODE: &str = "http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetApiCode";
// EXAMPLE: GET http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetApiCode

/// 登录接口
pub const GET_LOGIN: &str = "http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetLogin";
// EXAMPLE: GET http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetLogin?AppPassword=XXXXXXXXXX&AppPhoneNumber=111111111111&nonce=1000000532&signature=cccccccccc8335d26d4efd468da5cfdac005f&timestamp=1259840392

pub const GET_TEACH_INFO: &str = "http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetStudentBindTeachInfo";
// EXAMPLE: GET http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetStudentBindTeachInfo?AppRecordID=EQESXZCSADSACXZD81C22C5D10230D&nonce=1000000532&signature=cccccccccc8335d26d4efd468da5cfdac005f&timestamp=1259840392

/// 查询可预约排期: phase 1 2 3 对应上午 下午 晚上
pub const GET_RESERVED_TIME_LIST: &str = "http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetReservedTimeList";
// EXAMPLE: GET http://appwebsrv01.22168168.com/PCMIS_App.asmx/GetReservedTimeList?AppRecordID=EQESXZCSADSACXZD81C22C5D10230D&Phase=1&ReservedDate=2019-12-01&TeachID=pc100&nonce=1000000532&signature=cccccccccc8335d26d4efd468da5cfdac005f&timestamp=1259840392

/// 提交预约申请
pub const ADD_CAR_ORDER_INFO_HOUS_IOS: &str = "http://appwebsrv01.22168168.com/PCMIS_App.asmx/AddCarOrderInfoHous_Ios";
// EXAMPLE: GET http://appwebsrv01.22168168.com/PCMIS_App.asmx/AddCarOrderInfoHous_Ios?AppRecordID=EQESXZCSADSACXZD81C22C5D10230D&ReservedID=SDSADASDASZXCASDASASDZ123&nonce=1000000532&signature=cccccccccc8335d26d4efd468da5cfdac005f&timestamp=1259840392
