#ifndef QT_GUI_C_QLINEARGRADIENT_H
#define QT_GUI_C_QLINEARGRADIENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QGradient* qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(QLinearGradient* ptr);
QT_GUI_C_EXPORT QLinearGradient* qt_gui_c_QLinearGradient_G_static_cast_QLinearGradient_ptr(QGradient* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_delete(QLinearGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_finalStop_to_output(const QLinearGradient* this_ptr, QPointF* output);
QT_GUI_C_EXPORT QLinearGradient* qt_gui_c_QLinearGradient_new_no_args();
QT_GUI_C_EXPORT QLinearGradient* qt_gui_c_QLinearGradient_new_start_finalStop(const QPointF* start, const QPointF* finalStop);
QT_GUI_C_EXPORT QLinearGradient* qt_gui_c_QLinearGradient_new_xStart_yStart_xFinalStop_yFinalStop(double xStart, double yStart, double xFinalStop, double yFinalStop);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_setFinalStop_stop(QLinearGradient* this_ptr, const QPointF* stop);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_setFinalStop_x_y(QLinearGradient* this_ptr, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_setStart_start(QLinearGradient* this_ptr, const QPointF* start);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_setStart_x_y(QLinearGradient* this_ptr, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QLinearGradient_start_to_output(const QLinearGradient* this_ptr, QPointF* output);

} // extern "C"

#endif // QT_GUI_C_QLINEARGRADIENT_H
