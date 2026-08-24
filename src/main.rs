use std::ptr::null_mut;
use windows::Win32::Foundation::{LPARAM,LRESULT,WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx,GetMessageW,SetWindowsHookExW,HHOOK,KBDLLHOOKSTRUCT,MSG,WH_KEYBOARD_LL,WM_KEYDOWN,WM_SYSKEYDOWN};

const LLKHF_INJECTED         : u32 = 0x00000010;
const LLKHF_LOWER_IL_INJECTED: u32 = 0x00000002;

unsafe extern "system" fn low_level_keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
 if code >= 0 {
  let event_id = w_param.0 as u32;

  if event_id == WM_KEYDOWN || event_id == WM_SYSKEYDOWN {
   println!("l_param: {:?}", l_param);
   println!("w_param: {:?}", w_param);

   let hook_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);

   let is_injected = (hook_struct.flags.0 & LLKHF_INJECTED) != 0;
   let is_lower_il_injected = (hook_struct.flags.0 & LLKHF_LOWER_IL_INJECTED) != 0;

   if is_injected || is_lower_il_injected {
    println!("Simulated");

   } else {//if is_injected || is_lower_il_injected {
    println!("Physical");

   }//} else {//if is_injected || is_lower_il_injected {
  }//if event_id == WM_KEYDOWN || event_id == WM_SYSKEYDOWN {
 }//if code >= 0 {

 CallNextHookEx(Some(HHOOK::default()),code,w_param,l_param)
}//unsafe extern "system" fn low_level_keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

fn main() {
 unsafe {
  let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), None, 0).expect("Failed");

  let mut msg = MSG::default();

  while GetMessageW(&mut msg, None, 0, 0).into() {

  }//while GetMessageW(&mut msg, None, 0, 0).into() {
 }//unsafe {
}//fn main() {
