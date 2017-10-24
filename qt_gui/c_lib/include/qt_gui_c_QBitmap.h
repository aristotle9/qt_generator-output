#ifndef QT_GUI_C_QBITMAP_H
#define QT_GUI_C_QBITMAP_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPixmap(QPixmap* ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPixmap(QPixmap* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QBitmap_G_static_cast_QPaintDevice_ptr(QBitmap* ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(QBitmap* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBitmap_G_swap(QBitmap* value1, QBitmap* value2);
QT_GUI_C_EXPORT void qt_gui_c_QBitmap_clear(QBitmap* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBitmap_convert_to_QVariant_to_output(const QBitmap* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QBitmap_delete(QBitmap* this_ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_fromData_as_ptr_size_bits(const QSize* size, const unsigned char* bits);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_fromData_as_ptr_size_bits_monoFormat(const QSize* size, const unsigned char* bits, const QImage::Format* monoFormat);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_QBitmap(const QBitmap* other);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_QPixmap(const QPixmap* arg1);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_QSize(const QSize* arg1);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_QString(const QString* fileName);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_QString_char(const QString* fileName, const char* format);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_int_int(int w, int h);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_new_no_args();
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_operator_assign_arg1(QBitmap* this_ptr, const QPixmap* arg1);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_operator_assign_other(QBitmap* this_ptr, const QBitmap* other);
QT_GUI_C_EXPORT void qt_gui_c_QBitmap_swap(QBitmap* this_ptr, QBitmap* other);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_transformed_as_ptr_arg1(const QBitmap* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QBitmap_transformed_as_ptr_matrix(const QBitmap* this_ptr, const QTransform* matrix);

} // extern "C"

#endif // QT_GUI_C_QBITMAP_H
