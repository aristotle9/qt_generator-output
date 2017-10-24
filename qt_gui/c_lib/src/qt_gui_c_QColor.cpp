#include "qt_gui_c_QColor.h"

QDataStream* qt_gui_c_QColor_G_operator_shl(QDataStream* arg1, const QColor* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QColor_G_operator_shl_to_output(const QDebug* arg1, const QColor* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QColor_G_operator_shr(QDataStream* arg1, QColor* arg2) {
  return &operator>>(*arg1, *arg2);
}

int qt_gui_c_QColor_alpha(const QColor* this_ptr) {
  return this_ptr->alpha();
}

double qt_gui_c_QColor_alphaF(const QColor* this_ptr) {
  return this_ptr->alphaF();
}

int qt_gui_c_QColor_black(const QColor* this_ptr) {
  return this_ptr->black();
}

double qt_gui_c_QColor_blackF(const QColor* this_ptr) {
  return this_ptr->blackF();
}

int qt_gui_c_QColor_blue(const QColor* this_ptr) {
  return this_ptr->blue();
}

double qt_gui_c_QColor_blueF(const QColor* this_ptr) {
  return this_ptr->blueF();
}

void qt_gui_c_QColor_colorNames_to_output(QStringList* output) {
  new(output) QStringList(QColor::colorNames());
}

void qt_gui_c_QColor_constructor_QColor(const QColor* color, QColor* output) {
  new(output) QColor(*color);
}

void qt_gui_c_QColor_constructor_QColor_Spec(QColor::Spec spec, QColor* output) {
  new(output) QColor(spec);
}

void qt_gui_c_QColor_constructor_QLatin1String(const QLatin1String* name, QColor* output) {
  new(output) QColor(*name);
}

void qt_gui_c_QColor_constructor_QRgba64(const QRgba64* rgba64, QColor* output) {
  new(output) QColor(*rgba64);
}

void qt_gui_c_QColor_constructor_QString(const QString* name, QColor* output) {
  new(output) QColor(*name);
}

void qt_gui_c_QColor_constructor_Qt_GlobalColor(const Qt::GlobalColor* color, QColor* output) {
  new(output) QColor(*color);
}

void qt_gui_c_QColor_constructor_char(const char* aname, QColor* output) {
  new(output) QColor(aname);
}

void qt_gui_c_QColor_constructor_int_int_int(int r, int g, int b, QColor* output) {
  new(output) QColor(r, g, b);
}

void qt_gui_c_QColor_constructor_int_int_int_int(int r, int g, int b, int a, QColor* output) {
  new(output) QColor(r, g, b, a);
}

void qt_gui_c_QColor_constructor_no_args(QColor* output) {
  new(output) QColor();
}

void qt_gui_c_QColor_constructor_unsigned_int(unsigned int rgb, QColor* output) {
  new(output) QColor(rgb);
}

void qt_gui_c_QColor_convertTo_to_output(const QColor* this_ptr, QColor::Spec colorSpec, QColor* output) {
  new(output) QColor(this_ptr->convertTo(colorSpec));
}

void qt_gui_c_QColor_convert_to_QVariant_to_output(const QColor* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

int qt_gui_c_QColor_cyan(const QColor* this_ptr) {
  return this_ptr->cyan();
}

double qt_gui_c_QColor_cyanF(const QColor* this_ptr) {
  return this_ptr->cyanF();
}

void qt_gui_c_QColor_dark_to_output_f(const QColor* this_ptr, int f, QColor* output) {
  new(output) QColor(this_ptr->dark(f));
}

void qt_gui_c_QColor_dark_to_output_no_args(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->dark());
}

void qt_gui_c_QColor_darker_to_output_f(const QColor* this_ptr, int f, QColor* output) {
  new(output) QColor(this_ptr->darker(f));
}

void qt_gui_c_QColor_darker_to_output_no_args(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->darker());
}

void qt_gui_c_QColor_destructor(QColor* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QColor_fromCmykF_to_output_c_m_y_k(double c, double m, double y, double k, QColor* output) {
  new(output) QColor(QColor::fromCmykF(c, m, y, k));
}

void qt_gui_c_QColor_fromCmykF_to_output_c_m_y_k_a(double c, double m, double y, double k, double a, QColor* output) {
  new(output) QColor(QColor::fromCmykF(c, m, y, k, a));
}

void qt_gui_c_QColor_fromCmyk_to_output_c_m_y_k(int c, int m, int y, int k, QColor* output) {
  new(output) QColor(QColor::fromCmyk(c, m, y, k));
}

void qt_gui_c_QColor_fromCmyk_to_output_c_m_y_k_a(int c, int m, int y, int k, int a, QColor* output) {
  new(output) QColor(QColor::fromCmyk(c, m, y, k, a));
}

void qt_gui_c_QColor_fromHslF_to_output_h_s_l(double h, double s, double l, QColor* output) {
  new(output) QColor(QColor::fromHslF(h, s, l));
}

void qt_gui_c_QColor_fromHslF_to_output_h_s_l_a(double h, double s, double l, double a, QColor* output) {
  new(output) QColor(QColor::fromHslF(h, s, l, a));
}

void qt_gui_c_QColor_fromHsl_to_output_h_s_l(int h, int s, int l, QColor* output) {
  new(output) QColor(QColor::fromHsl(h, s, l));
}

void qt_gui_c_QColor_fromHsl_to_output_h_s_l_a(int h, int s, int l, int a, QColor* output) {
  new(output) QColor(QColor::fromHsl(h, s, l, a));
}

void qt_gui_c_QColor_fromHsvF_to_output_h_s_v(double h, double s, double v, QColor* output) {
  new(output) QColor(QColor::fromHsvF(h, s, v));
}

void qt_gui_c_QColor_fromHsvF_to_output_h_s_v_a(double h, double s, double v, double a, QColor* output) {
  new(output) QColor(QColor::fromHsvF(h, s, v, a));
}

void qt_gui_c_QColor_fromHsv_to_output_h_s_v(int h, int s, int v, QColor* output) {
  new(output) QColor(QColor::fromHsv(h, s, v));
}

void qt_gui_c_QColor_fromHsv_to_output_h_s_v_a(int h, int s, int v, int a, QColor* output) {
  new(output) QColor(QColor::fromHsv(h, s, v, a));
}

void qt_gui_c_QColor_fromRgbF_to_output_r_g_b(double r, double g, double b, QColor* output) {
  new(output) QColor(QColor::fromRgbF(r, g, b));
}

void qt_gui_c_QColor_fromRgbF_to_output_r_g_b_a(double r, double g, double b, double a, QColor* output) {
  new(output) QColor(QColor::fromRgbF(r, g, b, a));
}

void qt_gui_c_QColor_fromRgb_to_output_r_g_b(int r, int g, int b, QColor* output) {
  new(output) QColor(QColor::fromRgb(r, g, b));
}

void qt_gui_c_QColor_fromRgb_to_output_r_g_b_a(int r, int g, int b, int a, QColor* output) {
  new(output) QColor(QColor::fromRgb(r, g, b, a));
}

void qt_gui_c_QColor_fromRgb_to_output_rgb(unsigned int rgb, QColor* output) {
  new(output) QColor(QColor::fromRgb(rgb));
}

void qt_gui_c_QColor_fromRgba64_to_output_r_g_b(unsigned short r, unsigned short g, unsigned short b, QColor* output) {
  new(output) QColor(QColor::fromRgba64(r, g, b));
}

void qt_gui_c_QColor_fromRgba64_to_output_r_g_b_a(unsigned short r, unsigned short g, unsigned short b, unsigned short a, QColor* output) {
  new(output) QColor(QColor::fromRgba64(r, g, b, a));
}

void qt_gui_c_QColor_fromRgba64_to_output_rgba(const QRgba64* rgba, QColor* output) {
  new(output) QColor(QColor::fromRgba64(*rgba));
}

void qt_gui_c_QColor_fromRgba_to_output(unsigned int rgba, QColor* output) {
  new(output) QColor(QColor::fromRgba(rgba));
}

void qt_gui_c_QColor_getCmykF_c_m_y_k(QColor* this_ptr, double* c, double* m, double* y, double* k) {
  this_ptr->getCmykF(c, m, y, k);
}

void qt_gui_c_QColor_getCmykF_c_m_y_k_a(QColor* this_ptr, double* c, double* m, double* y, double* k, double* a) {
  this_ptr->getCmykF(c, m, y, k, a);
}

void qt_gui_c_QColor_getCmyk_c_m_y_k(QColor* this_ptr, int* c, int* m, int* y, int* k) {
  this_ptr->getCmyk(c, m, y, k);
}

void qt_gui_c_QColor_getCmyk_c_m_y_k_a(QColor* this_ptr, int* c, int* m, int* y, int* k, int* a) {
  this_ptr->getCmyk(c, m, y, k, a);
}

void qt_gui_c_QColor_getHslF_h_s_l(const QColor* this_ptr, double* h, double* s, double* l) {
  this_ptr->getHslF(h, s, l);
}

void qt_gui_c_QColor_getHslF_h_s_l_a(const QColor* this_ptr, double* h, double* s, double* l, double* a) {
  this_ptr->getHslF(h, s, l, a);
}

void qt_gui_c_QColor_getHsl_h_s_l(const QColor* this_ptr, int* h, int* s, int* l) {
  this_ptr->getHsl(h, s, l);
}

void qt_gui_c_QColor_getHsl_h_s_l_a(const QColor* this_ptr, int* h, int* s, int* l, int* a) {
  this_ptr->getHsl(h, s, l, a);
}

void qt_gui_c_QColor_getHsvF_h_s_v(const QColor* this_ptr, double* h, double* s, double* v) {
  this_ptr->getHsvF(h, s, v);
}

void qt_gui_c_QColor_getHsvF_h_s_v_a(const QColor* this_ptr, double* h, double* s, double* v, double* a) {
  this_ptr->getHsvF(h, s, v, a);
}

void qt_gui_c_QColor_getHsv_h_s_v(const QColor* this_ptr, int* h, int* s, int* v) {
  this_ptr->getHsv(h, s, v);
}

void qt_gui_c_QColor_getHsv_h_s_v_a(const QColor* this_ptr, int* h, int* s, int* v, int* a) {
  this_ptr->getHsv(h, s, v, a);
}

void qt_gui_c_QColor_getRgbF_r_g_b(const QColor* this_ptr, double* r, double* g, double* b) {
  this_ptr->getRgbF(r, g, b);
}

void qt_gui_c_QColor_getRgbF_r_g_b_a(const QColor* this_ptr, double* r, double* g, double* b, double* a) {
  this_ptr->getRgbF(r, g, b, a);
}

void qt_gui_c_QColor_getRgb_r_g_b(const QColor* this_ptr, int* r, int* g, int* b) {
  this_ptr->getRgb(r, g, b);
}

void qt_gui_c_QColor_getRgb_r_g_b_a(const QColor* this_ptr, int* r, int* g, int* b, int* a) {
  this_ptr->getRgb(r, g, b, a);
}

int qt_gui_c_QColor_green(const QColor* this_ptr) {
  return this_ptr->green();
}

double qt_gui_c_QColor_greenF(const QColor* this_ptr) {
  return this_ptr->greenF();
}

int qt_gui_c_QColor_hslHue(const QColor* this_ptr) {
  return this_ptr->hslHue();
}

double qt_gui_c_QColor_hslHueF(const QColor* this_ptr) {
  return this_ptr->hslHueF();
}

int qt_gui_c_QColor_hslSaturation(const QColor* this_ptr) {
  return this_ptr->hslSaturation();
}

double qt_gui_c_QColor_hslSaturationF(const QColor* this_ptr) {
  return this_ptr->hslSaturationF();
}

int qt_gui_c_QColor_hsvHue(const QColor* this_ptr) {
  return this_ptr->hsvHue();
}

double qt_gui_c_QColor_hsvHueF(const QColor* this_ptr) {
  return this_ptr->hsvHueF();
}

int qt_gui_c_QColor_hsvSaturation(const QColor* this_ptr) {
  return this_ptr->hsvSaturation();
}

double qt_gui_c_QColor_hsvSaturationF(const QColor* this_ptr) {
  return this_ptr->hsvSaturationF();
}

int qt_gui_c_QColor_hue(const QColor* this_ptr) {
  return this_ptr->hue();
}

double qt_gui_c_QColor_hueF(const QColor* this_ptr) {
  return this_ptr->hueF();
}

bool qt_gui_c_QColor_isValid(const QColor* this_ptr) {
  return this_ptr->isValid();
}

bool qt_gui_c_QColor_isValidColor_arg1(const QLatin1String* arg1) {
  return QColor::isValidColor(*arg1);
}

bool qt_gui_c_QColor_isValidColor_name(const QString* name) {
  return QColor::isValidColor(*name);
}

void qt_gui_c_QColor_light_to_output_f(const QColor* this_ptr, int f, QColor* output) {
  new(output) QColor(this_ptr->light(f));
}

void qt_gui_c_QColor_light_to_output_no_args(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->light());
}

void qt_gui_c_QColor_lighter_to_output_f(const QColor* this_ptr, int f, QColor* output) {
  new(output) QColor(this_ptr->lighter(f));
}

void qt_gui_c_QColor_lighter_to_output_no_args(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->lighter());
}

int qt_gui_c_QColor_lightness(const QColor* this_ptr) {
  return this_ptr->lightness();
}

double qt_gui_c_QColor_lightnessF(const QColor* this_ptr) {
  return this_ptr->lightnessF();
}

int qt_gui_c_QColor_magenta(const QColor* this_ptr) {
  return this_ptr->magenta();
}

double qt_gui_c_QColor_magentaF(const QColor* this_ptr) {
  return this_ptr->magentaF();
}

void qt_gui_c_QColor_name_to_output_format(const QColor* this_ptr, QColor::NameFormat format, QString* output) {
  new(output) QString(this_ptr->name(format));
}

void qt_gui_c_QColor_name_to_output_no_args(const QColor* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

QColor* qt_gui_c_QColor_operator_assign_arg1(QColor* this_ptr, const QColor* arg1) {
  return &this_ptr->operator=(*arg1);
}

QColor* qt_gui_c_QColor_operator_assign_color(QColor* this_ptr, const Qt::GlobalColor* color) {
  return &this_ptr->operator=(*color);
}

bool qt_gui_c_QColor_operator_eq(const QColor* this_ptr, const QColor* c) {
  return this_ptr->operator==(*c);
}

bool qt_gui_c_QColor_operator_neq(const QColor* this_ptr, const QColor* c) {
  return this_ptr->operator!=(*c);
}

int qt_gui_c_QColor_red(const QColor* this_ptr) {
  return this_ptr->red();
}

double qt_gui_c_QColor_redF(const QColor* this_ptr) {
  return this_ptr->redF();
}

unsigned int qt_gui_c_QColor_rgb(const QColor* this_ptr) {
  return this_ptr->rgb();
}

unsigned int qt_gui_c_QColor_rgba(const QColor* this_ptr) {
  return this_ptr->rgba();
}

void qt_gui_c_QColor_rgba64_to_output(const QColor* this_ptr, QRgba64* output) {
  new(output) QRgba64(this_ptr->rgba64());
}

int qt_gui_c_QColor_saturation(const QColor* this_ptr) {
  return this_ptr->saturation();
}

double qt_gui_c_QColor_saturationF(const QColor* this_ptr) {
  return this_ptr->saturationF();
}

void qt_gui_c_QColor_setAlpha(QColor* this_ptr, int alpha) {
  this_ptr->setAlpha(alpha);
}

void qt_gui_c_QColor_setAlphaF(QColor* this_ptr, double alpha) {
  this_ptr->setAlphaF(alpha);
}

void qt_gui_c_QColor_setBlue(QColor* this_ptr, int blue) {
  this_ptr->setBlue(blue);
}

void qt_gui_c_QColor_setBlueF(QColor* this_ptr, double blue) {
  this_ptr->setBlueF(blue);
}

void qt_gui_c_QColor_setCmykF_c_m_y_k(QColor* this_ptr, double c, double m, double y, double k) {
  this_ptr->setCmykF(c, m, y, k);
}

void qt_gui_c_QColor_setCmykF_c_m_y_k_a(QColor* this_ptr, double c, double m, double y, double k, double a) {
  this_ptr->setCmykF(c, m, y, k, a);
}

void qt_gui_c_QColor_setCmyk_c_m_y_k(QColor* this_ptr, int c, int m, int y, int k) {
  this_ptr->setCmyk(c, m, y, k);
}

void qt_gui_c_QColor_setCmyk_c_m_y_k_a(QColor* this_ptr, int c, int m, int y, int k, int a) {
  this_ptr->setCmyk(c, m, y, k, a);
}

void qt_gui_c_QColor_setGreen(QColor* this_ptr, int green) {
  this_ptr->setGreen(green);
}

void qt_gui_c_QColor_setGreenF(QColor* this_ptr, double green) {
  this_ptr->setGreenF(green);
}

void qt_gui_c_QColor_setHslF_h_s_l(QColor* this_ptr, double h, double s, double l) {
  this_ptr->setHslF(h, s, l);
}

void qt_gui_c_QColor_setHslF_h_s_l_a(QColor* this_ptr, double h, double s, double l, double a) {
  this_ptr->setHslF(h, s, l, a);
}

void qt_gui_c_QColor_setHsl_h_s_l(QColor* this_ptr, int h, int s, int l) {
  this_ptr->setHsl(h, s, l);
}

void qt_gui_c_QColor_setHsl_h_s_l_a(QColor* this_ptr, int h, int s, int l, int a) {
  this_ptr->setHsl(h, s, l, a);
}

void qt_gui_c_QColor_setHsvF_h_s_v(QColor* this_ptr, double h, double s, double v) {
  this_ptr->setHsvF(h, s, v);
}

void qt_gui_c_QColor_setHsvF_h_s_v_a(QColor* this_ptr, double h, double s, double v, double a) {
  this_ptr->setHsvF(h, s, v, a);
}

void qt_gui_c_QColor_setHsv_h_s_v(QColor* this_ptr, int h, int s, int v) {
  this_ptr->setHsv(h, s, v);
}

void qt_gui_c_QColor_setHsv_h_s_v_a(QColor* this_ptr, int h, int s, int v, int a) {
  this_ptr->setHsv(h, s, v, a);
}

void qt_gui_c_QColor_setNamedColor_QLatin1String(QColor* this_ptr, const QLatin1String* name) {
  this_ptr->setNamedColor(*name);
}

void qt_gui_c_QColor_setNamedColor_QString(QColor* this_ptr, const QString* name) {
  this_ptr->setNamedColor(*name);
}

void qt_gui_c_QColor_setRed(QColor* this_ptr, int red) {
  this_ptr->setRed(red);
}

void qt_gui_c_QColor_setRedF(QColor* this_ptr, double red) {
  this_ptr->setRedF(red);
}

void qt_gui_c_QColor_setRgbF_r_g_b(QColor* this_ptr, double r, double g, double b) {
  this_ptr->setRgbF(r, g, b);
}

void qt_gui_c_QColor_setRgbF_r_g_b_a(QColor* this_ptr, double r, double g, double b, double a) {
  this_ptr->setRgbF(r, g, b, a);
}

void qt_gui_c_QColor_setRgb_r_g_b(QColor* this_ptr, int r, int g, int b) {
  this_ptr->setRgb(r, g, b);
}

void qt_gui_c_QColor_setRgb_r_g_b_a(QColor* this_ptr, int r, int g, int b, int a) {
  this_ptr->setRgb(r, g, b, a);
}

void qt_gui_c_QColor_setRgb_rgb(QColor* this_ptr, unsigned int rgb) {
  this_ptr->setRgb(rgb);
}

void qt_gui_c_QColor_setRgba(QColor* this_ptr, unsigned int rgba) {
  this_ptr->setRgba(rgba);
}

void qt_gui_c_QColor_setRgba64(QColor* this_ptr, const QRgba64* rgba) {
  this_ptr->setRgba64(*rgba);
}

QColor::Spec qt_gui_c_QColor_spec(const QColor* this_ptr) {
  return this_ptr->spec();
}

void qt_gui_c_QColor_toCmyk_to_output(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->toCmyk());
}

void qt_gui_c_QColor_toHsl_to_output(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->toHsl());
}

void qt_gui_c_QColor_toHsv_to_output(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->toHsv());
}

void qt_gui_c_QColor_toRgb_to_output(const QColor* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->toRgb());
}

int qt_gui_c_QColor_value(const QColor* this_ptr) {
  return this_ptr->value();
}

double qt_gui_c_QColor_valueF(const QColor* this_ptr) {
  return this_ptr->valueF();
}

int qt_gui_c_QColor_yellow(const QColor* this_ptr) {
  return this_ptr->yellow();
}

double qt_gui_c_QColor_yellowF(const QColor* this_ptr) {
  return this_ptr->yellowF();
}

