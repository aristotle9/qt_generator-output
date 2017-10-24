#ifndef QT_GUI_C_QOPENGLSHADER_H
#define QT_GUI_C_QOPENGLSHADER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(QOpenGLShader* ptr);
QT_GUI_C_EXPORT QOpenGLShader* qt_gui_c_QOpenGLShader_G_static_cast_QOpenGLShader_ptr(QObject* ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_compileSourceCode_QByteArray(QOpenGLShader* this_ptr, const QByteArray* source);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_compileSourceCode_QString(QOpenGLShader* this_ptr, const QString* source);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_compileSourceCode_char(QOpenGLShader* this_ptr, const char* source);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_compileSourceFile(QOpenGLShader* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLShader_delete(QOpenGLShader* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_hasOpenGLShaders_type(unsigned int type);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_hasOpenGLShaders_type_context(unsigned int type, QOpenGLContext* context);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLShader_isCompiled(const QOpenGLShader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLShader_log_to_output(const QOpenGLShader* this_ptr, QString* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLShader_metaObject(const QOpenGLShader* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLShader_qt_metacall(QOpenGLShader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLShader_qt_metacast(QOpenGLShader* this_ptr, const char* arg1);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLShader_shaderId(const QOpenGLShader* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLShader_sourceCode_to_output(const QOpenGLShader* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLShader_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLShader_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLSHADER_H
