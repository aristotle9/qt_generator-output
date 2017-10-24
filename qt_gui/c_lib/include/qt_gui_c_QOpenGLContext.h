#ifndef QT_GUI_C_QOPENGLCONTEXT_H
#define QT_GUI_C_QOPENGLCONTEXT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_G_operator_eq(const QOpenGLVersionProfile* lhs, const QOpenGLVersionProfile* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_G_operator_neq(const QOpenGLVersionProfile* lhs, const QOpenGLVersionProfile* rhs);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QOpenGLContext_G_qHash_v(const QOpenGLVersionProfile* v);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QOpenGLContext_G_qHash_v_seed(const QOpenGLVersionProfile* v, unsigned int seed);
QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(QOpenGLContext* ptr);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_G_static_cast_QOpenGLContext_ptr(QObject* ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_areSharing(QOpenGLContext* first, QOpenGLContext* second);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_create(QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_currentContext();
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLContext_defaultFramebufferObject(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_delete(QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_doneCurrent(QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_extensions_to_output(const QOpenGLContext* this_ptr, QSet< QByteArray >* output);
QT_GUI_C_EXPORT QOpenGLExtraFunctions* qt_gui_c_QOpenGLContext_extraFunctions(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_format_to_output(const QOpenGLContext* this_ptr, QSurfaceFormat* output);
QT_GUI_C_EXPORT QOpenGLFunctions* qt_gui_c_QOpenGLContext_functions(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void (*qt_gui_c_QOpenGLContext_getProcAddress_QByteArray(const QOpenGLContext* this_ptr, const QByteArray* procName))();
QT_GUI_C_EXPORT void (*qt_gui_c_QOpenGLContext_getProcAddress_char(const QOpenGLContext* this_ptr, const char* procName))();
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_globalShareContext();
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_hasExtension(const QOpenGLContext* this_ptr, const QByteArray* extension);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_isOpenGLES(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_isValid(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_makeCurrent(QOpenGLContext* this_ptr, QSurface* surface);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLContext_metaObject(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_nativeHandle_to_output(const QOpenGLContext* this_ptr, QVariant* output);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_new_no_args();
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_new_parent(QObject* parent);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLContext_openGLModuleHandle();
QT_GUI_C_EXPORT QOpenGLContext::OpenGLModuleType qt_gui_c_QOpenGLContext_openGLModuleType();
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLContext_qt_metacall(QOpenGLContext* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLContext_qt_metacast(QOpenGLContext* this_ptr, const char* arg1);
QT_GUI_C_EXPORT QScreen* qt_gui_c_QOpenGLContext_screen(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_setFormat(QOpenGLContext* this_ptr, const QSurfaceFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_setNativeHandle(QOpenGLContext* this_ptr, const QVariant* handle);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_setScreen(QOpenGLContext* this_ptr, QScreen* screen);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_setShareContext(QOpenGLContext* this_ptr, QOpenGLContext* shareContext);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLContext_shareContext(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT QOpenGLContextGroup* qt_gui_c_QOpenGLContext_shareGroup(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLContext_supportsThreadedOpenGL();
QT_GUI_C_EXPORT QSurface* qt_gui_c_QOpenGLContext_surface(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_swapBuffers(QOpenGLContext* this_ptr, QSurface* surface);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContext_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT QAbstractOpenGLFunctions* qt_gui_c_QOpenGLContext_versionFunctions_no_args(const QOpenGLContext* this_ptr);
QT_GUI_C_EXPORT QAbstractOpenGLFunctions* qt_gui_c_QOpenGLContext_versionFunctions_versionProfile(const QOpenGLContext* this_ptr, const QOpenGLVersionProfile* versionProfile);

} // extern "C"

#endif // QT_GUI_C_QOPENGLCONTEXT_H
