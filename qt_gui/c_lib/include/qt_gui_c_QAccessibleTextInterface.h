#ifndef QT_GUI_C_QACCESSIBLETEXTINTERFACE_H
#define QT_GUI_C_QACCESSIBLETEXTINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_addSelection(QAccessibleTextInterface* this_ptr, int startOffset, int endOffset);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_attributes_to_output(const QAccessibleTextInterface* this_ptr, int offset, int* startOffset, int* endOffset, QString* output);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTextInterface_characterCount(const QAccessibleTextInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_characterRect_to_output(const QAccessibleTextInterface* this_ptr, int offset, QRect* output);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTextInterface_cursorPosition(const QAccessibleTextInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_delete(QAccessibleTextInterface* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTextInterface_offsetAtPoint(const QAccessibleTextInterface* this_ptr, const QPoint* point);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_removeSelection(QAccessibleTextInterface* this_ptr, int selectionIndex);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_scrollToSubstring(QAccessibleTextInterface* this_ptr, int startIndex, int endIndex);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_selection(const QAccessibleTextInterface* this_ptr, int selectionIndex, int* startOffset, int* endOffset);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTextInterface_selectionCount(const QAccessibleTextInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_setCursorPosition(QAccessibleTextInterface* this_ptr, int position);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_setSelection(QAccessibleTextInterface* this_ptr, int selectionIndex, int startOffset, int endOffset);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_textAfterOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_textAtOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_textBeforeOffset_to_output(const QAccessibleTextInterface* this_ptr, int offset, const QAccessible::TextBoundaryType* boundaryType, int* startOffset, int* endOffset, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextInterface_text_to_output(const QAccessibleTextInterface* this_ptr, int startOffset, int endOffset, QString* output);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLETEXTINTERFACE_H
