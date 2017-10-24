#ifndef QT_GUI_C_QSCROLLPREPAREEVENT_H
#define QT_GUI_C_QSCROLLPREPAREEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(QScrollPrepareEvent* ptr);
QT_GUI_C_EXPORT QScrollPrepareEvent* qt_gui_c_QScrollPrepareEvent_G_static_cast_QScrollPrepareEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_contentPosRange_to_output(const QScrollPrepareEvent* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_contentPos_to_output(const QScrollPrepareEvent* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_delete(QScrollPrepareEvent* this_ptr);
QT_GUI_C_EXPORT QScrollPrepareEvent* qt_gui_c_QScrollPrepareEvent_new(const QPointF* startPos);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_setContentPos(QScrollPrepareEvent* this_ptr, const QPointF* pos);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_setContentPosRange(QScrollPrepareEvent* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_setViewportSize(QScrollPrepareEvent* this_ptr, const QSizeF* size);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_startPos_to_output(const QScrollPrepareEvent* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QScrollPrepareEvent_viewportSize_to_output(const QScrollPrepareEvent* this_ptr, QSizeF* output);

} // extern "C"

#endif // QT_GUI_C_QSCROLLPREPAREEVENT_H
