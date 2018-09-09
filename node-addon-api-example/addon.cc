#include <napi.h>

extern int32_t double_input(int32_t input);

Napi::Value Add(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();

  if (info.Length() < 2) {
    Napi::TypeError::New(env, "Wrong number of arguments").ThrowAsJavaScriptException();
    return env.Null();
  }

  if (!info[0].IsNumber() || !info[1].IsNumber()) {
    Napi::TypeError::New(env, "Wrong arguments").ThrowAsJavaScriptException();
    return env.Null();
  }

  int32_t arg0 = info[0].As<Napi::Number>().Uint32Value();
  int32_t arg1 = info[1].As<Napi::Number>().Uint32Value();
  Napi::Number num = Napi::Number::New(env, double_input(arg0 + arg1));

  return num;
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "add"),
              Napi::Function::New(env, Add));
  return exports;
}

NODE_API_MODULE(addon, Init)