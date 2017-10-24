#include "qt_gui_c_QOpenGLShaderProgram.h"

QObject* qt_gui_c_QOpenGLShaderProgram_G_static_cast_QObject_ptr(QOpenGLShaderProgram* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLShaderProgram* qt_gui_c_QOpenGLShaderProgram_G_static_cast_QOpenGLShaderProgram_ptr(QObject* ptr) {
  return static_cast<QOpenGLShaderProgram*>(ptr);
}

bool qt_gui_c_QOpenGLShaderProgram_addShader(QOpenGLShaderProgram* this_ptr, QOpenGLShader* shader) {
  return this_ptr->addShader(shader);
}

int qt_gui_c_QOpenGLShaderProgram_attributeLocation_QByteArray(const QOpenGLShaderProgram* this_ptr, const QByteArray* name) {
  return this_ptr->attributeLocation(*name);
}

int qt_gui_c_QOpenGLShaderProgram_attributeLocation_QString(const QOpenGLShaderProgram* this_ptr, const QString* name) {
  return this_ptr->attributeLocation(*name);
}

int qt_gui_c_QOpenGLShaderProgram_attributeLocation_char(const QOpenGLShaderProgram* this_ptr, const char* name) {
  return this_ptr->attributeLocation(name);
}

bool qt_gui_c_QOpenGLShaderProgram_bind(QOpenGLShaderProgram* this_ptr) {
  return this_ptr->bind();
}

void qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_QByteArray_int(QOpenGLShaderProgram* this_ptr, const QByteArray* name, int location) {
  this_ptr->bindAttributeLocation(*name, location);
}

void qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_QString_int(QOpenGLShaderProgram* this_ptr, const QString* name, int location) {
  this_ptr->bindAttributeLocation(*name, location);
}

void qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_char_int(QOpenGLShaderProgram* this_ptr, const char* name, int location) {
  this_ptr->bindAttributeLocation(name, location);
}

bool qt_gui_c_QOpenGLShaderProgram_create(QOpenGLShaderProgram* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLShaderProgram_defaultInnerTessellationLevels_to_output(const QOpenGLShaderProgram* this_ptr, QVector< float >* output) {
  new(output) QVector< float >(this_ptr->defaultInnerTessellationLevels());
}

void qt_gui_c_QOpenGLShaderProgram_defaultOuterTessellationLevels_to_output(const QOpenGLShaderProgram* this_ptr, QVector< float >* output) {
  new(output) QVector< float >(this_ptr->defaultOuterTessellationLevels());
}

void qt_gui_c_QOpenGLShaderProgram_delete(QOpenGLShaderProgram* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLShaderProgram_disableAttributeArray_location(QOpenGLShaderProgram* this_ptr, int location) {
  this_ptr->disableAttributeArray(location);
}

void qt_gui_c_QOpenGLShaderProgram_disableAttributeArray_name(QOpenGLShaderProgram* this_ptr, const char* name) {
  this_ptr->disableAttributeArray(name);
}

void qt_gui_c_QOpenGLShaderProgram_enableAttributeArray_location(QOpenGLShaderProgram* this_ptr, int location) {
  this_ptr->enableAttributeArray(location);
}

void qt_gui_c_QOpenGLShaderProgram_enableAttributeArray_name(QOpenGLShaderProgram* this_ptr, const char* name) {
  this_ptr->enableAttributeArray(name);
}

bool qt_gui_c_QOpenGLShaderProgram_hasOpenGLShaderPrograms_context(QOpenGLContext* context) {
  return QOpenGLShaderProgram::hasOpenGLShaderPrograms(context);
}

bool qt_gui_c_QOpenGLShaderProgram_hasOpenGLShaderPrograms_no_args() {
  return QOpenGLShaderProgram::hasOpenGLShaderPrograms();
}

bool qt_gui_c_QOpenGLShaderProgram_isLinked(const QOpenGLShaderProgram* this_ptr) {
  return this_ptr->isLinked();
}

bool qt_gui_c_QOpenGLShaderProgram_link(QOpenGLShaderProgram* this_ptr) {
  return this_ptr->link();
}

void qt_gui_c_QOpenGLShaderProgram_log_to_output(const QOpenGLShaderProgram* this_ptr, QString* output) {
  new(output) QString(this_ptr->log());
}

int qt_gui_c_QOpenGLShaderProgram_maxGeometryOutputVertices(const QOpenGLShaderProgram* this_ptr) {
  return this_ptr->maxGeometryOutputVertices();
}

const QMetaObject* qt_gui_c_QOpenGLShaderProgram_metaObject(const QOpenGLShaderProgram* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLShaderProgram* qt_gui_c_QOpenGLShaderProgram_new_no_args() {
  return new QOpenGLShaderProgram();
}

QOpenGLShaderProgram* qt_gui_c_QOpenGLShaderProgram_new_parent(QObject* parent) {
  return new QOpenGLShaderProgram(parent);
}

int qt_gui_c_QOpenGLShaderProgram_patchVertexCount(const QOpenGLShaderProgram* this_ptr) {
  return this_ptr->patchVertexCount();
}

GLuint qt_gui_c_QOpenGLShaderProgram_programId(const QOpenGLShaderProgram* this_ptr) {
  return this_ptr->programId();
}

int qt_gui_c_QOpenGLShaderProgram_qt_metacall(QOpenGLShaderProgram* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLShaderProgram_qt_metacast(QOpenGLShaderProgram* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOpenGLShaderProgram_release(QOpenGLShaderProgram* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QOpenGLShaderProgram_removeAllShaders(QOpenGLShaderProgram* this_ptr) {
  this_ptr->removeAllShaders();
}

void qt_gui_c_QOpenGLShaderProgram_removeShader(QOpenGLShaderProgram* this_ptr, QOpenGLShader* shader) {
  this_ptr->removeShader(shader);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector2D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector2D* values) {
  this_ptr->setAttributeArray(name, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector2D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector2D* values, int stride) {
  this_ptr->setAttributeArray(name, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector3D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector3D* values) {
  this_ptr->setAttributeArray(name, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector3D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector3D* values, int stride) {
  this_ptr->setAttributeArray(name, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector4D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector4D* values) {
  this_ptr->setAttributeArray(name, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector4D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector4D* values, int stride) {
  this_ptr->setAttributeArray(name, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_float_int(QOpenGLShaderProgram* this_ptr, const char* name, const float* values, int tupleSize) {
  this_ptr->setAttributeArray(name, values, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_float_int_int(QOpenGLShaderProgram* this_ptr, const char* name, const float* values, int tupleSize, int stride) {
  this_ptr->setAttributeArray(name, values, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_unsigned_int_void_int(QOpenGLShaderProgram* this_ptr, const char* name, unsigned int type, const void* values, int tupleSize) {
  this_ptr->setAttributeArray(name, type, values, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_unsigned_int_void_int_int(QOpenGLShaderProgram* this_ptr, const char* name, unsigned int type, const void* values, int tupleSize, int stride) {
  this_ptr->setAttributeArray(name, type, values, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector2D(QOpenGLShaderProgram* this_ptr, int location, const QVector2D* values) {
  this_ptr->setAttributeArray(location, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector2D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector2D* values, int stride) {
  this_ptr->setAttributeArray(location, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector3D(QOpenGLShaderProgram* this_ptr, int location, const QVector3D* values) {
  this_ptr->setAttributeArray(location, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector3D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector3D* values, int stride) {
  this_ptr->setAttributeArray(location, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector4D(QOpenGLShaderProgram* this_ptr, int location, const QVector4D* values) {
  this_ptr->setAttributeArray(location, values);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector4D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector4D* values, int stride) {
  this_ptr->setAttributeArray(location, values, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_float_int(QOpenGLShaderProgram* this_ptr, int location, const float* values, int tupleSize) {
  this_ptr->setAttributeArray(location, values, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_float_int_int(QOpenGLShaderProgram* this_ptr, int location, const float* values, int tupleSize, int stride) {
  this_ptr->setAttributeArray(location, values, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_unsigned_int_void_int(QOpenGLShaderProgram* this_ptr, int location, unsigned int type, const void* values, int tupleSize) {
  this_ptr->setAttributeArray(location, type, values, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_unsigned_int_void_int_int(QOpenGLShaderProgram* this_ptr, int location, unsigned int type, const void* values, int tupleSize, int stride) {
  this_ptr->setAttributeArray(location, type, values, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_location_type_offset_tupleSize(QOpenGLShaderProgram* this_ptr, int location, unsigned int type, int offset, int tupleSize) {
  this_ptr->setAttributeBuffer(location, type, offset, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_location_type_offset_tupleSize_stride(QOpenGLShaderProgram* this_ptr, int location, unsigned int type, int offset, int tupleSize, int stride) {
  this_ptr->setAttributeBuffer(location, type, offset, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_name_type_offset_tupleSize(QOpenGLShaderProgram* this_ptr, const char* name, unsigned int type, int offset, int tupleSize) {
  this_ptr->setAttributeBuffer(name, type, offset, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_name_type_offset_tupleSize_stride(QOpenGLShaderProgram* this_ptr, const char* name, unsigned int type, int offset, int tupleSize, int stride) {
  this_ptr->setAttributeBuffer(name, type, offset, tupleSize, stride);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QColor(QOpenGLShaderProgram* this_ptr, const char* name, const QColor* value) {
  this_ptr->setAttributeValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector2D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector2D* value) {
  this_ptr->setAttributeValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector3D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector3D* value) {
  this_ptr->setAttributeValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector4D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector4D* value) {
  this_ptr->setAttributeValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float(QOpenGLShaderProgram* this_ptr, const char* name, float value) {
  this_ptr->setAttributeValue(name, value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y) {
  this_ptr->setAttributeValue(name, x, y);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y, float z) {
  this_ptr->setAttributeValue(name, x, y, z);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y, float z, float w) {
  this_ptr->setAttributeValue(name, x, y, z, w);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_int_int(QOpenGLShaderProgram* this_ptr, const char* name, const float* values, int columns, int rows) {
  this_ptr->setAttributeValue(name, values, columns, rows);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QColor(QOpenGLShaderProgram* this_ptr, int location, const QColor* value) {
  this_ptr->setAttributeValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector2D(QOpenGLShaderProgram* this_ptr, int location, const QVector2D* value) {
  this_ptr->setAttributeValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector3D(QOpenGLShaderProgram* this_ptr, int location, const QVector3D* value) {
  this_ptr->setAttributeValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector4D(QOpenGLShaderProgram* this_ptr, int location, const QVector4D* value) {
  this_ptr->setAttributeValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float(QOpenGLShaderProgram* this_ptr, int location, float value) {
  this_ptr->setAttributeValue(location, value);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y) {
  this_ptr->setAttributeValue(location, x, y);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y, float z) {
  this_ptr->setAttributeValue(location, x, y, z);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y, float z, float w) {
  this_ptr->setAttributeValue(location, x, y, z, w);
}

void qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_int_int(QOpenGLShaderProgram* this_ptr, int location, const float* values, int columns, int rows) {
  this_ptr->setAttributeValue(location, values, columns, rows);
}

void qt_gui_c_QOpenGLShaderProgram_setDefaultInnerTessellationLevels(QOpenGLShaderProgram* this_ptr, const QVector< float >* levels) {
  this_ptr->setDefaultInnerTessellationLevels(*levels);
}

void qt_gui_c_QOpenGLShaderProgram_setDefaultOuterTessellationLevels(QOpenGLShaderProgram* this_ptr, const QVector< float >* levels) {
  this_ptr->setDefaultOuterTessellationLevels(*levels);
}

void qt_gui_c_QOpenGLShaderProgram_setPatchVertexCount(QOpenGLShaderProgram* this_ptr, int count) {
  this_ptr->setPatchVertexCount(count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_GLint_int(QOpenGLShaderProgram* this_ptr, const char* name, const GLint* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_GLuint_int(QOpenGLShaderProgram* this_ptr, const char* name, const GLuint* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QMatrix4x4_int(QOpenGLShaderProgram* this_ptr, const char* name, const QMatrix4x4* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector2D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector2D* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector3D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector3D* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector4D_int(QOpenGLShaderProgram* this_ptr, const char* name, const QVector4D* values, int count) {
  this_ptr->setUniformValueArray(name, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_float_int_int(QOpenGLShaderProgram* this_ptr, const char* name, const float* values, int count, int tupleSize) {
  this_ptr->setUniformValueArray(name, values, count, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_GLint_int(QOpenGLShaderProgram* this_ptr, int location, const GLint* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_GLuint_int(QOpenGLShaderProgram* this_ptr, int location, const GLuint* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QMatrix4x4_int(QOpenGLShaderProgram* this_ptr, int location, const QMatrix4x4* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector2D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector2D* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector3D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector3D* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector4D_int(QOpenGLShaderProgram* this_ptr, int location, const QVector4D* values, int count) {
  this_ptr->setUniformValueArray(location, values, count);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_float_int_int(QOpenGLShaderProgram* this_ptr, int location, const float* values, int count, int tupleSize) {
  this_ptr->setUniformValueArray(location, values, count, tupleSize);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_GLint(QOpenGLShaderProgram* this_ptr, const char* name, GLint value) {
  this_ptr->setUniformValue(name, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_GLuint(QOpenGLShaderProgram* this_ptr, const char* name, GLuint value) {
  this_ptr->setUniformValue(name, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QColor(QOpenGLShaderProgram* this_ptr, const char* name, const QColor* color) {
  this_ptr->setUniformValue(name, *color);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QMatrix4x4(QOpenGLShaderProgram* this_ptr, const char* name, const QMatrix4x4* value) {
  this_ptr->setUniformValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QPoint(QOpenGLShaderProgram* this_ptr, const char* name, const QPoint* point) {
  this_ptr->setUniformValue(name, *point);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QPointF(QOpenGLShaderProgram* this_ptr, const char* name, const QPointF* point) {
  this_ptr->setUniformValue(name, *point);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QSize(QOpenGLShaderProgram* this_ptr, const char* name, const QSize* size) {
  this_ptr->setUniformValue(name, *size);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QSizeF(QOpenGLShaderProgram* this_ptr, const char* name, const QSizeF* size) {
  this_ptr->setUniformValue(name, *size);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QTransform(QOpenGLShaderProgram* this_ptr, const char* name, const QTransform* value) {
  this_ptr->setUniformValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector2D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector2D* value) {
  this_ptr->setUniformValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector3D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector3D* value) {
  this_ptr->setUniformValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector4D(QOpenGLShaderProgram* this_ptr, const char* name, const QVector4D* value) {
  this_ptr->setUniformValue(name, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float(QOpenGLShaderProgram* this_ptr, const char* name, float value) {
  this_ptr->setUniformValue(name, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y) {
  this_ptr->setUniformValue(name, x, y);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y, float z) {
  this_ptr->setUniformValue(name, x, y, z);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float_float_float(QOpenGLShaderProgram* this_ptr, const char* name, float x, float y, float z, float w) {
  this_ptr->setUniformValue(name, x, y, z, w);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_GLint(QOpenGLShaderProgram* this_ptr, int location, GLint value) {
  this_ptr->setUniformValue(location, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_GLuint(QOpenGLShaderProgram* this_ptr, int location, GLuint value) {
  this_ptr->setUniformValue(location, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QColor(QOpenGLShaderProgram* this_ptr, int location, const QColor* color) {
  this_ptr->setUniformValue(location, *color);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QMatrix4x4(QOpenGLShaderProgram* this_ptr, int location, const QMatrix4x4* value) {
  this_ptr->setUniformValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QPoint(QOpenGLShaderProgram* this_ptr, int location, const QPoint* point) {
  this_ptr->setUniformValue(location, *point);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QPointF(QOpenGLShaderProgram* this_ptr, int location, const QPointF* point) {
  this_ptr->setUniformValue(location, *point);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QSize(QOpenGLShaderProgram* this_ptr, int location, const QSize* size) {
  this_ptr->setUniformValue(location, *size);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QSizeF(QOpenGLShaderProgram* this_ptr, int location, const QSizeF* size) {
  this_ptr->setUniformValue(location, *size);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QTransform(QOpenGLShaderProgram* this_ptr, int location, const QTransform* value) {
  this_ptr->setUniformValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector2D(QOpenGLShaderProgram* this_ptr, int location, const QVector2D* value) {
  this_ptr->setUniformValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector3D(QOpenGLShaderProgram* this_ptr, int location, const QVector3D* value) {
  this_ptr->setUniformValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector4D(QOpenGLShaderProgram* this_ptr, int location, const QVector4D* value) {
  this_ptr->setUniformValue(location, *value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float(QOpenGLShaderProgram* this_ptr, int location, float value) {
  this_ptr->setUniformValue(location, value);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y) {
  this_ptr->setUniformValue(location, x, y);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y, float z) {
  this_ptr->setUniformValue(location, x, y, z);
}

void qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float_float_float(QOpenGLShaderProgram* this_ptr, int location, float x, float y, float z, float w) {
  this_ptr->setUniformValue(location, x, y, z, w);
}

void qt_gui_c_QOpenGLShaderProgram_shaders_to_output(const QOpenGLShaderProgram* this_ptr, QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >(this_ptr->shaders());
}

void qt_gui_c_QOpenGLShaderProgram_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLShaderProgram::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLShaderProgram_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLShaderProgram::tr(s, c, n));
}

int qt_gui_c_QOpenGLShaderProgram_uniformLocation_QByteArray(const QOpenGLShaderProgram* this_ptr, const QByteArray* name) {
  return this_ptr->uniformLocation(*name);
}

int qt_gui_c_QOpenGLShaderProgram_uniformLocation_QString(const QOpenGLShaderProgram* this_ptr, const QString* name) {
  return this_ptr->uniformLocation(*name);
}

int qt_gui_c_QOpenGLShaderProgram_uniformLocation_char(const QOpenGLShaderProgram* this_ptr, const char* name) {
  return this_ptr->uniformLocation(name);
}

