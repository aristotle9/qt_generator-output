#ifndef QT_GUI_C_QSCREEN_H
#define QT_GUI_C_QSCREEN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QScreen_G_operator_shl_to_output(const QDebug* arg1, const QScreen* arg2, QDebug* output);
QT_GUI_C_EXPORT QObject* qt_gui_c_QScreen_G_static_cast_QObject_ptr(QScreen* ptr);
QT_GUI_C_EXPORT QScreen* qt_gui_c_QScreen_G_static_cast_QScreen_ptr(QObject* ptr);
QT_GUI_C_EXPORT int qt_gui_c_QScreen_angleBetween(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_availableGeometry_to_output(const QScreen* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_availableSize_to_output(const QScreen* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_availableVirtualGeometry_to_output(const QScreen* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_availableVirtualSize_to_output(const QScreen* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_delete(QScreen* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QScreen_depth(const QScreen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_devicePixelRatio(const QScreen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_geometry_to_output(const QScreen* this_ptr, QRect* output);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window(QScreen* this_ptr, unsigned long long window);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x(QScreen* this_ptr, unsigned long long window, int x);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y(QScreen* this_ptr, unsigned long long window, int x, int y);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w(QScreen* this_ptr, unsigned long long window, int x, int y, int w);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w_h(QScreen* this_ptr, unsigned long long window, int x, int y, int w, int h);
QT_GUI_C_EXPORT bool qt_gui_c_QScreen_isLandscape(const QScreen* this_ptr, const Qt::ScreenOrientation* orientation);
QT_GUI_C_EXPORT bool qt_gui_c_QScreen_isPortrait(const QScreen* this_ptr, const Qt::ScreenOrientation* orientation);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_logicalDotsPerInch(const QScreen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_logicalDotsPerInchX(const QScreen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_logicalDotsPerInchY(const QScreen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_manufacturer_to_output(const QScreen* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_mapBetween_to_output(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b, const QRect* rect, QRect* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QScreen_metaObject(const QScreen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_model_to_output(const QScreen* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_name_to_output(const QScreen* this_ptr, QString* output);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_physicalDotsPerInch(const QScreen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_physicalDotsPerInchX(const QScreen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_physicalDotsPerInchY(const QScreen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_physicalSize_to_output(const QScreen* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT int qt_gui_c_QScreen_qt_metacall(QScreen* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QScreen_qt_metacast(QScreen* this_ptr, const char* arg1);
QT_GUI_C_EXPORT double qt_gui_c_QScreen_refreshRate(const QScreen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_serialNumber_to_output(const QScreen* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_size_to_output(const QScreen* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_transformBetween_to_output(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b, const QRect* target, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_virtualGeometry_to_output(const QScreen* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_virtualSiblings_to_output(const QScreen* this_ptr, QList< QScreen* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QScreen_virtualSize_to_output(const QScreen* this_ptr, QSize* output);

} // extern "C"

#endif // QT_GUI_C_QSCREEN_H
