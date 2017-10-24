#include "qt_gui_c_QAbstractUndoItem.h"

void qt_gui_c_QAbstractUndoItem_delete(QAbstractUndoItem* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QAbstractUndoItem_redo(QAbstractUndoItem* this_ptr) {
  this_ptr->redo();
}

void qt_gui_c_QAbstractUndoItem_undo(QAbstractUndoItem* this_ptr) {
  this_ptr->undo();
}

