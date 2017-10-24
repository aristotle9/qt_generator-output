#ifndef QT_GUI_C_QOPENGLTEXTUREBLITTER_H
#define QT_GUI_C_QOPENGLTEXTUREBLITTER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_bind_no_args(QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_bind_target(QOpenGLTextureBlitter* this_ptr, unsigned int target);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_blit(QOpenGLTextureBlitter* this_ptr, GLuint texture, const QMatrix4x4* targetTransform, QOpenGLTextureBlitter::Origin sourceOrigin);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_constructor(QOpenGLTextureBlitter* output);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTextureBlitter_create(QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_destroy(QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_destructor(QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTextureBlitter_isCreated(const QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_release(QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_setOpacity(QOpenGLTextureBlitter* this_ptr, float opacity);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTextureBlitter_setRedBlueSwizzle(QOpenGLTextureBlitter* this_ptr, bool swizzle);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTextureBlitter_supportsExternalOESTarget(const QOpenGLTextureBlitter* this_ptr);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QOpenGLTextureBlitter_targetTransform_as_ptr(const QRectF* target, const QRect* viewport);

} // extern "C"

#endif // QT_GUI_C_QOPENGLTEXTUREBLITTER_H
