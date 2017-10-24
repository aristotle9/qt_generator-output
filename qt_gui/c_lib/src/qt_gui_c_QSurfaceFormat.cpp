#include "qt_gui_c_QSurfaceFormat.h"

int qt_gui_c_QSurfaceFormat_alphaBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->alphaBufferSize();
}

int qt_gui_c_QSurfaceFormat_blueBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->blueBufferSize();
}

void qt_gui_c_QSurfaceFormat_constructor_no_args(QSurfaceFormat* output) {
  new(output) QSurfaceFormat();
}

void qt_gui_c_QSurfaceFormat_constructor_options(unsigned int options, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(QFlags< QSurfaceFormat::FormatOption >(options));
}

void qt_gui_c_QSurfaceFormat_constructor_other(const QSurfaceFormat* other, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(*other);
}

void qt_gui_c_QSurfaceFormat_defaultFormat_to_output(QSurfaceFormat* output) {
  new(output) QSurfaceFormat(QSurfaceFormat::defaultFormat());
}

int qt_gui_c_QSurfaceFormat_depthBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->depthBufferSize();
}

void qt_gui_c_QSurfaceFormat_destructor(QSurfaceFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QSurfaceFormat_greenBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->greenBufferSize();
}

bool qt_gui_c_QSurfaceFormat_hasAlpha(const QSurfaceFormat* this_ptr) {
  return this_ptr->hasAlpha();
}

int qt_gui_c_QSurfaceFormat_majorVersion(const QSurfaceFormat* this_ptr) {
  return this_ptr->majorVersion();
}

int qt_gui_c_QSurfaceFormat_minorVersion(const QSurfaceFormat* this_ptr) {
  return this_ptr->minorVersion();
}

QSurfaceFormat* qt_gui_c_QSurfaceFormat_operator_assign(QSurfaceFormat* this_ptr, const QSurfaceFormat* other) {
  return &this_ptr->operator=(*other);
}

QSurfaceFormat::OpenGLContextProfile qt_gui_c_QSurfaceFormat_profile(const QSurfaceFormat* this_ptr) {
  return this_ptr->profile();
}

int qt_gui_c_QSurfaceFormat_redBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->redBufferSize();
}

QSurfaceFormat::RenderableType qt_gui_c_QSurfaceFormat_renderableType(const QSurfaceFormat* this_ptr) {
  return this_ptr->renderableType();
}

int qt_gui_c_QSurfaceFormat_samples(const QSurfaceFormat* this_ptr) {
  return this_ptr->samples();
}

void qt_gui_c_QSurfaceFormat_setAlphaBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setAlphaBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setBlueBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setBlueBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setDefaultFormat(const QSurfaceFormat* format) {
  QSurfaceFormat::setDefaultFormat(*format);
}

void qt_gui_c_QSurfaceFormat_setDepthBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setDepthBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setGreenBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setGreenBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setMajorVersion(QSurfaceFormat* this_ptr, int majorVersion) {
  this_ptr->setMajorVersion(majorVersion);
}

void qt_gui_c_QSurfaceFormat_setMinorVersion(QSurfaceFormat* this_ptr, int minorVersion) {
  this_ptr->setMinorVersion(minorVersion);
}

void qt_gui_c_QSurfaceFormat_setOption_option(QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option) {
  this_ptr->setOption(option);
}

void qt_gui_c_QSurfaceFormat_setOption_option_on(QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_gui_c_QSurfaceFormat_setProfile(QSurfaceFormat* this_ptr, QSurfaceFormat::OpenGLContextProfile profile) {
  this_ptr->setProfile(profile);
}

void qt_gui_c_QSurfaceFormat_setRedBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setRedBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setRenderableType(QSurfaceFormat* this_ptr, QSurfaceFormat::RenderableType type) {
  this_ptr->setRenderableType(type);
}

void qt_gui_c_QSurfaceFormat_setSamples(QSurfaceFormat* this_ptr, int numSamples) {
  this_ptr->setSamples(numSamples);
}

void qt_gui_c_QSurfaceFormat_setStencilBufferSize(QSurfaceFormat* this_ptr, int size) {
  this_ptr->setStencilBufferSize(size);
}

void qt_gui_c_QSurfaceFormat_setStereo(QSurfaceFormat* this_ptr, bool enable) {
  this_ptr->setStereo(enable);
}

void qt_gui_c_QSurfaceFormat_setSwapBehavior(QSurfaceFormat* this_ptr, QSurfaceFormat::SwapBehavior behavior) {
  this_ptr->setSwapBehavior(behavior);
}

void qt_gui_c_QSurfaceFormat_setSwapInterval(QSurfaceFormat* this_ptr, int interval) {
  this_ptr->setSwapInterval(interval);
}

void qt_gui_c_QSurfaceFormat_setVersion(QSurfaceFormat* this_ptr, int major, int minor) {
  this_ptr->setVersion(major, minor);
}

int qt_gui_c_QSurfaceFormat_stencilBufferSize(const QSurfaceFormat* this_ptr) {
  return this_ptr->stencilBufferSize();
}

bool qt_gui_c_QSurfaceFormat_stereo(const QSurfaceFormat* this_ptr) {
  return this_ptr->stereo();
}

QSurfaceFormat::SwapBehavior qt_gui_c_QSurfaceFormat_swapBehavior(const QSurfaceFormat* this_ptr) {
  return this_ptr->swapBehavior();
}

int qt_gui_c_QSurfaceFormat_swapInterval(const QSurfaceFormat* this_ptr) {
  return this_ptr->swapInterval();
}

bool qt_gui_c_QSurfaceFormat_testOption(const QSurfaceFormat* this_ptr, QSurfaceFormat::FormatOption option) {
  return this_ptr->testOption(option);
}

void qt_gui_c_QSurfaceFormat_version_to_output(const QSurfaceFormat* this_ptr, QPair< int, int >* output) {
  new(output) QPair< int, int >(this_ptr->version());
}

