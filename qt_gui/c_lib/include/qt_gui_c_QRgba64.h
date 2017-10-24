#ifndef QT_GUI_C_QRGBA64_H
#define QT_GUI_C_QRGBA64_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgba64_G_qAlpha(const QRgba64* rgb);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgba64_G_qBlue(const QRgba64* rgb);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgba64_G_qGreen(const QRgba64* rgb);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_G_qPremultiply_to_output(const QRgba64* c, QRgba64* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgba64_G_qRed(const QRgba64* rgb);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_G_qRgba64_to_output_c(quint64 c, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_G_qRgba64_to_output_r_g_b_a(quint16 r, quint16 g, quint16 b, quint16 a, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_G_qUnpremultiply_to_output(const QRgba64* c, QRgba64* output);
QT_GUI_C_EXPORT quint16 qt_gui_c_QRgba64_alpha(const QRgba64* this_ptr);
QT_GUI_C_EXPORT quint8 qt_gui_c_QRgba64_alpha8(const QRgba64* this_ptr);
QT_GUI_C_EXPORT quint16 qt_gui_c_QRgba64_blue(const QRgba64* this_ptr);
QT_GUI_C_EXPORT quint8 qt_gui_c_QRgba64_blue8(const QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_constructor(QRgba64* output);
QT_GUI_C_EXPORT quint64 qt_gui_c_QRgba64_convert_to_unsigned_long_long(const QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_destructor(QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_fromArgb32_to_output(unsigned int rgb, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_fromRgba64_to_output_c(quint64 c, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_fromRgba64_to_output_red_green_blue_alpha(quint16 red, quint16 green, quint16 blue, quint16 alpha, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_fromRgba_to_output(quint8 red, quint8 green, quint8 blue, quint8 alpha, QRgba64* output);
QT_GUI_C_EXPORT quint16 qt_gui_c_QRgba64_green(const QRgba64* this_ptr);
QT_GUI_C_EXPORT quint8 qt_gui_c_QRgba64_green8(const QRgba64* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QRgba64_isOpaque(const QRgba64* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QRgba64_isTransparent(const QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_operator_assign_to_output(QRgba64* this_ptr, quint64 _rgba, QRgba64* output);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_premultiplied_to_output(const QRgba64* this_ptr, QRgba64* output);
QT_GUI_C_EXPORT quint16 qt_gui_c_QRgba64_red(const QRgba64* this_ptr);
QT_GUI_C_EXPORT quint8 qt_gui_c_QRgba64_red8(const QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_setAlpha(QRgba64* this_ptr, quint16 _alpha);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_setBlue(QRgba64* this_ptr, quint16 _blue);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_setGreen(QRgba64* this_ptr, quint16 _green);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_setRed(QRgba64* this_ptr, quint16 _red);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgba64_toArgb32(const QRgba64* this_ptr);
QT_GUI_C_EXPORT unsigned short qt_gui_c_QRgba64_toRgb16(const QRgba64* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRgba64_unpremultiplied_to_output(const QRgba64* this_ptr, QRgba64* output);

} // extern "C"

#endif // QT_GUI_C_QRGBA64_H
