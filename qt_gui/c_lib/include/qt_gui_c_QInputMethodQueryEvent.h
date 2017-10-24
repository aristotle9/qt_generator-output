#ifndef QT_GUI_C_QINPUTMETHODQUERYEVENT_H
#define QT_GUI_C_QINPUTMETHODQUERYEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(QInputMethodQueryEvent* ptr);
QT_GUI_C_EXPORT QInputMethodQueryEvent* qt_gui_c_QInputMethodQueryEvent_G_static_cast_QInputMethodQueryEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodQueryEvent_delete(QInputMethodQueryEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodQueryEvent_setValue(QInputMethodQueryEvent* this_ptr, const Qt::InputMethodQuery* query, const QVariant* value);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodQueryEvent_value_to_output(const QInputMethodQueryEvent* this_ptr, const Qt::InputMethodQuery* query, QVariant* output);

} // extern "C"

#endif // QT_GUI_C_QINPUTMETHODQUERYEVENT_H
