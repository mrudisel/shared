use std::fmt;


pub enum StaticOrBoxed<T: ?Sized + 'static> {
    Static(&'static T),
    Boxed(Box<T>)
}

impl<T: ?Sized + Clone> Clone for StaticOrBoxed<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Boxed(boxed) => Self::Boxed(boxed.clone()),
            Self::Static(stat) => Self::Static(stat),
        }
    }
}


impl<T: ?Sized> StaticOrBoxed<T> {
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    pub fn is_boxed(&self) -> bool {
        matches!(self, Self::Boxed(_))
    }


    pub fn as_static(&self) -> Option<&'static T> {
        match self {
            Self::Static(stat) => Some(stat),
            _ => None,
        }
    }
    
    pub fn as_boxed(&self) -> Option<&Box<T>> {
        match self {
            Self::Boxed(b) => Some(b),
            _ => None,
        }
    }
}

impl<T: ?Sized> Default for StaticOrBoxed<T>
where   
    Box<T>: Default,
{
    fn default() -> Self {
        Self::Boxed(Box::default())
    }
}

impl<T: ?Sized, U> PartialEq<U> for StaticOrBoxed<T>
where 
    T: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.as_ref().eq(other)
    }
}

impl<T: ?Sized, U> PartialOrd<U> for StaticOrBoxed<T>
where 
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}





impl<T: ?Sized> AsRef<T> for StaticOrBoxed<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        match self {
            Self::Boxed(shared) => shared,
            Self::Static(stat) => stat,
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for StaticOrBoxed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for StaticOrBoxed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: ?Sized> std::ops::Deref for StaticOrBoxed<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> std::borrow::Borrow<T> for StaticOrBoxed<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}


impl<T: ?Sized> From<&'static T> for StaticOrBoxed<T> {
    fn from(value: &'static T) -> Self {
        Self::Static(value)
    }
}

impl<T: ?Sized> From<Box<T>> for StaticOrBoxed<T> {
    fn from(value: Box<T>) -> Self {
        Self::Boxed(value)
    }
}

impl From<String> for StaticOrBoxed<str> {
    fn from(value: String) -> Self {
        Self::Boxed(Box::from(value))        
    }
}


impl<T: ?Sized + std::hash::Hash> std::hash::Hash for StaticOrBoxed<T> {
    fn hash<H>(&self, state: &mut H) 
    where   
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

#[cfg(feature = "serde")]
impl<T: ?Sized + serde::Serialize> serde::Serialize for StaticOrBoxed<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        self.as_ref().serialize(serializer)    
    }
}

#[cfg(feature = "serde")]
impl<'de, T: ?Sized> serde::Deserialize<'de> for StaticOrBoxed<T> 
where 
    Box<T>: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        let boxed = Box::<T>::deserialize(deserializer)?;
        Ok(Self::Boxed(boxed))        
    }
}