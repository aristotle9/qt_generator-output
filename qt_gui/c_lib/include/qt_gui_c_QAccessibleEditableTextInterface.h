#ifndef QT_GUI_C_QACCESSIBLEEDITABLETEXTINTERFACE_H
#define QT_GUI_C_QACCESSIBLEEDITABLETEXTINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEditableTextInterface_delete(QAccessibleEditableTextInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEditableTextInterface_deleteText(QAccessibleEditableTextInterface* this_ptr, int startOffset, int endOffset);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEditableTextInterface_insertText(QAccessibleEditableTextInterface* this_ptr, int offset, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEditableTextInterface_replaceText(QAccessibleEditableTextInterface* this_ptr, int startOffset, int endOffset, const QString* text);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEEDITABLETEXTINTERFACE_H
