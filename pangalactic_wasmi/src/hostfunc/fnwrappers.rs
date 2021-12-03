use super::get_name;
use crate::{FromGuestValue, HostFunc, IntoGuestReturn};
use wasmi::Trap;

macro_rules! def_hfwrapper {
    ( $wrapper:ident, ( $( $argtype:ident ),* ), ( $( $argname:ident ),* ) ) => {
        pub(crate) struct $wrapper<V, F, R, E, $( $argtype ),* >
        where
            F: Fn(&mut V, $( $argtype ),* ) -> Result<R, E>,
            R: IntoGuestReturn,
            Trap: From<E>,
        {
            f: F,
            phantom: std::marker::PhantomData<(V, R, E, $( $argtype ),* )>,
        }

        impl<V, F, R, E, $( $argtype ),* > From<F> for $wrapper<V, F, R, E, $( $argtype ),* >
        where
            F: Fn(&mut V, $( $argtype ),* ) -> Result<R, E>,
            R: IntoGuestReturn,
            Trap: From<E>,
            $(
                $argtype : FromGuestValue
            ),*
        {
            fn from(f: F) -> Self {
                $wrapper {
                    f,
                    phantom: std::marker::PhantomData,
                }
            }
        }

        impl<V, F, R, E, $( $argtype ),* > HostFunc<V> for $wrapper<V, F, R, E, $( $argtype ),* >
        where
            F: Fn(&mut V, $( $argtype ),* ) -> Result<R, E>,
            R: IntoGuestReturn,
            Trap: From<E>,
            $(
                $argtype : FromGuestValue
            ),*
        {
            type Args = ( $( $argtype , )* );
            type Return = R;

            fn name(&self) -> String {
                get_name::<F>()
            }

            fn invoke(&self, vm: &mut V, ( $( $argname , )* ): ( $( $argtype , )* )) -> Result<Self::Return, Trap> {
                self.f.call((vm, $( $argname ),* )).map_err(|e: E| Trap::from(e))
            }
        }
    }
}

def_hfwrapper!(HostFn0, (), ());
def_hfwrapper!(HostFn1, (A1), (a1));
def_hfwrapper!(HostFn2, (A1, A2), (a1, a2));
def_hfwrapper!(HostFn3, (A1, A2, A3), (a1, a2, a3));
def_hfwrapper!(HostFn4, (A1, A2, A3, A4), (a1, a2, a3, a4));
