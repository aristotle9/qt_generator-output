#include "qt_gui_c_QAccessibleTextInterface.h"

void qt_gui_c_QAccessibleTextInterface_addSelection(QAccessibleTextInterface* this_ptr, int startOffset, int endOffset) {
  this_ptr->addSelection(startOffset, endOffset);
}

void qt_gui_c_QAccessibleTextInterface_attributes_to_output(const QAccessibleTextInterface* this_ptr, int offset, int* startOffset, int* endOffset, QString* output) {
  new(output) QString(this_ptr->attributes(offset, startOffset, endOffset));
}

int qt_gui_c_QAccessibleTextInterface_characterCount(const QAccessibleTextInterface* this_ptr) {
  return this_ptr->characterCount();
}

void qt_gui_c_QAccessibleTextInterface_characterRect_to_output(const QAccessibleTextInterface* this_ptr, int offset, QRect* output) {
  new(output) QRect(this_ptr->characterRect(offset));
}

int qt_gui_c_QAccessibleTextInterface_cursorPosition(const QAccessibleTextInterface* this_ptr) {
  return this_ptr->cursorPosition();
}

void qt_gui_c_QAccessibleTextInterface_delete(QAccessibleTextInterface* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QAccessibleTextInterface_offsetAtPoint(const QAccessibleTextInterface* this_ptr, const QPoint* point) {
  return this_ptr->offsetAtPoint(*point);
}

void qt_gui_c_QAccessibleTextInterface_removeSelection(QAccessibleTextInterface* this_ptr, int selectionIndex) {
  this_ptr->removeSelection(selectionIndex);
}

void qt_gui_c_QAccessibleTextInterface_scrollToSubstring(QAccessibleTextInterface* this_ptr, int startIndex, int endIndex) {
  this_ptr->scrollToSubstring(startIndex, endIndex);
}

void qt_gui_c_QAccessibleTextInterface_selection(const QAccessibleTextInterface* this_ptr, int selectionIndex, int* startOffset, int* endOffset) {
  this_ptr->selection(selectionIndex, startOffset, endOffset);
}

int qt_gui_c_QAccessibleTextInterface_selectionCount(const QAccessibleTextInterface* this_ptr) {
  return this_ptr->selectionCount();
}

void qt_gui_c_QAccessibleTextInterface_setCursorPosition(QAccessibleTextInterface* this_ptr, int position) {
  this_ptr->setCursorPosition(position);
}

void qt_gui_c_QAccessibleTextInterface_setSelection(QAccessibleTextInterface* this_ptr, int selectionIndex, int startOffset, int endOffset) {
  this_ptr->setSelection(selectionIndex, startOffset, endOffset);
}

void qt_gui_c_QAccessibleTextInterface_textAfterOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output) {
  new(output) QString(this_ptr->textAfterOffset(offset, *boundaryType, startOffset, endOffset));
}

void qt_gui_c_QAccessibleTextInterface_textAtOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output) {
  new(output) QString(this_ptr->textAtOffset(offset, *boundaryType, startOffset, endOffset));
}

void qt_gui_c_QAccessibleTextInterface_textBeforeOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output) {
  new(output) QString(this_ptr->textBeforeOffset(offset, *boundaryType, startOffset, endOffset));
}

void qt_gui_c_QAccessibleTextInterface_text_to_output(const QAccessibleTextInterface* this_ptr, int startOffset, int endOffset, QString* output) {
  new(output) QString(this_ptr->text(startOffset, endOffset));
}

