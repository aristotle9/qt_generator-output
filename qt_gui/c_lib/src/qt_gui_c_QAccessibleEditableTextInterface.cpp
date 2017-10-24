#include "qt_gui_c_QAccessibleEditableTextInterface.h"

void qt_gui_c_QAccessibleEditableTextInterface_delete(QAccessibleEditableTextInterface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QAccessibleEditableTextInterface_deleteText(QAccessibleEditableTextInterface* this_ptr, int startOffset, int endOffset) {
  this_ptr->deleteText(startOffset, endOffset);
}

void qt_gui_c_QAccessibleEditableTextInterface_insertText(QAccessibleEditableTextInterface* this_ptr, int offset, const QString* text) {
  this_ptr->insertText(offset, *text);
}

void qt_gui_c_QAccessibleEditableTextInterface_replaceText(QAccessibleEditableTextInterface* this_ptr, int startOffset, int endOffset, const QString* text) {
  this_ptr->replaceText(startOffset, endOffset, *text);
}

