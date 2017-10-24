#ifndef QT_GUI_C_QOPENGLVERSIONPROFILE_H
#define QT_GUI_C_QOPENGLVERSIONPROFILE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_constructor_format(const QSurfaceFormat* format, QOpenGLVersionProfile* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_constructor_no_args(QOpenGLVersionProfile* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_constructor_other(const QOpenGLVersionProfile* other, QOpenGLVersionProfile* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_destructor(QOpenGLVersionProfile* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLVersionProfile_hasProfiles(const QOpenGLVersionProfile* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLVersionProfile_isLegacyVersion(const QOpenGLVersionProfile* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLVersionProfile_isValid(const QOpenGLVersionProfile* this_ptr);
QT_GUI_C_EXPORT QOpenGLVersionProfile* qt_gui_c_QOpenGLVersionProfile_operator_assign(QOpenGLVersionProfile* this_ptr, const QOpenGLVersionProfile* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_setProfile(QOpenGLVersionProfile* this_ptr, const QSurfaceFormat::OpenGLContextProfile* profile);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_setVersion(QOpenGLVersionProfile* this_ptr, int majorVersion, int minorVersion);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLVersionProfile_version_to_output(const QOpenGLVersionProfile* this_ptr, QPair< int, int >* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLVERSIONPROFILE_H
