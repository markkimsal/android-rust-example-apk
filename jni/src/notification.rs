use ::jni::objects::{JObject, JValue, JValueGen};
use ::log::debug;

pub fn display_toast()
{
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.expect("cannot get vm from ctx");
    let mut env = vm.attach_current_thread().expect("cannot attach to current thread");

    let toast_clazz = env.find_class("android/widget/Toast").expect("cannot load context class");

    let message = env.new_string("EHLO World").unwrap();
    let ctx_as_jobect = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    let ctx_as_param = JValueGen::Object(&ctx_as_jobect);
    let args = &[ctx_as_param, JValue::Object(&message), JValue::Int(0)];
    let toastobj = env.call_static_method(toast_clazz, "makeText", "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;", args);
    if let Err(e) = toastobj.as_ref() {
        debug!("err: {}", e);
        let _ = env.exception_describe().unwrap();
        let _ = env.fatal_error("exception calling Toast::makeText");
    }
    let mut toastobj = toastobj.unwrap().l().unwrap();

    let _ = env.call_method(&mut toastobj, "show", "()V", &[]);
}
