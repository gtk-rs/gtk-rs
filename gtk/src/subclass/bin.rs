use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use Bin;
use Container;

pub trait BinImpl: ContainerImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
