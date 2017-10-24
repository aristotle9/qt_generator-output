#ifndef QT_WIDGETS_C_QRUBBERBAND_H
#define QT_WIDGETS_C_QRUBBERBAND_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_G_dynamic_cast_QRubberBand_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QRubberBand_G_static_cast_QObject_ptr(QRubberBand* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QRubberBand_G_static_cast_QPaintDevice_ptr(QRubberBand* ptr);
QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QRubberBand_G_static_cast_QWidget_ptr(QRubberBand* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_delete(QRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QRubberBand_metaObject(const QRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_move_p(QRubberBand* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_move_x_y(QRubberBand* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_new_arg1(QRubberBand::Shape arg1);
QT_WIDGETS_C_EXPORT QRubberBand* qt_widgets_c_QRubberBand_new_arg1_arg2(QRubberBand::Shape arg1, QWidget* arg2);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QRubberBand_qt_metacall(QRubberBand* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QRubberBand_qt_metacast(QRubberBand* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_resize_s(QRubberBand* this_ptr, const QSize* s);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_resize_w_h(QRubberBand* this_ptr, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_setGeometry_r(QRubberBand* this_ptr, const QRect* r);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_setGeometry_x_y_w_h(QRubberBand* this_ptr, int x, int y, int w, int h);
QT_WIDGETS_C_EXPORT QRubberBand::Shape qt_widgets_c_QRubberBand_shape(const QRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QRubberBand_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QRUBBERBAND_H
