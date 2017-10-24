#ifndef QT_GUI_C_QTOOLBARCHANGEEVENT_H
#define QT_GUI_C_QTOOLBARCHANGEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(QToolBarChangeEvent* ptr);
QT_GUI_C_EXPORT QToolBarChangeEvent* qt_gui_c_QToolBarChangeEvent_G_static_cast_QToolBarChangeEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QToolBarChangeEvent_delete(QToolBarChangeEvent* this_ptr);
QT_GUI_C_EXPORT QToolBarChangeEvent* qt_gui_c_QToolBarChangeEvent_new(bool t);
QT_GUI_C_EXPORT bool qt_gui_c_QToolBarChangeEvent_toggle(const QToolBarChangeEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTOOLBARCHANGEEVENT_H
