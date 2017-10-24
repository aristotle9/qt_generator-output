#ifndef QT_GUI_C_QRADIALGRADIENT_H
#define QT_GUI_C_QRADIALGRADIENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QGradient* qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(QRadialGradient* ptr);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_G_static_cast_QRadialGradient_ptr(QGradient* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRadialGradient_centerRadius(const QRadialGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_center_to_output(const QRadialGradient* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_delete(QRadialGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_focalPoint_to_output(const QRadialGradient* this_ptr, QPointF* output);
QT_GUI_C_EXPORT double qt_gui_c_QRadialGradient_focalRadius(const QRadialGradient* this_ptr);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_center_centerRadius_focalPoint_focalRadius(const QPointF* center, double centerRadius, const QPointF* focalPoint, double focalRadius);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_center_radius(const QPointF* center, double radius);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_center_radius_focalPoint(const QPointF* center, double radius, const QPointF* focalPoint);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_centerRadius_fx_fy_focalRadius(double cx, double cy, double centerRadius, double fx, double fy, double focalRadius);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_radius(double cx, double cy, double radius);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_radius_fx_fy(double cx, double cy, double radius, double fx, double fy);
QT_GUI_C_EXPORT QRadialGradient* qt_gui_c_QRadialGradient_new_no_args();
QT_GUI_C_EXPORT double qt_gui_c_QRadialGradient_radius(const QRadialGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setCenterRadius(QRadialGradient* this_ptr, double radius);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setCenter_center(QRadialGradient* this_ptr, const QPointF* center);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setCenter_x_y(QRadialGradient* this_ptr, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setFocalPoint_focalPoint(QRadialGradient* this_ptr, const QPointF* focalPoint);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setFocalPoint_x_y(QRadialGradient* this_ptr, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setFocalRadius(QRadialGradient* this_ptr, double radius);
QT_GUI_C_EXPORT void qt_gui_c_QRadialGradient_setRadius(QRadialGradient* this_ptr, double radius);

} // extern "C"

#endif // QT_GUI_C_QRADIALGRADIENT_H
