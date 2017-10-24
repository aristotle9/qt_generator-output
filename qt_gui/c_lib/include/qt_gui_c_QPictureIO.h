#ifndef QT_GUI_C_QPICTUREIO_H
#define QT_GUI_C_QPICTUREIO_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_constructor_fileName_format(const QString* fileName, const char* format, QPictureIO* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_constructor_ioDevice_format(QIODevice* ioDevice, const char* format, QPictureIO* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_constructor_no_args(QPictureIO* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_defineIOHandler(const char* format, const char* header, const char* flags, void (*read_picture)(QPictureIO*), void (*write_picture)(QPictureIO*));
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_description_to_output(const QPictureIO* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_destructor(QPictureIO* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_fileName_to_output(const QPictureIO* this_ptr, QString* output);
QT_GUI_C_EXPORT const char* qt_gui_c_QPictureIO_format(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QPictureIO_gamma(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_inputFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QPictureIO_ioDevice(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_outputFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT const char* qt_gui_c_QPictureIO_parameters(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT const QPicture* qt_gui_c_QPictureIO_picture(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_pictureFormat_to_output_arg1(QIODevice* arg1, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_pictureFormat_to_output_fileName(const QString* fileName, QByteArray* output);
QT_GUI_C_EXPORT int qt_gui_c_QPictureIO_quality(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPictureIO_read(QPictureIO* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setDescription(QPictureIO* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setFileName(QPictureIO* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setFormat(QPictureIO* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setGamma(QPictureIO* this_ptr, float arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setIODevice(QPictureIO* this_ptr, QIODevice* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setParameters(QPictureIO* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setPicture(QPictureIO* this_ptr, const QPicture* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setQuality(QPictureIO* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPictureIO_setStatus(QPictureIO* this_ptr, int arg1);
QT_GUI_C_EXPORT int qt_gui_c_QPictureIO_status(const QPictureIO* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPictureIO_write(QPictureIO* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPICTUREIO_H
