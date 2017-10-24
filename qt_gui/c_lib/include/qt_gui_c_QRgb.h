#ifndef QT_GUI_C_QRGB_H
#define QT_GUI_C_QRGB_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qAlpha(unsigned int rgb);
QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qBlue(unsigned int rgb);
QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qGray_r_g_b(int r, int g, int b);
QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qGray_rgb(unsigned int rgb);
QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qGreen(unsigned int rgb);
QT_GUI_C_EXPORT bool qt_gui_c_QRgb_G_qIsGray(unsigned int rgb);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgb_G_qPremultiply(unsigned int x);
QT_GUI_C_EXPORT int qt_gui_c_QRgb_G_qRed(unsigned int rgb);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgb_G_qRgb(int r, int g, int b);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgb_G_qRgba(int r, int g, int b, int a);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRgb_G_qUnpremultiply(unsigned int p);

} // extern "C"

#endif // QT_GUI_C_QRGB_H
