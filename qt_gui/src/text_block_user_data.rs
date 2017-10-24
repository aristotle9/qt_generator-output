/// C++ type: <span style='color: green;'>```QTextBlockUserData```</span>
#[repr(C)]
pub struct TextBlockUserData(u8);

impl ::cpp_utils::CppDeletable for ::text_block_user_data::TextBlockUserData {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextBlockUserData_delete
  }
}
