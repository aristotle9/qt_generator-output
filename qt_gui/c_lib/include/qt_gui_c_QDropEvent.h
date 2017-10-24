#ifndef QT_GUI_C_QDROPEVENT_H
#define QT_GUI_C_QDROPEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDropEvent* qt_gui_c_QDropEvent_G_static_cast_QDropEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(QDropEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDropEvent_acceptProposedAction(QDropEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDropEvent_delete(QDropEvent* this_ptr);
QT_GUI_C_EXPORT const QMimeData* qt_gui_c_QDropEvent_mimeData(const QDropEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QDropEvent_posF(const QDropEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDropEvent_pos_to_output(const QDropEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QDropEvent_setDropAction(QDropEvent* this_ptr, const Qt::DropAction* action);
QT_GUI_C_EXPORT QObject* qt_gui_c_QDropEvent_source(const QDropEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QDROPEVENT_H
