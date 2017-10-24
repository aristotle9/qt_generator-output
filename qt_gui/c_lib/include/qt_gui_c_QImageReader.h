#ifndef QT_GUI_C_QIMAGEREADER_H
#define QT_GUI_C_QIMAGEREADER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_autoDetectImageFormat(const QImageReader* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_autoTransform(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_backgroundColor_to_output(const QImageReader* this_ptr, QColor* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_canRead(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_clipRect_to_output(const QImageReader* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_constructor_device(QIODevice* device, QImageReader* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_constructor_device_format(QIODevice* device, const QByteArray* format, QImageReader* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_constructor_fileName(const QString* fileName, QImageReader* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_constructor_fileName_format(const QString* fileName, const QByteArray* format, QImageReader* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_constructor_no_args(QImageReader* output);
QT_GUI_C_EXPORT int qt_gui_c_QImageReader_currentImageNumber(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_currentImageRect_to_output(const QImageReader* this_ptr, QRect* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_decideFormatFromContent(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_destructor(QImageReader* this_ptr);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QImageReader_device(const QImageReader* this_ptr);
QT_GUI_C_EXPORT QImageReader::ImageReaderError qt_gui_c_QImageReader_error(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_errorString_to_output(const QImageReader* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_fileName_to_output(const QImageReader* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_format_to_output(const QImageReader* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT float qt_gui_c_QImageReader_gamma(const QImageReader* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageReader_imageCount(const QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_imageFormat_to_output_device(QIODevice* device, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_imageFormat_to_output_fileName(const QString* fileName, QByteArray* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_jumpToImage(QImageReader* this_ptr, int imageNumber);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_jumpToNextImage(QImageReader* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageReader_loopCount(const QImageReader* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageReader_nextImageDelay(const QImageReader* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageReader_quality(const QImageReader* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_read(QImageReader* this_ptr, QImage* image);
QT_GUI_C_EXPORT QImage* qt_gui_c_QImageReader_read_as_ptr(QImageReader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_scaledClipRect_to_output(const QImageReader* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_scaledSize_to_output(const QImageReader* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setAutoDetectImageFormat(QImageReader* this_ptr, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setAutoTransform(QImageReader* this_ptr, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setBackgroundColor(QImageReader* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setClipRect(QImageReader* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setDecideFormatFromContent(QImageReader* this_ptr, bool ignored);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setDevice(QImageReader* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setFileName(QImageReader* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setFormat(QImageReader* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setGamma(QImageReader* this_ptr, float gamma);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setQuality(QImageReader* this_ptr, int quality);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setScaledClipRect(QImageReader* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_setScaledSize(QImageReader* this_ptr, const QSize* size);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_size_to_output(const QImageReader* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_subType_to_output(const QImageReader* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_supportedImageFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_supportedMimeTypes_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_supportedSubTypes_to_output(const QImageReader* this_ptr, QList< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_supportsAnimation(const QImageReader* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QImageReader_supportsOption(const QImageReader* this_ptr, const QImageIOHandler::ImageOption* option);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_textKeys_to_output(const QImageReader* this_ptr, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_text_to_output(const QImageReader* this_ptr, const QString* key, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageReader_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QIMAGEREADER_H
