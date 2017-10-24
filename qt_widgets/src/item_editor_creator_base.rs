/// C++ type: <span style='color: green;'>```QItemEditorCreatorBase```</span>
#[repr(C)]
pub struct ItemEditorCreatorBase(u8);

impl ItemEditorCreatorBase {
  /// C++ method: <span style='color: green;'>```pure virtual QWidget* QItemEditorCreatorBase::createWidget(QWidget* parent) const```</span>
  ///
  ///
  pub unsafe fn create_widget(&self, parent: *mut ::widget::Widget) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QItemEditorCreatorBase_createWidget(self as *const ::item_editor_creator_base::ItemEditorCreatorBase, parent)
  }

  /// C++ method: <span style='color: green;'>```pure virtual QByteArray QItemEditorCreatorBase::valuePropertyName() const```</span>
  ///
  ///
  pub fn value_property_name(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QItemEditorCreatorBase_valuePropertyName_to_output(self as *const ::item_editor_creator_base::ItemEditorCreatorBase, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::item_editor_creator_base::ItemEditorCreatorBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QItemEditorCreatorBase_delete
  }
}
