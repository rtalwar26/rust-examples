#pragma once

#include <napi.h>

class NApiHello : public Napi::ObjectWrap<NApiHello>
{
public:
    NApiHello(const Napi::CallbackInfo&);
    Napi::Value Greet(const Napi::CallbackInfo&);

    static Napi::Function GetClass(Napi::Env);

private:
    std::string _greeterName;
};
