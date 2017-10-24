/// C++ type: <span style='color: green;'>```QTextList```</span>
#[repr(C)]
pub struct TextList(u8);

impl TextList {
  /// C++ method: <span style='color: green;'>```void QTextList::add(const QTextBlock& block)```</span>
  ///
  ///
  pub fn add(&mut self, block: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QTextList_add(self as *mut ::text_list::TextList,
                                    block as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextList::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextList_count(self as *const ::text_list::TextList) }
  }

  /// C++ method: <span style='color: green;'>```QTextListFormat QTextList::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::text_list_format::TextListFormat {
    {
      let mut object: ::text_list_format::TextListFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextList_format_to_output(self as *const ::text_list::TextList, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextList::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextList_isEmpty(self as *const ::text_list::TextList) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextList::item(int i) const```</span>
  ///
  ///
  pub fn item(&self, i: ::libc::c_int) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextList_item_to_output(self as *const ::text_list::TextList, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextList::itemNumber(const QTextBlock& arg1) const```</span>
  ///
  ///
  pub fn item_number(&self, arg1: &::text_block::TextBlock) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QTextList_itemNumber(self as *const ::text_list::TextList,
                                           arg1 as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextList::itemText(const QTextBlock& arg1) const```</span>
  ///
  ///
  pub fn item_text(&self, arg1: &::text_block::TextBlock) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextList_itemText_to_output(self as *const ::text_list::TextList,
                                                     arg1 as *const ::text_block::TextBlock,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextList::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextList_metaObject(self as *const ::text_list::TextList) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextList::QTextList(QTextDocument* doc)```</span>
  ///
  ///
  pub unsafe fn new(doc: *mut ::text_document::TextDocument) -> ::cpp_utils::CppBox<::text_list::TextList> {
    let ffi_result = ::ffi::qt_gui_c_QTextList_new(doc);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextList::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextList_qt_metacall(self as *mut ::text_list::TextList,
                                          arg1 as *const ::qt_core::meta_object::Call,
                                          arg2,
                                          arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextList::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextList_qt_metacast(self as *mut ::text_list::TextList, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTextList::remove(const QTextBlock& arg1)```</span>
  ///
  ///
  pub fn remove(&mut self, arg1: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QTextList_remove(self as *mut ::text_list::TextList,
                                       arg1 as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextList::removeItem(int i)```</span>
  ///
  ///
  pub fn remove_item(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextList_removeItem(self as *mut ::text_list::TextList, i) }
  }

  /// C++ method: <span style='color: green;'>```void QTextList::setFormat(const QTextListFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::text_list_format::TextListFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextList_setFormat(self as *mut ::text_list::TextList,
                                          format as *const ::text_list_format::TextListFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextList::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextList_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextList::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextList_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_list::TextList {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextList_delete
  }
}

impl ::cpp_utils::DynamicCast<::text_list::TextList> for ::text_block_group::TextBlockGroup {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_list::TextList> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextBlockGroup(self as *mut ::text_block_group::TextBlockGroup) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_list::TextList> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextBlockGroup(self as *const ::text_block_group::TextBlockGroup as *mut ::text_block_group::TextBlockGroup) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_list::TextList> for ::text_object::TextObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_list::TextList> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextObject(self as *mut ::text_object::TextObject)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_list::TextList> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_list::TextList {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QObject_ptr(self as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QObject_ptr(self as *const ::text_list::TextList as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_block_group::TextBlockGroup> for ::text_list::TextList {
  fn static_cast_mut(&mut self) -> &mut ::text_block_group::TextBlockGroup {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(self as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_block_group::TextBlockGroup {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(self as *const ::text_list::TextList as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_object::TextObject> for ::text_list::TextList {
  fn static_cast_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextObject_ptr(self as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextObject_ptr(self as *const ::text_list::TextList as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_list::TextList> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_list::TextList {
    let ffi_result =
      ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_list::TextList {
    let ffi_result = ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_list::TextList> for ::text_block_group::TextBlockGroup {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_list::TextList {
    let ffi_result = ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextBlockGroup(self as *mut ::text_block_group::TextBlockGroup);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_list::TextList {
    let ffi_result = ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextBlockGroup(self as *const ::text_block_group::TextBlockGroup as *mut ::text_block_group::TextBlockGroup);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_list::TextList> for ::text_object::TextObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_list::TextList {
    let ffi_result =
      ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextObject(self as *mut ::text_object::TextObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_list::TextList {
    let ffi_result = ::ffi::qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_list::TextList {
  type Target = ::text_block_group::TextBlockGroup;
  fn deref(&self) -> &::text_block_group::TextBlockGroup {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(self as *const ::text_list::TextList as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_list::TextList {
  fn deref_mut(&mut self) -> &mut ::text_block_group::TextBlockGroup {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(self as *mut ::text_list::TextList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
