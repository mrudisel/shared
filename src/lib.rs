use std::{fmt, sync::Arc};




#[derive(Clone)]
pub enum Shared<T: ?Sized + 'static> {
    Static(&'static T),
    Shared(Arc<T>)
}

impl<T: ?Sized> Shared<T> {
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    pub fn is_stared(&self) -> bool {
        matches!(self, Self::Shared(_))
    }


    pub fn as_static(&self) -> Option<&'static T> {
        match self {
            Self::Static(stat) => Some(stat),
            _ => None,
        }
    }
    
    pub fn as_shared(&self) -> Option<&Arc<T>> {
        match self {
            Self::Shared(shared) => Some(shared),
            _ => None,
        }
    }

    pub fn to_shared(&mut self) -> &Arc<T>
    where 
        Box<T>: From<&'static T>
    {
        match self {
            Self::Shared(shared) => shared,
            Self::Static(stat) => {
                let shared = Arc::from(Box::from(stat));    
                
                *self = Self::Shared(shared);

                match self {
                    Self::Shared(shared) => shared,
                    _ => unreachable!(),
                }
            }
        }
    } 
}

impl<T: ?Sized> Default for Shared<T>
where   
    Arc<T>: Default,
{
    fn default() -> Self {
        Self::Shared(Arc::default())
    }
}

impl<T: ?Sized, U> PartialEq<U> for Shared<T>
where 
    T: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.as_ref().eq(other)
    }
}

impl<T: ?Sized, U> PartialOrd<U> for Shared<T>
where 
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}





impl<T: ?Sized> AsRef<T> for Shared<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        match self {
            Self::Shared(shared) => shared,
            Self::Static(stat) => stat,
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: ?Sized> std::ops::Deref for Shared<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> std::borrow::Borrow<T> for Shared<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}


impl<T: ?Sized> From<&'static T> for Shared<T> {
    fn from(value: &'static T) -> Self {
        Self::Static(value)
    }
}

impl<T: ?Sized> From<Arc<T>> for Shared<T> {
    fn from(value: Arc<T>) -> Self {
        Self::Shared(value)
    }
}

impl From<String> for Shared<str> {
    fn from(value: String) -> Self {
        Self::Shared(Arc::from(value))        
    }
}


impl<T: ?Sized + std::hash::Hash> std::hash::Hash for Shared<T> {
    fn hash<H>(&self, state: &mut H) 
    where   
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

#[cfg(feature = "serde")]
impl<T: ?Sized + serde::Serialize> serde::Serialize for Shared<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        self.as_ref().serialize(serializer)    
    }
}

#[cfg(feature = "serde")]
impl<'de, T: ?Sized> serde::Deserialize<'de> for Shared<T> 
where 
    Box<T>: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        let boxed = Box::<T>::deserialize(deserializer)?;
        Ok(Self::Shared(Arc::from(boxed)))        
    }
}