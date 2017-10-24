#ifndef QT_GUI_C_QIMAGEIOHANDLER_H
#define QT_GUI_C_QIMAGEIOHANDLER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_canRead(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageIOHandler_currentImageNumber(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_currentImageRect_to_output(const QImageIOHandler* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_delete(QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QImageIOHandler_device(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_format_to_output(const QImageIOHandler* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT int qt_gui_c_QImageIOHandler_imageCount(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_jumpToImage(QImageIOHandler* this_ptr, int imageNumber);
QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_jumpToNextImage(QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageIOHandler_loopCount(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_name_to_output(const QImageIOHandler* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT int qt_gui_c_QImageIOHandler_nextImageDelay(const QImageIOHandler* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_option_to_output(const QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option, QVariant* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_read(QImageIOHandler* this_ptr, QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_setDevice(QImageIOHandler* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_setFormat(QImageIOHandler* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_setFormat_const(const QImageIOHandler* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOHandler_setOption(QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option, const QVariant* value);
QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_supportsOption(const QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option);
QT_GUI_C_EXPORT bool qt_gui_c_QImageIOHandler_write(QImageIOHandler* this_ptr, const QImage* image);

} // extern "C"

#endif // QT_GUI_C_QIMAGEIOHANDLER_H
