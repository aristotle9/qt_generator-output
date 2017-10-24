#ifndef QT_GUI_C_QMOVIE_H
#define QT_GUI_C_QMOVIE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_G_static_cast_QMovie_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QMovie_G_static_cast_QObject_ptr(QMovie* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_backgroundColor_to_output(const QMovie* this_ptr, QColor* output);
QT_GUI_C_EXPORT QMovie::CacheMode qt_gui_c_QMovie_cacheMode(const QMovie* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_currentFrameNumber(const QMovie* this_ptr);
QT_GUI_C_EXPORT QImage* qt_gui_c_QMovie_currentImage_as_ptr(const QMovie* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QMovie_currentPixmap_as_ptr(const QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_delete(QMovie* this_ptr);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QMovie_device(const QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_fileName_to_output(const QMovie* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_format_to_output(const QMovie* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_frameCount(const QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_frameRect_to_output(const QMovie* this_ptr, QRect* output);
QT_GUI_C_EXPORT bool qt_gui_c_QMovie_isValid(const QMovie* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QMovie_jumpToFrame(QMovie* this_ptr, int frameNumber);
QT_GUI_C_EXPORT bool qt_gui_c_QMovie_jumpToNextFrame(QMovie* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_loopCount(const QMovie* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QMovie_metaObject(const QMovie* this_ptr);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_device(QIODevice* device);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_device_format(QIODevice* device, const QByteArray* format);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_device_format_parent(QIODevice* device, const QByteArray* format, QObject* parent);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_fileName(const QString* fileName);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_fileName_format(const QString* fileName, const QByteArray* format);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_fileName_format_parent(const QString* fileName, const QByteArray* format, QObject* parent);
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_no_args();
QT_GUI_C_EXPORT QMovie* qt_gui_c_QMovie_new_parent(QObject* parent);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_nextFrameDelay(const QMovie* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_qt_metacall(QMovie* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QMovie_qt_metacast(QMovie* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_scaledSize_to_output(QMovie* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setBackgroundColor(QMovie* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setCacheMode(QMovie* this_ptr, QMovie::CacheMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setDevice(QMovie* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setFileName(QMovie* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setFormat(QMovie* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setPaused(QMovie* this_ptr, bool paused);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setScaledSize(QMovie* this_ptr, const QSize* size);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_setSpeed(QMovie* this_ptr, int percentSpeed);
QT_GUI_C_EXPORT int qt_gui_c_QMovie_speed(const QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_start(QMovie* this_ptr);
QT_GUI_C_EXPORT QMovie::MovieState qt_gui_c_QMovie_state(const QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_stop(QMovie* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_supportedFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QMovie_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QMOVIE_H
