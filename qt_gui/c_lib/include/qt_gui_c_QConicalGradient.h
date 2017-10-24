#ifndef QT_GUI_C_QCONICALGRADIENT_H
#define QT_GUI_C_QCONICALGRADIENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QConicalGradient* qt_gui_c_QConicalGradient_G_static_cast_QConicalGradient_ptr(QGradient* ptr);
QT_GUI_C_EXPORT QGradient* qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(QConicalGradient* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QConicalGradient_angle(const QConicalGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QConicalGradient_center_to_output(const QConicalGradient* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QConicalGradient_delete(QConicalGradient* this_ptr);
QT_GUI_C_EXPORT QConicalGradient* qt_gui_c_QConicalGradient_new_center_startAngle(const QPointF* center, double startAngle);
QT_GUI_C_EXPORT QConicalGradient* qt_gui_c_QConicalGradient_new_cx_cy_startAngle(double cx, double cy, double startAngle);
QT_GUI_C_EXPORT QConicalGradient* qt_gui_c_QConicalGradient_new_no_args();
QT_GUI_C_EXPORT void qt_gui_c_QConicalGradient_setAngle(QConicalGradient* this_ptr, double angle);
QT_GUI_C_EXPORT void qt_gui_c_QConicalGradient_setCenter_center(QConicalGradient* this_ptr, const QPointF* center);
QT_GUI_C_EXPORT void qt_gui_c_QConicalGradient_setCenter_x_y(QConicalGradient* this_ptr, double x, double y);

} // extern "C"

#endif // QT_GUI_C_QCONICALGRADIENT_H
