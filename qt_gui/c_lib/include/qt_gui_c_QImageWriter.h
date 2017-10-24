#ifndef QT_GUI_C_QIMAGEWRITER_H
#define QT_GUI_C_QIMAGEWRITER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QImageWriter_canWrite(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageWriter_compression(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_delete(QImageWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_description_to_output(const QImageWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QImageWriter_device(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT QImageWriter::ImageWriterError qt_gui_c_QImageWriter_error(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_errorString_to_output(const QImageWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_fileName_to_output(const QImageWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_format_to_output(const QImageWriter* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT float qt_gui_c_QImageWriter_gamma(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT QImageWriter* qt_gui_c_QImageWriter_new_device_format(QIODevice* device, const QByteArray* format);
QT_GUI_C_EXPORT QImageWriter* qt_gui_c_QImageWriter_new_fileName(const QString* fileName);
QT_GUI_C_EXPORT QImageWriter* qt_gui_c_QImageWriter_new_fileName_format(const QString* fileName, const QByteArray* format);
QT_GUI_C_EXPORT QImageWriter* qt_gui_c_QImageWriter_new_no_args();
QT_GUI_C_EXPORT bool qt_gui_c_QImageWriter_optimizedWrite(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QImageWriter_progressiveScanWrite(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageWriter_quality(const QImageWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setCompression(QImageWriter* this_ptr, int compression);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setDescription(QImageWriter* this_ptr, const QString* description);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setDevice(QImageWriter* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setFileName(QImageWriter* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setFormat(QImageWriter* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setGamma(QImageWriter* this_ptr, float gamma);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setOptimizedWrite(QImageWriter* this_ptr, bool optimize);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setProgressiveScanWrite(QImageWriter* this_ptr, bool progressive);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setQuality(QImageWriter* this_ptr, int quality);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setSubType(QImageWriter* this_ptr, const QByteArray* type);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_setText(QImageWriter* this_ptr, const QString* key, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_subType_to_output(const QImageWriter* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_supportedImageFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_supportedMimeTypes_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_supportedSubTypes_to_output(const QImageWriter* this_ptr, QList< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageWriter_supportsOption(const QImageWriter* this_ptr, const QImageIOHandler::ImageOption* option);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageWriter_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageWriter_write(QImageWriter* this_ptr, const QImage* image);

} // extern "C"

#endif // QT_GUI_C_QIMAGEWRITER_H
