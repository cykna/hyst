use std::{
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, mpsc::Sender},
};

use wgpu::naga::FastHashSet;

use super::HystElementKey;

#[derive(Clone)]
///A pulse is a thread shared value which is used to get track of changing states and tell ui to compute them.
pub struct Pulse<T> {
    pulse: Arc<RwLock<T>>,
    ///The id of dependent elements. When this Pulse modifies, every element with the listed keys will be updated before drawing the next frame
    dep_ids: Arc<RwLock<FastHashSet<HystElementKey>>>,
    sender: Sender<HystElementKey>,
}

impl<T> Pulse<T> {
    ///Creates a new pulse with the given `initial` value and Sender `tx` used to communicate when it changed
    pub fn new(initial: T, tx: Sender<HystElementKey>) -> Self {
        Self {
            pulse: Arc::new(RwLock::new(initial)),
            dep_ids: Arc::new(RwLock::new(FastHashSet::default())),
            sender: tx,
        }
    }

    ///Executes the given method and tells the ui that an update is required.
    pub fn mutate<F>(&self, f: F)
    where
        F: Fn(RwLockWriteGuard<T>),
    {
        let guard = self.pulse.write().unwrap();
        f(guard);
        self.tell_receiver();
    }

    ///Adds the given dependency on this pulse. When modifying, the element whose is owner of the given key will be requested to update.
    pub fn add_dependency(&mut self, dep: HystElementKey) {
        self.dep_ids.write().unwrap().insert(dep);
    }

    ///Tells the receiver to update every dependency this pulse has got. The dependenc
    pub fn tell_receiver(&self) {
        let guard = self.dep_ids.read().unwrap();
        for key in guard.iter() {
            self.sender.send(*key).unwrap();
        }
    }
    #[inline]
    ///Retrieves the guard for the underlying data of this pulse
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.pulse.read().unwrap()
    }
    #[inline]
    ///Retrives the writeable guard for the underlying data of this pulse. When modifying something with this, it's not automatically sent that this Pulse has changed
    ///so it might be used carefully
    pub unsafe fn write(&mut self) -> RwLockWriteGuard<T> {
        self.pulse.write().unwrap()
    }
}

impl<T> Pulse<T>
where
    T: Clone,
{
    ///Clones the underlying data and returns it.
    #[inline]
    pub fn cloned(&self) -> T {
        self.pulse.read().unwrap().clone()
    }
}

impl<T> AddAssign<T> for Pulse<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: T) {
        unsafe {
            *self.write() += rhs;
        }
        self.tell_receiver();
    }
}
impl<T> SubAssign<T> for Pulse<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        unsafe {
            *self.write() -= rhs;
        }
        self.tell_receiver();
    }
}

impl<T> MulAssign<T> for Pulse<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        unsafe {
            *self.write() *= rhs;
        }
        self.tell_receiver();
    }
}
impl<T> DivAssign<T> for Pulse<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        unsafe {
            *self.write() /= rhs;
        }
        self.tell_receiver();
    }
}
