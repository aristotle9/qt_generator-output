/// C++ type: <span style='color: green;'>```QItemEditorFactory```</span>
#[repr(C)]
pub struct ItemEditorFactory(u8);

impl ItemEditorFactory {
  /// C++ method: <span style='color: green;'>```virtual QWidget* QItemEditorFactory::createEditor(int userType, QWidget* parent) const```</span>
  ///
  ///
  pub unsafe fn create_editor(&self, user_type: ::libc::c_int, parent: *mut ::widget::Widget) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QItemEditorFactory_createEditor(self as *const ::item_editor_factory::ItemEditorFactory,
                                                        user_type,
                                                        parent)
  }

  /// C++ method: <span style='color: green;'>```static const QItemEditorFactory* QItemEditorFactory::defaultFactory()```</span>
  ///
  ///
  pub fn default_factory() -> *const ::item_editor_factory::ItemEditorFactory {
    unsafe { ::ffi::qt_widgets_c_QItemEditorFactory_defaultFactory() }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QItemEditorFactory::QItemEditorFactory()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::item_editor_factory::ItemEditorFactory> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemEditorFactory_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase* creator)```</span>
  ///
  ///
  pub unsafe fn register_editor(&mut self,
                                user_type: ::libc::c_int,
                                creator: *mut ::item_editor_creator_base::ItemEditorCreatorBase) {
    ::ffi::qt_widgets_c_QItemEditorFactory_registerEditor(self as *mut ::item_editor_factory::ItemEditorFactory,
                                                          user_type,
                                                          creator)
  }

  /// C++ method: <span style='color: green;'>```static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory* factory)```</span>
  ///
  ///
  pub unsafe fn set_default_factory(factory: *mut ::item_editor_factory::ItemEditorFactory) {
    ::ffi::qt_widgets_c_QItemEditorFactory_setDefaultFactory(factory)
  }

  /// C++ method: <span style='color: green;'>```virtual QByteArray QItemEditorFactory::valuePropertyName(int userType) const```</span>
  ///
  ///
  pub fn value_property_name(&self, user_type: ::libc::c_int) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QItemEditorFactory_valuePropertyName_to_output(self as *const ::item_editor_factory::ItemEditorFactory, user_type, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::item_editor_factory::ItemEditorFactory {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QItemEditorFactory_delete
  }
}
