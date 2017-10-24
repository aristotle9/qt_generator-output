#ifndef QT_GUI_C_QOPENGLBUFFER_H
#define QT_GUI_C_QOPENGLBUFFER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_allocate_count(QOpenGLBuffer* this_ptr, int count);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_allocate_data_count(QOpenGLBuffer* this_ptr, const void* data, int count);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLBuffer_bind(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLBuffer_bufferId(const QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_constructor_no_args(QOpenGLBuffer* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_constructor_other(const QOpenGLBuffer* other, QOpenGLBuffer* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_constructor_type(const QOpenGLBuffer::Type* type, QOpenGLBuffer* output);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLBuffer_create(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_destroy(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_destructor(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLBuffer_isCreated(const QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLBuffer_map(QOpenGLBuffer* this_ptr, const QOpenGLBuffer::Access* access);
QT_GUI_C_EXPORT QOpenGLBuffer* qt_gui_c_QOpenGLBuffer_operator_assign(QOpenGLBuffer* this_ptr, const QOpenGLBuffer* other);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLBuffer_read(QOpenGLBuffer* this_ptr, int offset, void* data, int count);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_release_no_args(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_release_type(const QOpenGLBuffer::Type* type);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_setUsagePattern(QOpenGLBuffer* this_ptr, const QOpenGLBuffer::UsagePattern* value);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLBuffer_size(const QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLBuffer_unmap(QOpenGLBuffer* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLBuffer_write(QOpenGLBuffer* this_ptr, int offset, const void* data, int count);

} // extern "C"

#endif // QT_GUI_C_QOPENGLBUFFER_H
