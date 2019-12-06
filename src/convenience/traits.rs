/*
  create at 2019/12/4 by 'itachy'
*/

pub trait RequestStatus {
    fn is_req_succeed(&self) -> bool;
}