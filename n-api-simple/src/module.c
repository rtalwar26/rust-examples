#define NAPI_VERSION 2
#include <node_api.h>
#include <td/telegram/td_json_client.h>


napi_value MyFunction(napi_env env, napi_callback_info info)
{
    napi_status status;

    size_t argc = 1;
    napi_value argv[1];
    status = napi_get_cb_info(env, info, &argc, argv, NULL, NULL);    
    if (status != napi_ok)
    {
        napi_throw_error(env, NULL, "Unable to read callback_info");
        
    }

    int number = 0;
    status = napi_get_value_int32(env, argv[0], &number);
    if (status != napi_ok) {
        napi_throw_error(env, NULL, "Invalid number was passed as argument");
    }
    napi_value myJSNumber;
    number = number * 2;
    status = napi_create_int32(env,number,&myJSNumber);
    if (status != napi_ok) {
        napi_throw_error(env, NULL, "Unable to create JsNumber");
    }
    return myJSNumber;
}

napi_value Init(napi_env env, napi_value exports)
{
    napi_status status;
    napi_value fn;

    status = napi_create_function(env, NULL, 0, MyFunction, NULL, &fn);

    if (status != napi_ok)
    {
        napi_throw_error(env, NULL, "Unable to wrap native function");
    }
    status = napi_set_named_property(env, exports, "my_function", fn);
    if (status != napi_ok)
    {
        napi_throw_error(env, NULL, "Unable to populate exports");
    }

    return exports;
}
NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)