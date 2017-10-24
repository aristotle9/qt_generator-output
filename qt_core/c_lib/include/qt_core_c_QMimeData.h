#ifndef QT_CORE_C_QMIMEDATA_H
#define QT_CORE_C_QMIMEDATA_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QMimeData* qt_core_c_QMimeData_G_dynamic_cast_QMimeData_ptr(QObject* ptr);
QT_CORE_C_EXPORT QMimeData* qt_core_c_QMimeData_G_static_cast_QMimeData_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QMimeData_G_static_cast_QObject_ptr(QMimeData* ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_clear(QMimeData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_colorData_to_output(const QMimeData* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_data_to_output(const QMimeData* this_ptr, const QString* mimetype, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_delete(QMimeData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_formats_to_output(const QMimeData* this_ptr, QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasColor(const QMimeData* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasFormat(const QMimeData* this_ptr, const QString* mimetype);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasHtml(const QMimeData* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasImage(const QMimeData* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasText(const QMimeData* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMimeData_hasUrls(const QMimeData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_html_to_output(const QMimeData* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_imageData_to_output(const QMimeData* this_ptr, QVariant* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMimeData_metaObject(const QMimeData* this_ptr);
QT_CORE_C_EXPORT QMimeData* qt_core_c_QMimeData_new();
QT_CORE_C_EXPORT void qt_core_c_QMimeData_removeFormat(QMimeData* this_ptr, const QString* mimetype);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setColorData(QMimeData* this_ptr, const QVariant* color);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setData(QMimeData* this_ptr, const QString* mimetype, const QByteArray* data);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setHtml(QMimeData* this_ptr, const QString* html);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setImageData(QMimeData* this_ptr, const QVariant* image);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setText(QMimeData* this_ptr, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_setUrls(QMimeData* this_ptr, const QList< QUrl >* urls);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_text_to_output(const QMimeData* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeData_urls_to_output(const QMimeData* this_ptr, QList< QUrl >* output);

} // extern "C"

#endif // QT_CORE_C_QMIMEDATA_H
