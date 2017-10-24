#ifndef QT_GUI_C_QPDFWRITER_H
#define QT_GUI_C_QPDFWRITER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPagedPaintDevice(QPagedPaintDevice* ptr);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QPdfWriter_G_static_cast_QObject_ptr(QPdfWriter* ptr);
QT_GUI_C_EXPORT QPagedPaintDevice* qt_gui_c_QPdfWriter_G_static_cast_QPagedPaintDevice_ptr(QPdfWriter* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPdfWriter_G_static_cast_QPaintDevice_ptr(QPdfWriter* ptr);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPagedPaintDevice(QPagedPaintDevice* ptr);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_creator_to_output(const QPdfWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_delete(QPdfWriter* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QPdfWriter_metaObject(const QPdfWriter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPdfWriter_newPage(QPdfWriter* this_ptr);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_new_device(QIODevice* device);
QT_GUI_C_EXPORT QPdfWriter* qt_gui_c_QPdfWriter_new_filename(const QString* filename);
QT_GUI_C_EXPORT int qt_gui_c_QPdfWriter_qt_metacall(QPdfWriter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QPdfWriter_qt_metacast(QPdfWriter* this_ptr, const char* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QPdfWriter_resolution(const QPdfWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setCreator(QPdfWriter* this_ptr, const QString* creator);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setMargins(QPdfWriter* this_ptr, const QPagedPaintDevice::Margins* m);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setPageSize(QPdfWriter* this_ptr, QPagedPaintDevice::PageSize size);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setPageSizeMM(QPdfWriter* this_ptr, const QSizeF* size);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setResolution(QPdfWriter* this_ptr, int resolution);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_setTitle(QPdfWriter* this_ptr, const QString* title);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_title_to_output(const QPdfWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPdfWriter_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QPDFWRITER_H
