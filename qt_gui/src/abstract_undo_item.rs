/// C++ type: <span style='color: green;'>```QAbstractUndoItem```</span>
#[repr(C)]
pub struct AbstractUndoItem(u8);

impl AbstractUndoItem {
  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractUndoItem::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_gui_c_QAbstractUndoItem_redo(self as *mut ::abstract_undo_item::AbstractUndoItem) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractUndoItem::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_gui_c_QAbstractUndoItem_undo(self as *mut ::abstract_undo_item::AbstractUndoItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_undo_item::AbstractUndoItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAbstractUndoItem_delete
  }
}
