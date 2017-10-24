#include "qt_gui_c_QOpenGLVersionProfile.h"

void qt_gui_c_QOpenGLVersionProfile_constructor_format(const QSurfaceFormat* format, QOpenGLVersionProfile* output) {
  new(output) QOpenGLVersionProfile(*format);
}

void qt_gui_c_QOpenGLVersionProfile_constructor_no_args(QOpenGLVersionProfile* output) {
  new(output) QOpenGLVersionProfile();
}

void qt_gui_c_QOpenGLVersionProfile_constructor_other(const QOpenGLVersionProfile* other, QOpenGLVersionProfile* output) {
  new(output) QOpenGLVersionProfile(*other);
}

void qt_gui_c_QOpenGLVersionProfile_destructor(QOpenGLVersionProfile* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QOpenGLVersionProfile_hasProfiles(const QOpenGLVersionProfile* this_ptr) {
  return this_ptr->hasProfiles();
}

bool qt_gui_c_QOpenGLVersionProfile_isLegacyVersion(const QOpenGLVersionProfile* this_ptr) {
  return this_ptr->isLegacyVersion();
}

bool qt_gui_c_QOpenGLVersionProfile_isValid(const QOpenGLVersionProfile* this_ptr) {
  return this_ptr->isValid();
}

QOpenGLVersionProfile* qt_gui_c_QOpenGLVersionProfile_operator_assign(QOpenGLVersionProfile* this_ptr, const QOpenGLVersionProfile* rhs) {
  return &this_ptr->operator=(*rhs);
}

void qt_gui_c_QOpenGLVersionProfile_setProfile(QOpenGLVersionProfile* this_ptr, const QSurfaceFormat::OpenGLContextProfile* profile) {
  this_ptr->setProfile(*profile);
}

void qt_gui_c_QOpenGLVersionProfile_setVersion(QOpenGLVersionProfile* this_ptr, int majorVersion, int minorVersion) {
  this_ptr->setVersion(majorVersion, minorVersion);
}

void qt_gui_c_QOpenGLVersionProfile_version_to_output(const QOpenGLVersionProfile* this_ptr, QPair< int, int >* output) {
  new(output) QPair< int, int >(this_ptr->version());
}

