use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

pub enum ArcOrRef<'a, T: ?Sized> {
    Ref(&'a Arc<T>),
    Arc(Arc<T>),
}

impl<T: ?Sized> Clone for ArcOrRef<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Arc(arc) => ArcOrRef::Arc(Arc::clone(arc)),
            Self::Ref(refer) => ArcOrRef::Ref(refer),
        }
    }
}

impl<T: ?Sized + std::hash::Hash> std::hash::Hash for ArcOrRef<'_, T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        <T as std::hash::Hash>::hash(self, state)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for ArcOrRef<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Debug>::fmt(self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for ArcOrRef<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Display>::fmt(self, f)
    }
}

impl<T1: ?Sized, T2: ?Sized> PartialEq<ArcOrRef<'_, T2>> for ArcOrRef<'_, T1>
where
    T1: PartialEq<T2>,
{
    #[inline]
    fn eq(&self, other: &ArcOrRef<'_, T2>) -> bool {
        <T1 as PartialEq<T2>>::eq(self, other)
    }
}

impl<T: ?Sized + Eq> Eq for ArcOrRef<'_, T> {}

impl<T1: ?Sized, T2: ?Sized> PartialOrd<ArcOrRef<'_, T2>> for ArcOrRef<'_, T1>
where
    T1: PartialOrd<T2>,
{
    #[inline]
    fn partial_cmp(&self, other: &ArcOrRef<'_, T2>) -> Option<std::cmp::Ordering> {
        <T1 as PartialOrd<T2>>::partial_cmp(self, other)
    }
}

impl<T: ?Sized + Ord> Ord for ArcOrRef<'_, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        <T as Ord>::cmp(self, other)
    }
}

impl<T: ?Sized> std::borrow::Borrow<T> for ArcOrRef<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> From<Arc<T>> for ArcOrRef<'static, T> {
    #[inline]
    fn from(value: Arc<T>) -> Self {
        Self::Arc(value)
    }
}

impl<T: ?Sized> From<Box<T>> for ArcOrRef<'static, T> {
    #[inline]
    fn from(value: Box<T>) -> Self {
        Self::Arc(Arc::from(value))
    }
}

impl<T> From<T> for ArcOrRef<'static, T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Arc(Arc::new(value))
    }
}
impl<'a, T: ?Sized> From<&'a Arc<T>> for ArcOrRef<'a, T> {
    #[inline]
    fn from(value: &'a Arc<T>) -> Self {
        Self::Ref(value)
    }
}

impl<'a, T: ?Sized> ArcOrRef<'a, T> {
    #[inline]
    pub fn into_arc(self) -> Arc<T> {
        match self {
            Self::Arc(arc) => arc,
            Self::Ref(refer) => Arc::clone(refer),
        }
    }

    #[inline]
    pub fn as_arc_ref(&self) -> &Arc<T> {
        match self {
            Self::Arc(arc) => arc,
            Self::Ref(refer) => *refer,
        }
    }

    #[inline]
    pub fn into_owned(self) -> ArcOrRef<'static, T> {
        ArcOrRef::Arc(self.into_arc())
    }
}

impl<T: ?Sized> Deref for ArcOrRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Arc(arc) => arc,
            Self::Ref(refer) => refer,
        }
    }
}

impl<T: ?Sized> AsRef<T> for ArcOrRef<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::ArcOrRef;

    impl<T: ?Sized + serde::Serialize> serde::Serialize for ArcOrRef<'_, T> {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            <T as serde::Serialize>::serialize(self, serializer)
        }
    }

    impl<'a, 'de, T: ?Sized> serde::Deserialize<'de> for ArcOrRef<'a, T>
    where
        std::sync::Arc<T>: serde::Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            std::sync::Arc::deserialize(deserializer).map(Self::Arc)
        }
    }
}
