// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
use NavigationAction;
use NavigationType;
use PolicyDecision;
use URIRequest;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct NavigationPolicyDecision(Object<ffi::WebKitNavigationPolicyDecision, ffi::WebKitNavigationPolicyDecisionClass>): PolicyDecision;

    match fn {
        get_type => || ffi::webkit_navigation_policy_decision_get_type(),
    }
}

pub trait NavigationPolicyDecisionExt {
    fn get_frame_name(&self) -> Option<String>;

    fn get_modifiers(&self) -> u32;

    fn get_mouse_button(&self) -> u32;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn get_navigation_action(&self) -> Option<NavigationAction>;

    fn get_navigation_type(&self) -> NavigationType;

    fn get_request(&self) -> Option<URIRequest>;

    fn connect_property_frame_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modifiers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_navigation_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_navigation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NavigationPolicyDecision> + IsA<glib::object::Object>> NavigationPolicyDecisionExt for O {
    fn get_frame_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_frame_name(self.to_glib_none().0))
        }
    }

    fn get_modifiers(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_policy_decision_get_modifiers(self.to_glib_none().0)
        }
    }

    fn get_mouse_button(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_policy_decision_get_mouse_button(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn get_navigation_action(&self) -> Option<NavigationAction> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_navigation_action(self.to_glib_none().0))
        }
    }

    fn get_navigation_type(&self) -> NavigationType {
        unsafe {
            from_glib(ffi::webkit_navigation_policy_decision_get_navigation_type(self.to_glib_none().0))
        }
    }

    fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_request(self.to_glib_none().0))
        }
    }

    fn connect_property_frame_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::frame-name",
                transmute(notify_frame_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_modifiers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::modifiers",
                transmute(notify_modifiers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mouse-button",
                transmute(notify_mouse_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_navigation_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::navigation-action",
                transmute(notify_navigation_action_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_navigation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::navigation-type",
                transmute(notify_navigation_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::request",
                transmute(notify_request_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_frame_name_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_modifiers_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mouse_button_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_6", feature = "dox"))]
unsafe extern "C" fn notify_navigation_action_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_navigation_type_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_request_trampoline<P>(this: *mut ffi::WebKitNavigationPolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NavigationPolicyDecision> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NavigationPolicyDecision::from_glib_borrow(this).downcast_unchecked())
}
