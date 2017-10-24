#include "qt_gui_c_QOpenGLShader.h"

QObject* qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(QOpenGLShader* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLShader* qt_gui_c_QOpenGLShader_G_static_cast_QOpenGLShader_ptr(QObject* ptr) {
  return static_cast<QOpenGLShader*>(ptr);
}

bool qt_gui_c_QOpenGLShader_compileSourceCode_QByteArray(QOpenGLShader* this_ptr, const QByteArray* source) {
  return this_ptr->compileSourceCode(*source);
}

bool qt_gui_c_QOpenGLShader_compileSourceCode_QString(QOpenGLShader* this_ptr, const QString* source) {
  return this_ptr->compileSourceCode(*source);
}

bool qt_gui_c_QOpenGLShader_compileSourceCode_char(QOpenGLShader* this_ptr, const char* source) {
  return this_ptr->compileSourceCode(source);
}

bool qt_gui_c_QOpenGLShader_compileSourceFile(QOpenGLShader* this_ptr, const QString* fileName) {
  return this_ptr->compileSourceFile(*fileName);
}

void qt_gui_c_QOpenGLShader_delete(QOpenGLShader* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QOpenGLShader_hasOpenGLShaders_type(unsigned int type) {
  return QOpenGLShader::hasOpenGLShaders(QFlags< QOpenGLShader::ShaderTypeBit >(type));
}

bool qt_gui_c_QOpenGLShader_hasOpenGLShaders_type_context(unsigned int type, QOpenGLContext* context) {
  return QOpenGLShader::hasOpenGLShaders(QFlags< QOpenGLShader::ShaderTypeBit >(type), context);
}

bool qt_gui_c_QOpenGLShader_isCompiled(const QOpenGLShader* this_ptr) {
  return this_ptr->isCompiled();
}

void qt_gui_c_QOpenGLShader_log_to_output(const QOpenGLShader* this_ptr, QString* output) {
  new(output) QString(this_ptr->log());
}

const QMetaObject* qt_gui_c_QOpenGLShader_metaObject(const QOpenGLShader* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QOpenGLShader_qt_metacall(QOpenGLShader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLShader_qt_metacast(QOpenGLShader* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

GLuint qt_gui_c_QOpenGLShader_shaderId(const QOpenGLShader* this_ptr) {
  return this_ptr->shaderId();
}

void qt_gui_c_QOpenGLShader_sourceCode_to_output(const QOpenGLShader* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->sourceCode());
}

void qt_gui_c_QOpenGLShader_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLShader::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLShader_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLShader::tr(s, c, n));
}

