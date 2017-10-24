#ifndef QT_GUI_C_QSURFACEFORMAT_H
#define QT_GUI_C_QSURFACEFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_alphaBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_blueBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_constructor_no_args(QSurfaceFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_constructor_options(unsigned int options, QSurfaceFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_constructor_other(const QSurfaceFormat* other, QSurfaceFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_defaultFormat_to_output(QSurfaceFormat* output);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_depthBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_destructor(QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_greenBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSurfaceFormat_hasAlpha(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_majorVersion(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_minorVersion(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT QSurfaceFormat* qt_gui_c_QSurfaceFormat_operator_assign(QSurfaceFormat* this_ptr, const QSurfaceFormat* other);
QT_GUI_C_EXPORT QSurfaceFormat::OpenGLContextProfile qt_gui_c_QSurfaceFormat_profile(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_redBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT QSurfaceFormat::RenderableType qt_gui_c_QSurfaceFormat_renderableType(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_samples(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setAlphaBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setBlueBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setDefaultFormat(const QSurfaceFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setDepthBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setGreenBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setMajorVersion(QSurfaceFormat* this_ptr, int majorVersion);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setMinorVersion(QSurfaceFormat* this_ptr, int minorVersion);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setOption_option(QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setOption_option_on(QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option, bool on);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setProfile(QSurfaceFormat* this_ptr, QSurfaceFormat::OpenGLContextProfile profile);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setRedBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setRenderableType(QSurfaceFormat* this_ptr, QSurfaceFormat::RenderableType type);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setSamples(QSurfaceFormat* this_ptr, int numSamples);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setStencilBufferSize(QSurfaceFormat* this_ptr, int size);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setStereo(QSurfaceFormat* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setSwapBehavior(QSurfaceFormat* this_ptr, QSurfaceFormat::SwapBehavior behavior);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setSwapInterval(QSurfaceFormat* this_ptr, int interval);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_setVersion(QSurfaceFormat* this_ptr, int major, int minor);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_stencilBufferSize(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSurfaceFormat_stereo(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT QSurfaceFormat::SwapBehavior qt_gui_c_QSurfaceFormat_swapBehavior(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSurfaceFormat_swapInterval(const QSurfaceFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSurfaceFormat_testOption(const QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option);
QT_GUI_C_EXPORT void qt_gui_c_QSurfaceFormat_version_to_output(const QSurfaceFormat* this_ptr, QPair< int, int >* output);

} // extern "C"

#endif // QT_GUI_C_QSURFACEFORMAT_H
