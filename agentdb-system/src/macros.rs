/// Declare a new AgentDB root with the given name.
///
/// ```rust
/// declare_root!("my_root" => MY_ROOT);
/// ```
#[macro_export]
macro_rules! declare_root {
    ($name:literal => $v:ident) => {
        pub const $v: $crate::Root = $crate::Root::new($name);
        const _: () = {
            static STATIC_ROOT: $crate::Root = $v;
            $crate::hidden::inventory::submit! { STATIC_ROOT }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_agent {
    ($name:literal => $t:ty [
        $frangible:literal
    ]) => {
        #[$crate::hidden::typetag::serde(name = $name)]
        #[$crate::hidden::async_trait]
        impl $crate::Agent for $t {
            fn is_frangible() -> bool {
                $frangible
            }
            async fn _internal_destruct(
                self: Box<Self>,
                ref_: $crate::DynAgentRef,
                context: &mut $crate::Context,
            ) -> Result<(), $crate::Error> {
                $crate::hidden::destruct_agent(*self, ref_, context).await
            }
            async fn _internal_handle_dyn(
                &mut self,
                ref_: $crate::DynAgentRef,
                message: $crate::DynMessage,
                context: &mut $crate::Context,
            ) -> Result<bool, $crate::Error> {
                $crate::hidden::handle_dyn(self, ref_, message, context).await
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_message {
    ($name:literal => $t:ty) => {
        #[$crate::hidden::typetag::serde(name = $name)]
        #[$crate::hidden::async_trait]
        impl $crate::Message for $t {
            async fn _internal_deliver(
                self: Box<Self>,
                agent_ref: DynAgentRef,
                maybe_agent_state: &mut Option<DynAgent>,
                context: &mut Context,
            ) -> Result<(), $crate::Error> {
                $crate::hidden::deliver_message(*self, agent_ref, maybe_agent_state, context).await
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_constructor {
    ($t:ty) => {
        $crate::hidden::inventory::submit! {
            $crate::hidden::Constructor::<$t>::new()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_destructor {
    ($t:ty) => {
        $crate::hidden::inventory::submit! {
            $crate::hidden::Destructor::<$t>::new()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_handler {
    ($a:ty [ $($b:ty),* ]) => {
        $(
            $crate::hidden::inventory::submit! {
                $crate::hidden::Handler::<$b>::new::<$a>()
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_dyn_handler {
    ($a:ty) => {
        $crate::hidden::inventory::submit! {
            $crate::hidden::HandlerDyn::<$a>::new()
        }
    };
}
